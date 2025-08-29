#[cfg(feature = "pulsar")]
use pulsar::{DeserializeMessage, Payload, SerializeMessage, producer};
use serde::{Deserialize, Serialize};

use crate::workflow::{CancelJob, JobRun};

/// Message sent by the server to the worker (targeted by its worker ID)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "t", content = "d")]
pub enum ToWorkerMessage {
    JobRun(JobRun),
    CancelJob(CancelJob),
}

#[cfg(feature = "pulsar")]
impl DeserializeMessage for ToWorkerMessage {
    type Output = Result<ToWorkerMessage, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

#[cfg(feature = "pulsar")]
impl SerializeMessage for ToWorkerMessage {
    fn serialize_message(input: Self) -> Result<producer::Message, pulsar::Error> {
        let payload =
            serde_json::to_vec(&input).map_err(|e| pulsar::Error::Custom(e.to_string()))?;
        Ok(producer::Message {
            payload,
            ..Default::default()
        })
    }
}
