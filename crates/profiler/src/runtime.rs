use std::sync::OnceLock;

use crate::platform::{CycleCounter, NativeCycleCounter, cycle_counter};

static COUNTER: OnceLock<Result<NativeCycleCounter, String>> = OnceLock::new();

pub fn read_cycles() -> Option<u64> {
    let counter = COUNTER.get_or_init(cycle_counter).as_ref().ok()?;
    Some(counter.read_cycles())
}
