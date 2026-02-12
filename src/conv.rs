use super::*;

use std::ffi::{c_char, c_void};

pub mod back;
pub mod compact;
pub mod front;
pub mod ir;
pub mod proc;
pub mod valid;

pub use compact::*;
pub use ir::*;
pub use proc::*;
pub use valid::*;

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

fn bool_to_ffi(b: bool) -> u8 {
    if b { 1 } else { 0 }
}

const EMPTY_MUT: *mut ffi::Empty = core::ptr::null_mut();

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

unsafe fn slice_to_naga<T, O, F: FnOnce(&T) -> O>(v: &[T], f: F) -> *mut O {
    todo!()
}

unsafe fn string_to_naga(s: *mut c_char) -> String {
    todo!()
}
