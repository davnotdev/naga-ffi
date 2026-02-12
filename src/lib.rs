pub mod conv;
pub mod ffi;

use naga::{back, front, valid};
use static_assertions as sa;

// TODO: feature the conversion functions!

#[cfg(feature = "glsl-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_glsl_parse(
    options: ffi::GLSLFrontOptions,
    source: *const ::std::os::raw::c_char,
) -> ffi::GLSLFrontendResult {
    todo!()
}

#[cfg(feature = "spv-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_spv_parse(
    options: ffi::SPVFrontOptions,
    source: *mut u32,
    source_length: u32,
) -> ffi::SPVFrontendResult {
    todo!()
}

#[cfg(feature = "wgsl-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_wgsl_parse(
    options: ffi::WGSLFrontOptions,
    source: *const ::std::os::raw::c_char,
) -> ffi::WGSLFrontendResult {
    todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_new(
    flags: ffi::ValidationFlags,
    capabilities: ffi::Capabilities,
) -> ffi::Validator {
    todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_reset(validator: *mut ffi::Validator) {
    todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_validate(
    validator: *mut ffi::Validator,
    module: *mut ffi::Module,
) -> ffi::ValidateResult {
    todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_validate_resolved_overrides(
    validator: *mut ffi::Validator,
    module: *mut ffi::Module,
) -> ffi::ValidateResult {
    todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_compact_compact(
    module: *mut ffi::Module,
    keep_unused: ffi::KeepUnused,
) {
    todo!()
}

#[cfg(feature = "dot-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_dot_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::DOTBackOptions,
) -> ffi::DOTWriteResult {
    todo!()
}

#[cfg(feature = "glsl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_glsl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::GLSLBackOptions,
    pipeline_options: ffi::GLSLBackPipelineOptions,
    policies: ffi::BoundsCheckPolicies,
) -> ffi::GLSLWriteResult {
    todo!()
}

#[cfg(feature = "hlsl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_hlsl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::HLSLBackOptions,
    pipeline_options: ffi::HLSLBackPipelineOptions,
    fragment_entry_point: *mut ffi::HLSLBackFragmentEntryPoint,
) -> ffi::HLSLWriteResult {
    todo!()
}

#[cfg(feature = "msl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_msl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::MSLBackOptions,
    pipeline_options: ffi::MSLBackPipelineOptions,
) -> ffi::MSLWriteResult {
    todo!()
}

#[cfg(feature = "spv-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_spv_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::SPVBackOptions,
    pipeline_options: ffi::SPVBackPipelineOptions,
) -> ffi::SPVWriteResult {
    todo!()
}

#[cfg(feature = "wgsl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_wgsl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    writer_flags: ffi::WGSLBackWriterFlags,
) -> ffi::WGSLWriteResult {
    todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_process_overrides(
    module: *mut ffi::Module,
    module_flags: ffi::ModuleFillFlags,
    module_info: *mut ffi::ModuleInfo,
    module_fill_flags: ffi::ModuleInfoFillFlags,
    entry_point_stage: ffi::ShaderStage,
    entry_point_name: *const ::std::os::raw::c_char,
    constants: *mut ffi::PipelineConstant,
    constants_count: u32,
) -> ffi::ProcessOverridesResult {
    todo!()
}
