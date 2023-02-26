use cfg_if;

// `std::time::SystemTime` panics on `wasm32-unknown-unknown` target so use a u64 timestamp
// instead
cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
        use js_sys;

        /// Time is represented as a u64 unix timestamp on wasm32-unknown-unknown
        pub type Time = u64;

        /// Returns the current time
        pub fn get_current_time() -> Time {
            (js_sys::Date::now() / 1000.0) as u64
        }
    } else {
        use std::time::SystemTime;

        /// Time is represented as a SystemTime on other targets
        pub type Time = SystemTime;

        /// Returns the current time
        pub fn get_current_time() -> Time {
            SystemTime::now()
        }
    }
}
