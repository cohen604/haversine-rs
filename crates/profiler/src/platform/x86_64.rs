use std::arch::x86_64::_rdtsc;

pub struct RdtscCounter;

impl super::CycleCounter for RdtscCounter {
    fn read_cycles(&self) -> u64 {
        rdtsc()
    }
}

pub fn rdtsc() -> u64 {
    unsafe { _rdtsc() }
}
