#[macro_export]
macro_rules! profile_scope {
    () => {
        let _profiler_guard = $crate::enter_scope(concat!(module_path!(), "::", line!()));
    };

    ($name:literal) => {
        let _profiler_guard = $crate::enter_scope($name);
    };
}
