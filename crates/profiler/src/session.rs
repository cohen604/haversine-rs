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

#[derive(Clone, Debug)]
pub struct Session {
    pub name: String,
    pub started_at: u64,
    pub ended_at: Option<u64>,
}

pub fn print_sessions(start_cpu: u64, end_cpu: u64) {
    let total_cpu = end_cpu - start_cpu;
    let sessions = GLOBAL_SESSIONS
        .lock()
        .expect("Global sessions mutex poisoned");

    dbg!(total_cpu);

    for (name, session) in sessions.iter() {
        let elapsed = session
            .ended_at
            .map(|ended_at| ended_at.saturating_sub(session.started_at))
            .unwrap();

        let elapsed_percentage = (elapsed as f64 / total_cpu as f64) * 100.0;

        eprintln!(
            "{name}: started_at={}, ended_at={:?}, elapsed_cycles={:?}, elapsed_percent={:.2}%",
            session.started_at, session.ended_at, elapsed, elapsed_percentage
        );
    }
}
