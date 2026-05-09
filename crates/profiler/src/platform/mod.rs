#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(all(not(target_os = "macos"), target_arch = "x86_64"))]
pub mod x86_64;

pub trait CycleCounter {
    fn read_cycles(&self) -> u64;
}

#[cfg(target_os = "macos")]
pub type NativeCycleCounter = macos::KpcCounter;

#[cfg(all(not(target_os = "macos"), target_arch = "x86_64"))]
pub type NativeCycleCounter = x86_64::RdtscCounter;

pub fn cycle_counter() -> Result<NativeCycleCounter, String> {
    NativeCycleCounter::new()
}
