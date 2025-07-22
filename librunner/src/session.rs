use std::collections::HashMap;

use aginci_core::runner_messages::report_progress::ProgressReport;
use socketioxide::socket::Sid;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct EventsState {
    pub next_expected: u64,
    pub events_buffer: HashMap<u64, ProgressReport>,
}

#[derive(Clone, Default)]
pub struct Session {
    pub events: EventsState,
}

#[derive(Default)]
pub struct SessionManager {
    sessions: RwLock<HashMap<Sid, Session>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    // Get immutable access if needed
    pub async fn get(&self, sid: &Sid) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(sid).cloned()
    }

    // Get a write lock to the entire sessions map so you can mutate the session in place
    pub async fn get_map_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, HashMap<Sid, Session>> {
        self.sessions.write().await
    }

    pub async fn remove(&self, sid: &Sid) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(sid);
    }
}
