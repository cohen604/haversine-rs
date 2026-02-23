use std::os::raw::{c_int, c_uint, c_void};

// Function pointer types
type KpcGetThreadCounters =
    unsafe extern "C" fn(tid: c_uint, count: c_uint, counters: *mut u64) -> c_int;
type KpcSetCounting = unsafe extern "C" fn(classes: c_uint) -> c_int;
type KpcSetThreadCounting = unsafe extern "C" fn(classes: c_uint) -> c_int;
type KpcGetCounterCount = unsafe extern "C" fn(classes: c_uint) -> c_uint;
type KpcSetConfig = unsafe extern "C" fn(classes: c_uint, config: *mut u64) -> c_int;
type KpcGetConfig = unsafe extern "C" fn(classes: c_uint, config: *mut u64) -> c_int;
type KpcForceAllCtrsSet = unsafe extern "C" fn(val: c_int) -> c_int;

const KPC_CLASS_FIXED_MASK: u32 = 1;
const KPC_CLASS_CONFIGURABLE_MASK: u32 = 2;

pub struct Kpc {
    get_thread_counters: KpcGetThreadCounters,
    set_counting: KpcSetCounting,
    set_thread_counting: KpcSetThreadCounting,
    get_counter_count: KpcGetCounterCount,
    set_config: KpcSetConfig,
    force_all_ctrs_set: KpcForceAllCtrsSet,
    counter_count: u32,
}

impl Kpc {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let handle = libc::dlopen(
                b"/System/Library/PrivateFrameworks/kperf.framework/kperf\0".as_ptr() as *const i8,
                libc::RTLD_NOW,
            );
            if handle.is_null() {
                return Err("Failed to load kperf framework".into());
            }

            macro_rules! load_sym {
                ($name:ident, $type:ty) => {{
                    let sym = libc::dlsym(
                        handle,
                        concat!(stringify!($name), "\0").as_ptr() as *const i8,
                    );
                    if sym.is_null() {
                        return Err(format!("Failed to load symbol: {}", stringify!($name)));
                    }
                    std::mem::transmute::<*mut c_void, $type>(sym)
                }};
            }

            let get_thread_counters: KpcGetThreadCounters =
                load_sym!(kpc_get_thread_counters, KpcGetThreadCounters);
            let set_counting: KpcSetCounting = load_sym!(kpc_set_counting, KpcSetCounting);
            let set_thread_counting: KpcSetThreadCounting =
                load_sym!(kpc_set_thread_counting, KpcSetThreadCounting);
            let get_counter_count: KpcGetCounterCount =
                load_sym!(kpc_get_counter_count, KpcGetCounterCount);
            let set_config: KpcSetConfig = load_sym!(kpc_set_config, KpcSetConfig);
            let force_all_ctrs_set: KpcForceAllCtrsSet =
                load_sym!(kpc_force_all_ctrs_set, KpcForceAllCtrsSet);

            let mut kpc = Self {
                get_thread_counters,
                set_counting,
                set_thread_counting,
                get_counter_count,
                set_config,
                force_all_ctrs_set,
                counter_count: 0,
            };

            kpc.init()?;
            Ok(kpc)
        }
    }

    fn init(&mut self) -> Result<(), String> {
        unsafe {
            let classes = KPC_CLASS_FIXED_MASK | KPC_CLASS_CONFIGURABLE_MASK;

            // Force access to counters (requires sudo)
            if (self.force_all_ctrs_set)(1) != 0 {
                return Err("kpc_force_all_ctrs_set failed (are you running with sudo?)".into());
            }

            if (self.set_counting)(classes) != 0 {
                return Err("kpc_set_counting failed".into());
            }

            if (self.set_thread_counting)(classes) != 0 {
                return Err("kpc_set_thread_counting failed".into());
            }

            self.counter_count = (self.get_counter_count)(classes);
            Ok(())
        }
    }

    #[inline]
    pub fn read_cycles(&self) -> u64 {
        let mut counters = [0u64; 32];
        unsafe {
            (self.get_thread_counters)(0, self.counter_count, counters.as_mut_ptr());
        }
        // Counter layout varies; fixed counter 0 or 2 is usually cycles
        counters[0] + counters[2]
    }
}
