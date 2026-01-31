use super::*;

use std::ffi::{c_char, c_void};

pub mod ir;
pub mod proc;
pub mod valid;

// TODO: these should allocate and we should have a deallocate for these

unsafe fn slice_to_ffi<T, O, F: FnOnce(&T) -> O>(v: &[T], f: F) -> *mut O {
    todo!()
}

unsafe fn unique_arena_to_ffi<T: Clone, O, F: FnOnce(&T) -> O>(
    arena: &naga::UniqueArena<T>,
    f: F,
) -> *mut O {
    todo!()
}

unsafe fn arena_to_ffi<T: Clone, O, F: FnOnce(&T) -> O>(arena: &naga::Arena<T>, f: F) -> *mut O {
    todo!()
}

unsafe fn string_to_ffi(s: &str) -> *mut c_char {
    todo!()
}

// unsafe fn unsafe_copy<T>(val: T) -> T {

// }

fn bool_to_ffi(b: bool) -> u8 {
    if b { 1 } else { 0 }
}

const EMPTY_MUT: *mut ffi::Empty = core::ptr::null_mut();
