mod require_auth;
mod socket;
pub mod tokens_manager;

use axum::{
    Router, http::StatusCode, middleware::from_fn_with_state, response::IntoResponse, routing::get,
};
use bollard::{
    Docker, query_parameters::CreateContainerOptionsBuilder, secret::ContainerCreateBody,
};
use color_eyre::eyre::{Context, Result};
use socketioxide::{SocketIo, SocketIoBuilder, layer::SocketIoLayer};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::{
    require_auth::require_auth,
    socket::init_io,
    tokens_manager::{JobRun, TokensManager},
};

#[derive(Clone)]
pub struct AppState {
    pub docker: Arc<Docker>,
    pub tokens: Arc<RwLock<TokensManager>>,
}

pub struct WorkflowRunner {
    docker: Arc<Docker>,
    io: Arc<Option<SocketIo>>,
    state: AppState,
}

impl WorkflowRunner {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_defaults().wrap_err("Failed to connect to Docker")?;

        let tokens = TokensManager::new();

        let state = AppState {
            docker: Arc::new(docker.clone()),
            tokens: Arc::new(RwLock::new(tokens)),
        };

        Ok(WorkflowRunner {
            docker: Arc::new(docker),
            io: Arc::new(None),
            state,
        })
    }

    pub async fn serve(&mut self) -> Result<()> {
        let (layer, io) = SocketIoBuilder::new().build_layer();

        init_io(&io).await?;

        self.io = Arc::new(Some(io));

        self.init_axum(layer)
            .await
            .expect("Failed to initialize axum server");

        Ok(())
    }

    async fn init_axum(&self, io_layer: SocketIoLayer) -> Result<()> {
        let app_state = self.state.clone();

        let app = Router::new()
            .route("/", get(root_handler))
            .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() })
            .layer(io_layer)
            .layer(from_fn_with_state(app_state.clone(), require_auth))
            .with_state(app_state); // Provide shared state here

        let listener = tokio::net::TcpListener::bind("0.0.0.0:37581")
            .await
            .wrap_err("Failed to bind")?;

        tokio::spawn(async move {
            let app = app.into_make_service();

            if let Err(err) = axum::serve(listener, app).await {
                error!("Server crashed: {:?}", err);
            }
        });

        Ok(())
    }

    pub async fn run_workflow(&self, run: JobRun) -> Result<()> {
        // Access tokens inside the state for token generation
        let token = {
            let mut tokens_write = self.state.tokens.write().await;
            tokens_write.generate_run_token(run.clone())
        };

        info!("Token: {token}");

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

        // Use docker from the field directly
        self.docker
            .create_container(Some(create_options), container_config)
            .await?;

        Ok(())
    }
}

async fn root_handler() -> String {
    format!("Agin CI LibRunner {}", env!("CARGO_PKG_VERSION"))
}

// Default cannot return Result, so we must panic if it fails
impl Default for WorkflowRunner {
    fn default() -> Self {
        Self::new().expect("Failed to create WorkflowRunner with default config")
    }
}
