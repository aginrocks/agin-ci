mod axum_error;
mod database;
mod routes;
mod settings;
mod state;

use std::{net::SocketAddr, sync::Arc};

use axum::{Router, http::StatusCode, response::IntoResponse};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use tokio::net::TcpListener;
use tower_sessions::SessionManagerLayer;
use tower_sessions_mongodb_store::MongoDBStore;
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};

use crate::{
    database::{init_database, init_session_store},
    routes::RouteProtectionLevel,
    settings::Settings,
    state::{AppState, InnerState},
};

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenvy::dotenv().ok();
    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    let settings = Arc::new(Settings::try_load()?);

    let database = init_database(&settings).await?;

    let app_state = AppState::new(InnerState { database });

    let session_layer = init_session_store(&settings).await?;
    let app = init_axum(app_state, session_layer).await?;
    let listener = init_listener(&settings).await?;

    info!(
        "listening on {} ({})",
        listener
            .local_addr()
            .wrap_err("failed to get local address")?,
        settings.general.public_url
    );

    axum::serve(listener, app.into_make_service())
        .await
        .wrap_err("failed to run server")?;

    Ok(())
}

fn init_tracing() -> Result<()> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("RUST_LOG")
                .from_env()?,
        )
        .try_init()?;

    Ok(())
}

#[instrument(skip(state, session_layer))]
async fn init_axum(
    state: AppState,
    session_layer: SessionManagerLayer<MongoDBStore>,
) -> Result<Router> {
    let routes = routes::routes();

    let protected_router = {
        let protected_router = OpenApiRouter::with_openapi(ApiDoc::openapi());

        let protected_router = routes
            .clone()
            .into_iter()
            .filter(|(_, protected)| !matches!(*protected, RouteProtectionLevel::Public))
            .fold(protected_router, |protected_router, (route, _)| {
                protected_router.routes(route)
            });

        protected_router.layer(axum::extract::Extension(state.clone()))
    };

    let router = OpenApiRouter::with_openapi(ApiDoc::openapi()).merge(protected_router);

    let router = routes
        .clone()
        .into_iter()
        .filter(|(_, protected)| matches!(*protected, RouteProtectionLevel::Public))
        .fold(router, |router, (route, _)| router.routes(route));

    let router = router.layer(axum::extract::Extension(state.clone()));
    let (router, api) = router.with_state(state).split_for_parts();

    let openapi_prefix = "/apidoc";
    let spec_path = format!("{openapi_prefix}/openapi.json");

    let router = router
        .merge(Redoc::with_url(
            format!("{openapi_prefix}/redoc"),
            api.clone(),
        ))
        .merge(RapiDoc::new(spec_path.clone()).path(format!("{openapi_prefix}/rapidoc")))
        .merge(Scalar::with_url(
            format!("{openapi_prefix}/scalar"),
            api.clone(),
        ))
        .route(
            &spec_path,
            axum::routing::get(|| async move { axum::response::Json(api) }),
        );

    let router = router
        .layer(session_layer)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() });

    Ok(router)
}

async fn init_listener(settings: &Settings) -> Result<TcpListener> {
    let addr: Vec<SocketAddr> = settings.general.listen_address.clone().into();

    Ok(TcpListener::bind(addr.as_slice()).await?)
}
