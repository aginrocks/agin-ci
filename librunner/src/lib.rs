use std::sync::Arc;

use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use bollard::Docker;
use color_eyre::eyre::{Context, Result};
use socketioxide::{SocketIo, SocketIoBuilder, layer::SocketIoLayer};

pub struct WorkflowRunner {
    docker: Arc<Docker>,
    io: Arc<Option<SocketIo>>,
}

impl WorkflowRunner {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_defaults().wrap_err("Failed to connect to Docker")?;

        Ok(WorkflowRunner {
            docker: Arc::new(docker),
            io: Arc::new(None),
        })
    }

    pub async fn serve(&mut self) -> Result<()> {
        let (layer, io) = SocketIoBuilder::new().build_layer();

        self.io = Arc::new(Some(io));

        self.init_axum(layer)
            .await
            .wrap_err("Failed to initialize axum server")?;

        Ok(())
    }

    async fn init_axum(&self, io_layer: SocketIoLayer) -> Result<()> {
        let app = Router::new()
            .route(
                "/",
                get(|| async { format!("LibRunner {}", env!("CARGO_PKG_VERSION")) }),
            )
            .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() });

        let app = app.merge(Router::new().layer(io_layer));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:37581")
            .await
            .wrap_err("Failed to bind")?;

        if let Err(err) = axum::serve(listener, app).await {
            eprintln!("Server error: {err:?}");
        }

        Ok(())
    }
}

// Default cannot return Result, so we must panic if it fails
impl Default for WorkflowRunner {
    fn default() -> Self {
        Self::new().expect("Failed to create WorkflowRunner with default config")
    }
}
