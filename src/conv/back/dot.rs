use super::*;

pub fn dot_back_options_to_ffi(options: &naga::back::dot::Options) -> ffi::DOTBackOptions {
    ffi::DOTBackOptions {
        cfg_only: bool_to_ffi(options.cfg_only),
    }
}
