use super::*;

use std::{
    ffi::{CStr, CString, c_char, c_void},
    hash::Hash,
};

mod back;
mod compact;
mod front;
mod ir;
mod proc;
mod valid;

pub use back::*;
pub use compact::*;
pub use front::*;
pub use ir::*;
pub use proc::*;
pub use valid::*;

// TODO: these should allocate and we should have a deallocate for these

pub unsafe fn slice_to_ffi<T, O, F: Fn(&T) -> O>(v: &[T], f: F) -> *mut O {
    let mut converted = ManuallyDrop::new(v.iter().map(f).collect::<Vec<O>>());
    converted.as_mut_ptr()
}

unsafe fn unique_arena_to_ffi<T: Clone + Eq + Hash, O, F: Fn(&T) -> O>(
    arena: &naga::UniqueArena<T>,
    f: F,
) -> *mut O {
    let mut converted = ManuallyDrop::new(arena.iter().map(|(_, v)| f(v)).collect::<Vec<O>>());
    converted.as_mut_ptr()
}

unsafe fn arena_to_ffi<T: Clone, O, F: Fn(&T) -> O>(arena: &naga::Arena<T>, f: F) -> *mut O {
    let mut converted = ManuallyDrop::new(arena.iter().map(|(_, v)| f(v)).collect::<Vec<O>>());
    converted.as_mut_ptr()
}

pub unsafe fn string_to_ffi(s: &str) -> *mut c_char {
    let c_string = CString::new(s).expect("String contains null byte");
    c_string.into_raw()
}

fn bool_to_ffi(b: bool) -> u8 {
    if b { 1 } else { 0 }
}

pub const EMPTY_MUT: *mut ffi::NagaEmpty = core::ptr::null_mut();

fn span_to_ffi(span: &naga::Span) -> ffi::NagaSpan {
    ffi::NagaSpan {
        start: span.to_range().map(|r| r.start as u32).unwrap_or(0),
        end: span.to_range().map(|r| r.end as u32).unwrap_or(0),
    }
}

fn bool_to_naga(b: u8) -> bool {
    match b {
        0 => false,
        1 => true,
        n => panic!("{} is not a valid bool", n),
    }
}

pub unsafe fn string_to_naga(s: *const c_char) -> String {
    unsafe {
        CStr::from_ptr(s)
            .to_str()
            .expect("Invalid UTF-8 in C string")
            .to_owned()
    }
}
