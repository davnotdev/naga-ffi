use super::*;

use std::ffi::{c_char, c_void};

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

pub unsafe fn slice_to_ffi<T, O, F: FnOnce(&T) -> O>(v: &[T], f: F) -> *mut O {
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

pub unsafe fn string_to_ffi(s: &str) -> *mut c_char {
    todo!()
}

fn bool_to_ffi(b: bool) -> u8 {
    if b { 1 } else { 0 }
}

pub const EMPTY_MUT: *mut ffi::Empty = core::ptr::null_mut();

fn span_to_ffi(span: &naga::Span) -> ffi::Span {
    ffi::Span {
        start: span.to_range().map(|r| r.start as u32).unwrap_or(0),
        end: span.to_range().map(|r| r.end as u32).unwrap_or(0),
    }
}

fn span_context_to_ffi(span_context: &naga::SpanContext) -> ffi::SpanContext {
    ffi::SpanContext {
        span: span_to_ffi(&span_context.0),
        message: unsafe { string_to_ffi(&span_context.1) },
    }
}

fn bool_to_naga(b: u8) -> bool {
    match b {
        0 => false,
        1 => true,
        n => panic!("{} is not a valid bool", n),
    }
}

pub fn string_to_naga(s: *const c_char) -> String {
    todo!()
}
