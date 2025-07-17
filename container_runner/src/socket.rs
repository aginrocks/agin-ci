use rust_socketio::Payload;
use serde::de::DeserializeOwned;
use serde_json::from_value;

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
