pub mod tokens_manager;

use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use bollard::{
    Docker, query_parameters::CreateContainerOptionsBuilder, secret::ContainerCreateBody,
};
use color_eyre::eyre::{Context, Result};
use socketioxide::{SocketIo, SocketIoBuilder, layer::SocketIoLayer};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;

use crate::tokens_manager::{JobRun, TokensManager};

#[allow(dead_code)]
pub struct WorkflowRunner {
    docker: Arc<Docker>,
    io: Arc<Option<SocketIo>>,
    tokens: Arc<RwLock<TokensManager>>,
}

impl WorkflowRunner {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_defaults().wrap_err("Failed to connect to Docker")?;

        let tokens = TokensManager::new();

        Ok(WorkflowRunner {
            docker: Arc::new(docker),
            io: Arc::new(None),
            tokens: Arc::new(RwLock::new(tokens)),
        })
    }

    pub async fn serve(&mut self) -> Result<()> {
        let (layer, io) = SocketIoBuilder::new().build_layer();

        self.io = Arc::new(Some(io));

        self.init_axum(layer)
            .await
            .expect("Failed to initialize axum server");

        Ok(())
    }

    async fn init_axum(&self, io_layer: SocketIoLayer) -> Result<()> {
        let app = Router::new()
            .route(
                "/",
                get(|| async { format!("Agin CI LibRunner {}", env!("CARGO_PKG_VERSION")) }),
            )
            .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() });

        let app = app.merge(Router::new().layer(io_layer));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:37581")
            .await
            .wrap_err("Failed to bind")?;

        tokio::spawn(async move {
            if let Err(err) = axum::serve(listener, app).await {
                error!("Server crashed: {:?}", err);
            }
        });

        Ok(())
    }

    pub async fn run_workflow(&self, run: JobRun) -> Result<()> {
        let token = {
            let mut tokens_write = self.tokens.write().await;
            tokens_write.generate_run_token(run.clone())
        };

        let container_config = ContainerCreateBody {
            image: run.job.base_image,
            cmd: Some(vec!["/bin/aginci-container-runner".to_string()]),
            env: Some(vec![
                format!("AGINCI_LIBRUNNER_TOKEN={token}"),
                "AGINCI_LIBRUNNER_URL=ws://172.17.0.1:37581".to_string(),
            ]),
            ..Default::default()
        };

        let container_name = format!("aginci_{}", run.id);
        let create_options = CreateContainerOptionsBuilder::new()
            .name(&container_name)
            .build();

        self.docker
            .create_container(Some(create_options), container_config)
            .await?;

        Ok(())
    }
}

// Default cannot return Result, so we must panic if it fails
impl Default for WorkflowRunner {
    fn default() -> Self {
        Self::new().expect("Failed to create WorkflowRunner with default config")
    }
}
