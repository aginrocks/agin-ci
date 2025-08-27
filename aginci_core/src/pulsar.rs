#[cfg(feature = "pulsar")]
use pulsar::{DeserializeMessage, Payload};
use serde::{Deserialize, Serialize};

use crate::workflow::JobRun;

/// Message sent by the server to the worker (targeted by its worker ID)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "t", content = "d")]
pub enum ToWorkerMessage {
    JobRun(JobRun),
}

#[cfg(feature = "pulsar")]
impl DeserializeMessage for ToWorkerMessage {
    type Output = Result<ToWorkerMessage, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}
