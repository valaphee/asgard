use std::cell::Cell;

pub use ffi::*;

#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

thread_local! {
    static ENV: Cell<*mut JNIEnv> = Cell::new(std::ptr::null_mut());
}
