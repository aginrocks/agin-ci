mod axum_error;
mod database;
mod middlewares;
mod mongo_id;
mod notifications;
mod pulsar_client;
mod routes;
mod runner;
mod settings;
mod state;
mod utils;
mod validators;

use std::{net::SocketAddr, ops::Deref, sync::Arc};

use axum::{
    Router, error_handling::HandleErrorLayer, http::StatusCode, middleware, response::IntoResponse,
    routing::any,
};
use axum_oidc::{
    OidcAuthLayer, OidcClient, OidcLoginLayer, error::MiddlewareError, handle_oidc_redirect,
};
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool};
use tracing::{Instrument, error, info, info_span, instrument, level_filters::LevelFilter};
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
    middlewares::require_auth::require_auth,
    notifications::sender::NotificationSender,
    pulsar_client::init_pulsar,
    routes::RouteProtectionLevel,
    settings::Settings,
    state::AppState,
};

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct GroupClaims {}
impl axum_oidc::AdditionalClaims for GroupClaims {}
impl openidconnect::AdditionalClaims for GroupClaims {}

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

    let (pulsar, pulsar_admin) = init_pulsar(&settings).await?;

    let notifications = NotificationSender {
        database: database.clone(),
    };

    let app_state = AppState {
        database,
        settings: settings.clone(),
        pulsar,
        pulsar_admin,
        notifications,
    };

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
    session_layer: SessionManagerLayer<RedisStore<Pool>>,
) -> Result<Router> {
    let oidc_login_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            error!(error = ?e, "An error occurred in OIDC login middleware");
            e.into_response()
        }))
        .layer(OidcLoginLayer::<GroupClaims>::new());

    let app_url = format!(
        "{}/oidc",
        state
            .settings
            .general
            .public_url
            .to_string()
            .trim_end_matches('/')
    );

    let mut oidc_client = OidcClient::<GroupClaims>::builder()
        .with_default_http_client()
        .with_redirect_url(app_url.parse()?)
        .with_client_id(state.settings.oidc.client_id.as_str());

    for scope in state.settings.oidc.scopes.iter() {
        oidc_client = oidc_client.add_scope(scope.as_str());
    }

    if let Some(client_secret) = state.settings.oidc.client_secret.as_ref() {
        oidc_client = oidc_client.with_client_secret(client_secret.secret().clone());
    }

    let oidc_client = oidc_client
        .discover(state.settings.oidc.issuer.deref().clone())
        .instrument(info_span!("oidc_discover"))
        .await?
        .build();

    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            error!(error = ?e, "An error occurred in OIDC auth middleware");
            e.into_response()
        }))
        .layer(OidcAuthLayer::new(oidc_client));

    let routes = routes::routes();

    // Create separate routers for public and protected routes
    let public_router = OpenApiRouter::with_openapi(ApiDoc::openapi());
    let redirect_router = OpenApiRouter::with_openapi(ApiDoc::openapi());
    let auth_router = OpenApiRouter::with_openapi(ApiDoc::openapi());

    // Add public routes (these don't need authentication)
    let public_router = routes
        .clone()
        .into_iter()
        .filter(|(_, protected)| matches!(*protected, RouteProtectionLevel::Public))
        .fold(public_router, |router, (route, _)| router.routes(route));

    // Add protected routes with OIDC login layer
    let redirect_router = routes
        .clone()
        .into_iter()
        .filter(|(_, protected)| matches!(*protected, RouteProtectionLevel::Redirect))
        .fold(redirect_router, |router, (route, _)| router.routes(route))
        .layer(oidc_login_service.clone());

    // Add protected routes which don't redirect but require authentication
    let auth_router = routes
        .clone()
        .into_iter()
        .filter(|(_, protected)| matches!(*protected, RouteProtectionLevel::Authenticated))
        .fold(auth_router, |router, (route, _)| router.routes(route))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // Combine the routers
    let router = public_router.merge(redirect_router);

    let router = router.merge(auth_router);

    let router = router.layer(axum::extract::Extension(state.clone()));

    let oidc_handler_router: OpenApiRouter<AppState> =
        OpenApiRouter::with_openapi(ApiDoc::openapi())
            // .layer(session_layer.clone()) // Apply session layer first
            .layer(oidc_login_service)
            .route(
                "/oidc",
                any(|session, oidc_client, query| async move {
                    match handle_oidc_redirect::<GroupClaims>(session, oidc_client, query).await {
                        Ok(response) => response.into_response(),
                        Err(e) => {
                            error!(error = ?e, "OIDC redirect handler error: {e}");
                            (StatusCode::BAD_REQUEST, format!("OIDC error: {e}")).into_response()
                        }
                    }
                }),
            );

    let router = router.merge(oidc_handler_router);

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
        .layer(oidc_auth_service)
        .layer(session_layer)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() });

    Ok(router)
}

async fn init_listener(settings: &Settings) -> Result<TcpListener> {
    let addr: Vec<SocketAddr> = settings.general.listen_address.clone().into();

    Ok(TcpListener::bind(addr.as_slice()).await?)
}
