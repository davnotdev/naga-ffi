use super::*;

pub fn dot_back_options_to_naga(options: &ffi::NagaDOTBackOptions) -> naga::back::dot::Options {
    naga::back::dot::Options {
        cfg_only: bool_to_naga(options.cfg_only),
    }
}
