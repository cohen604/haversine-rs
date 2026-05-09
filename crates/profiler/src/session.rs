use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

pub static GLOBAL_SESSIONS: LazyLock<Mutex<HashMap<String, Session>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn insert_session(name: String, session: Session) {
    let mut sessions = GLOBAL_SESSIONS
        .lock()
        .expect("Global sessions mutex poisoned");
    sessions.insert(name, session);
}

#[derive(Clone)]
pub struct Session {
    pub name: String,
    pub started_at: u64,
    pub ended_at: Option<u64>,
}

pub fn print_sessions() {
    let sessions = GLOBAL_SESSIONS
        .lock()
        .expect("Global sessions mutex poisoned");

    for (name, session) in sessions.iter() {
        let elapsed = session
            .ended_at
            .map(|ended_at| ended_at.saturating_sub(session.started_at));

        eprintln!(
            "{name}: started_at={}, ended_at={:?}, elapsed_cycles={:?}",
            session.started_at, session.ended_at, elapsed,
        );
    }
}

pub struct PrintSessionsOnDrop;

impl Drop for PrintSessionsOnDrop {
    fn drop(&mut self) {
        print_sessions();
    }
}
