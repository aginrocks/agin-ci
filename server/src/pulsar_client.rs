use std::sync::Arc;

use color_eyre::eyre::Result;
use pulsar::{Authentication, Pulsar, TokioExecutor};

use crate::settings::Settings;

pub async fn init_pulsar(settings: &Settings) -> Result<Arc<Pulsar<TokioExecutor>>> {
    let mut builder = Pulsar::builder(&settings.pulsar.connection_string, TokioExecutor);

    let authentication = Authentication {
        name: "token".to_string(),
        data: settings.pulsar.token.clone().into_bytes(),
    };

    builder = builder.with_auth(authentication);
    let pulsar = builder.build().await?;

    Ok(Arc::new(pulsar))
}
