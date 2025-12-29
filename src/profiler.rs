use std::time::SystemTime;

#[cfg(target_arch = "x86_64")]
pub fn rdtsc() -> u64 {
    use std::arch::x86_64::_rdtsc;
    unsafe { _rdtsc() }
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub fn rdtsc() -> u64 {
    let val: u64;
    unsafe {
        std::arch::asm!("mrs {}, cntvct_el0", out(reg) val);
    }
    val
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_os_timer_frequency() -> u128 {
    1_000_000 // 1 second
}

pub fn read_os_timer() -> u128 {
    let now = SystemTime::now();

    now.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

pub fn get_cpu_frequency() {
    let os_freq = get_os_timer_frequency();
    println!("os_freq: {os_freq}");

    let cpu_start = rdtsc();
    let os_start = read_os_timer();
    let mut os_end = 0;
    let mut os_elapsed = 0;
    while os_elapsed < os_freq {
        os_end = read_os_timer();
        os_elapsed = os_end - os_start;
    }
    let cpu_end = rdtsc();

    println!("os timer: {os_start:?} -> {os_end:?} = {os_elapsed}");
    println!("os seconds: {}", os_elapsed / os_freq);
    println!(
        "cpu: {cpu_start:?} -> {cpu_end:?} = {}",
        cpu_end - cpu_start
    );
}
