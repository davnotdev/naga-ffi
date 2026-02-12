use super::*;

pub fn keep_unused_to_naga(keep_unused: ffi::KeepUnused) -> naga::compact::KeepUnused {
    match keep_unused {
        ffi::KeepUnused_KeepUnused_No => naga::compact::KeepUnused::No,
        ffi::KeepUnused_KeepUnused_Yes => naga::compact::KeepUnused::Yes,
        _ => panic!("Unknown KeepUnused"),
    }
}
