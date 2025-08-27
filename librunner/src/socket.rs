mod get_job;
mod report_progress;

use std::{ops::Deref, sync::Arc};

use aginci_core::{
    runner_messages::{auth::Auth, report_progress::ProgressReport},
    workflow::JobRun,
};
use color_eyre::eyre::{Result, bail};
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State},
    handler::ConnectHandler,
};
use tokio::sync::broadcast;
use tracing::debug;

use crate::AppState;

pub async fn init_io(io: &SocketIo) -> Result<()> {
    io.ns("/", on_connection.with(authenticate_middleware));

    Ok(())
}

pub async fn on_connection(s: SocketRef) {
    debug!("new connection");

    s.on("get_job", get_job::handler);
    s.on("report_progress", report_progress::handler);
}

#[derive(Clone)]
pub struct UserData {
    pub job: JobRun,
    pub progress_tx: Arc<broadcast::Sender<ProgressReport>>,
}

impl Deref for UserData {
    type Target = JobRun;

    fn deref(&self) -> &Self::Target {
        &self.job
    }
}

pub async fn authenticate_middleware(
    s: SocketRef,
    Data(auth): Data<Auth>,
    State(state): State<AppState>,
) -> Result<()> {
    let token_info = {
        let token_read = state.tokens.read().await;
        let token = token_read.tokens.get(&auth.token);

        match token {
            Some(job_run) => job_run.clone(),
            None => bail!("Unauthorized"),
        }
    };

    s.extensions.insert(UserData {
        job: token_info,
        progress_tx: state.progress_tx,
    });

    Ok(())
}
