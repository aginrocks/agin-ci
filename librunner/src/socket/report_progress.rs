use aginci_core::runner_messages::report_progress::OrderedReport;
use socketioxide::extract::{Data, Extension, SocketRef, State};
use tracing::{debug, info};

use crate::{AppState, socket::UserData};

pub async fn handler(
    socket: SocketRef,
    Data(OrderedReport { ord, body }): Data<OrderedReport>,
    Extension(data): Extension<UserData>,
    State(state): State<AppState>,
) {
    debug!("Received progress report {ord}");

    let mut sessions = state.sessions.get_map_mut().await;

    let session = sessions.entry(socket.id).or_default();

    session.events.events_buffer.insert(ord, body.clone());

    while let Some(event) = session
        .events
        .events_buffer
        .remove(&session.events.next_expected)
    {
        debug!("Handling progress report {}", session.events.next_expected);
        let _ = data.progress_tx.send(event);
        session.events.next_expected += 1;
    }

    debug!(
        "Buffer after processing: {:?}",
        session.events.events_buffer.keys()
    );
}
