use crate::{
    runtime,
    session::{self, Session},
};

pub struct SpanGuard {
    session: Session,
}

pub fn enter_scope(name: &str) -> SpanGuard {
    let session = Session {
        name: name.to_string(),
        started_at: runtime::read_cycles().unwrap(),
        ended_at: None,
    };

    SpanGuard { session }
}

impl Drop for SpanGuard {
    fn drop(&mut self) {
        self.session.ended_at = Some(runtime::read_cycles().unwrap());
        session::insert_session(self.session.name.clone(), self.session.clone());
    }
}
