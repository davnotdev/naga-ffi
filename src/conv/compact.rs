use super::*;

pub fn keep_unused_to_naga(keep_unused: ffi::NagaKeepUnused) -> naga::compact::KeepUnused {
    match keep_unused {
        ffi::NagaKeepUnused_NagaKeepUnused_No => naga::compact::KeepUnused::No,
        ffi::NagaKeepUnused_NagaKeepUnused_Yes => naga::compact::KeepUnused::Yes,
        _ => panic!("Unknown KeepUnused"),
    }
}
