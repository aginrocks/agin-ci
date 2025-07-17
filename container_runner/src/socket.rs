use aginci_core::runner_messages::auth::Auth;
use color_eyre::eyre::{Context, Result};
use rust_socketio::{
    Payload,
    asynchronous::{Client, ClientBuilder},
};
use serde::de::DeserializeOwned;
use serde_json::from_value;
use std::{env, pin::Pin, sync::Arc};
use tokio::sync::OnceCell;

// For now, only one arg is supported. I HATE THAT I HAVE TO DO THIS SHIT IN ORDER TO JUST DESERIALIZE OBJECTS

pub fn deserialize_payload<T>(payload: &Payload) -> Result<T, String>
where
    T: DeserializeOwned,
{
    match payload {
        Payload::Text(vec_values) => from_value::<T>(vec_values[0][0].clone())
            .map_err(|e| format!("Failed to deserialize JSON array: {e}")),

        #[allow(clippy::uninlined_format_args)]
        other => Err(format!(
            "Unsupported Payload variant: expected Text, got {:?}",
            other
        )),
    }
}

#[macro_export]
macro_rules! handler {
    ($job_ty:ty, $handler:expr) => {
        |payload, _| {
            Box::pin(async move {
                match deserialize_payload::<$job_ty>(&payload) {
                    Ok(job) => {
                        $handler(job).await;
                    }
                    Err(err) => {
                        tracing::error!("Failed to deserialize payload: {:?}", err);
                    }
                }
            })
        }
    };
}

static SOCKET: OnceCell<Arc<Client>> = OnceCell::const_new();

pub async fn init_socket() -> Result<&'static Arc<Client>> {
    SOCKET
        .get_or_try_init(|| async {
            let server_url =
                env::var("AGINCI_LIBRUNNER_URL").wrap_err("Missing AGINCI_LIBRUNNER_URL")?;

            let token =
                env::var("AGINCI_LIBRUNNER_TOKEN").wrap_err("Missing AGINCI_LIBRUNNER_TOKEN")?;

            let client = ClientBuilder::new(server_url)
                .auth(serde_json::to_value(Auth { token })?)
                .connect()
                .await
                .wrap_err("Failed to connect to LibRunner")?;

            Ok(Arc::new(client))
        })
        .await
}
