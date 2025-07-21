use color_eyre::eyre::Result;
use enum_dispatch::enum_dispatch;
use std::future::Future;
use std::pin::Pin;
#[cfg(feature = "step_executor")]
use std::sync::Arc;
#[cfg(feature = "step_executor")]
use tokio::sync::broadcast::{self, Receiver};

use crate::runner_messages::report_progress::ProgressReport;

/// Inner trait implemented by real executors
#[cfg(feature = "step_executor")]
pub trait StepExecutorInner {
    fn execute(
        self: Arc<Self>,
        progress_tx: broadcast::Sender<ProgressReport>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>>;
}

/// Public trait that wraps and sets up the broadcast channel
#[cfg(feature = "step_executor")]
#[enum_dispatch(Step)]
pub trait StepExecutor {
    fn execute(&self) -> Receiver<ProgressReport>;
}

/// Blanket impl: wraps inner trait with channel logic
#[cfg(feature = "step_executor")]
impl<T: StepExecutorInner + Send + Sync + 'static> StepExecutor for Arc<T> {
    fn execute(&self) -> Receiver<ProgressReport> {
        let (progress_tx, progress_rx) = broadcast::channel::<ProgressReport>(16);

        // Clone Arc so the spawned task owns a reference counted pointer.
        let self_clone = Arc::clone(self);
        let fut = <T as StepExecutorInner>::execute(self_clone, progress_tx);

        tokio::spawn(fut);

        progress_rx
    }
}
