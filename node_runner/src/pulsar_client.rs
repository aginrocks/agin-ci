use api_client::models::FinishRegistrationResponse;
use color_eyre::eyre::Result;
use pulsar::{Authentication, Pulsar, TokioExecutor};
use std::sync::Arc;

pub async fn init_pulsar(
    registration: FinishRegistrationResponse,
) -> Result<Arc<Pulsar<TokioExecutor>>> {
    let authentication = Authentication {
        name: "token".to_string(),
        data: registration.access_token.into_bytes(),
    };

    let builder =
        Pulsar::builder(registration.connection_string, TokioExecutor).with_auth(authentication);

    let pulsar = builder.build().await?;

    Ok(Arc::new(pulsar))
}
