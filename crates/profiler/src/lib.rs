pub mod clock;
pub mod guard;

mod macros;
mod platform;
mod runtime;
mod session;

pub use guard::enter_scope;
pub use runtime::read_cycles;
pub use session::print_sessions;
