pub mod conv;
pub mod ffi;

use static_assertions as sa;
use std::mem::ManuallyDrop;

#[cfg(feature = "glsl-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_glsl_parse(
    options: ffi::GLSLFrontOptions,
    source: *const ::std::os::raw::c_char,
    fill_flags: ffi::ModuleFillFlags,
    out_result: *mut ffi::GLSLFrontResult,
) -> ffi::Bool {
    let mut frontend = naga::front::glsl::Frontend::default();
    let options = conv::glsl_front_options_to_naga(&options);
    unsafe {
        let flags = (*out_result).flags;
        let source = conv::string_to_naga(source);
        match frontend.parse(&options, &source) {
            Ok(module) => {
                *out_result = ffi::GLSLFrontResult {
                    flags,
                    module: conv::module_to_ffi(module, fill_flags),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error =
                    if (flags & ffi::FrontResultOption_FrontResultOption_FormattedErrorOnly) != 0 {
                        ffi::GLSLFrontResult__bindgen_ty_1 {
                            fmt_error: conv::string_to_ffi(&error.to_string()),
                        }
                    } else {
                        ffi::GLSLFrontResult__bindgen_ty_1 {
                            errors: conv::glsl_front_parse_errors_to_ffi(&error),
                        }
                    };
                *out_result = ffi::GLSLFrontResult {
                    flags,
                    __bindgen_anon_1: error,
                    ..Default::default()
                };
                0
            }
        }
    }
}

#[cfg(feature = "spv-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_spv_parse(
    options: ffi::SPVFrontOptions,
    source: *mut u32,
    source_length: u32,
    fill_flags: ffi::ModuleFillFlags,
    out_result: *mut ffi::SPVFrontResult,
) -> ffi::Bool {
    let options = conv::spv_front_options_to_naga(&options);
    let spv_slice = unsafe { std::slice::from_raw_parts(source, source_length as usize) };
    let frontend = naga::front::spv::Frontend::new(spv_slice.iter().cloned(), &options);
    unsafe {
        let flags = (*out_result).flags;
        match frontend.parse() {
            Ok(module) => {
                *out_result = ffi::SPVFrontResult {
                    flags,
                    module: conv::module_to_ffi(module, fill_flags),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error =
                    if (flags & ffi::FrontResultOption_FrontResultOption_FormattedErrorOnly) != 0 {
                        ffi::SPVFrontResult__bindgen_ty_1 {
                            fmt_error: conv::string_to_ffi(&error.to_string()),
                        }
                    } else {
                        ffi::SPVFrontResult__bindgen_ty_1 {
                            error: conv::spv_front_error_to_ffi(&error),
                        }
                    };
                *out_result = ffi::SPVFrontResult {
                    flags,
                    __bindgen_anon_1: error,
                    ..Default::default()
                };
                0
            }
        }
    }
}

#[cfg(feature = "wgsl-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_wgsl_parse(
    options: ffi::WGSLFrontOptions,
    source: *const ::std::os::raw::c_char,
    fill_flags: ffi::ModuleFillFlags,
    out_result: *mut ffi::WGSLFrontResult,
) -> ffi::Bool {
    let options = conv::wgsl_front_options_to_naga(&options);
    let mut frontend = naga::front::wgsl::Frontend::new_with_options(options);
    let source = unsafe { conv::string_to_naga(source) };
    unsafe {
        let flags = (*out_result).flags;
        match frontend.parse(&source) {
            Ok(module) => {
                *out_result = ffi::WGSLFrontResult {
                    flags,
                    module: conv::module_to_ffi(module, fill_flags),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error =
                    if (flags & ffi::FrontResultOption_FrontResultOption_FormattedErrorOnly) != 0 {
                        ffi::WGSLFrontResult__bindgen_ty_1 {
                            fmt_error: conv::string_to_ffi(&error.to_string()),
                        }
                    } else {
                        ffi::WGSLFrontResult__bindgen_ty_1 {
                            error: conv::wgsl_front_parse_error_to_ffi(&error),
                        }
                    };
                *out_result = ffi::WGSLFrontResult {
                    flags,
                    __bindgen_anon_1: error,
                    ..Default::default()
                };
                0
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_new(
    flags: ffi::ValidationFlagsFlags,
    capabilities: ffi::Capabilities,
) -> ffi::Validator {
    let flags = conv::validation_flags_to_naga(flags);
    let capabilities = conv::capabilities_to_naga(capabilities);

    let validator = naga::valid::Validator::new(flags, capabilities);
    unsafe { conv::validator_to_ffi(validator) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_reset(validator: *mut ffi::Validator) {
    let mut validator = unsafe {
        let validator = &mut *validator;
        ManuallyDrop::new(Box::from_raw(
            validator._inner_validator as *mut naga::valid::Validator,
        ))
    };
    validator.reset();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_validate(
    validator: *mut ffi::Validator,
    module: *mut ffi::Module,
    out_result: *mut ffi::ValidateResult,
) -> ffi::Bool {
    let module = unsafe {
        let module = &mut *module;
        ManuallyDrop::new(Box::from_raw(module._inner_module as *mut naga::Module))
    };
    let mut validator = unsafe {
        let validator = &mut *validator;
        ManuallyDrop::new(Box::from_raw(
            validator._inner_validator as *mut naga::valid::Validator,
        ))
    };
    unsafe {
        let flags = (*out_result).flags;
        match validator.validate(&module) {
            Ok(module_info) => {
                *out_result = ffi::ValidateResult {
                    flags,
                    module_info: conv::module_info_to_ffi(module_info),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error = if (flags
                    & ffi::ValidateResultOption_ValidateResultOption_FormattedErrorOnly)
                    != 0
                {
                    ffi::ValidateResult__bindgen_ty_1 {
                        fmt_error: conv::string_to_ffi(&error.to_string()),
                    }
                } else {
                    ffi::ValidateResult__bindgen_ty_1 {
                        error: conv::EMPTY_MUT,
                    }
                };
                *out_result = ffi::ValidateResult {
                    flags,
                    __bindgen_anon_1: error,
                    ..Default::default()
                };
                0
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_validate_resolved_overrides(
    validator: *mut ffi::Validator,
    module: *mut ffi::Module,
    out_result: *mut ffi::ValidateResult,
) -> ffi::Bool {
    todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_compact_compact(
    module: *mut ffi::Module,
    keep_unused: ffi::KeepUnused,
) {
    let keep_unused = conv::keep_unused_to_naga(keep_unused);
    let mut module = unsafe {
        let module = &mut *module;
        ManuallyDrop::new(Box::from_raw(module._inner_module as *mut naga::Module))
    };
    naga::compact::compact(&mut module, keep_unused);
}

#[cfg(feature = "dot-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_dot_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::DOTBackOptions,
    out_result: *mut ffi::DOTWriteResult,
) -> ffi::Bool {
    let module = unsafe {
        let module = &mut *module;
        ManuallyDrop::new(Box::from_raw(module._inner_module as *mut naga::Module))
    };
    let module_info = unsafe {
        let module_info = &mut *module_info;
        ManuallyDrop::new(Box::from_raw(
            module_info._inner_module_info as *mut naga::valid::ModuleInfo,
        ))
    };
    let options = conv::dot_back_options_to_naga(&options);
    let result = naga::back::dot::write(&module, Some(&module_info), options);

    unsafe {
        let flags = (*out_result).flags;
        match result {
            Ok(result) => {
                *out_result = ffi::DOTWriteResult {
                    flags,
                    output: conv::string_to_ffi(&result),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                *out_result = ffi::DOTWriteResult {
                    flags,
                    error: conv::string_to_ffi(&error.to_string()),
                    ..Default::default()
                };
                0
            }
        }
    }
}

#[cfg(feature = "glsl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_glsl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::GLSLBackOptions,
    pipeline_options: ffi::GLSLBackPipelineOptions,
    policies: ffi::BoundsCheckPolicies,
    out_result: *mut ffi::GLSLWriteResult,
) -> ffi::Bool {
    let module = unsafe {
        let module = &mut *module;
        ManuallyDrop::new(Box::from_raw(module._inner_module as *mut naga::Module))
    };
    let module_info = unsafe {
        let module_info = &mut *module_info;
        ManuallyDrop::new(Box::from_raw(
            module_info._inner_module_info as *mut naga::valid::ModuleInfo,
        ))
    };
    let options = conv::glsl_back_options_to_naga(&options);
    let pipeline_options = conv::glsl_back_pipeline_options_to_naga(&pipeline_options);
    let policies = conv::bound_check_policies_to_naga(&policies);
    let mut buf = String::new();
    let writer = naga::back::glsl::Writer::new(
        &mut buf,
        &module,
        &module_info,
        &options,
        &pipeline_options,
        policies,
    );

    let flags = unsafe { (*out_result).flags };

    let create_error = |error: naga::back::glsl::Error| unsafe {
        let error = if (flags & ffi::WriteResultOption_WriteResultOption_FormattedErrorOnly) != 0 {
            ffi::GLSLWriteResult__bindgen_ty_1 {
                fmt_error: conv::string_to_ffi(&error.to_string()),
            }
        } else {
            ffi::GLSLWriteResult__bindgen_ty_1 {
                error: conv::glsl_back_error_to_ffi(&error),
            }
        };
        ffi::GLSLWriteResult {
            flags,
            __bindgen_anon_1: error,
            ..Default::default()
        }
    };

    unsafe {
        let mut writer = match writer {
            Ok(writer) => writer,
            Err(error) => {
                *out_result = create_error(error);
                return 0;
            }
        };
        match writer.write() {
            Ok(_reflection_info) => {
                *out_result = ffi::GLSLWriteResult {
                    flags,
                    output: conv::string_to_ffi(&buf),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                *out_result = create_error(error);
                0
            }
        }
    }
}

#[cfg(feature = "hlsl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_hlsl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::HLSLBackOptions,
    pipeline_options: ffi::HLSLBackPipelineOptions,
    fragment_entry_point: *mut ffi::HLSLBackFragmentEntryPoint,
    out_result: *mut ffi::HLSLWriteResult,
) -> ffi::Bool {
    let module = unsafe {
        let module = &mut *module;
        ManuallyDrop::new(Box::from_raw(module._inner_module as *mut naga::Module))
    };
    let module_info = unsafe {
        let module_info = &mut *module_info;
        ManuallyDrop::new(Box::from_raw(
            module_info._inner_module_info as *mut naga::valid::ModuleInfo,
        ))
    };
    let options = conv::hlsl_back_options_to_naga(&options);
    let pipeline_options = conv::hlsl_back_pipeline_options_to_naga(&pipeline_options);
    if !fragment_entry_point.is_null() {
        panic!("HLSLBackFragmentEntryPoint is unimplemented")
    }
    let mut buf = String::new();
    let mut writer = naga::back::hlsl::Writer::new(&mut buf, &options, &pipeline_options);

    unsafe {
        let flags = (*out_result).flags;
        match writer.write(&module, &module_info, None) {
            Ok(_reflection_info) => {
                *out_result = ffi::HLSLWriteResult {
                    flags,
                    output: conv::string_to_ffi(&buf),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error =
                    if (flags & ffi::WriteResultOption_WriteResultOption_FormattedErrorOnly) != 0 {
                        ffi::HLSLWriteResult__bindgen_ty_1 {
                            fmt_error: conv::string_to_ffi(&error.to_string()),
                        }
                    } else {
                        ffi::HLSLWriteResult__bindgen_ty_1 {
                            error: conv::hlsl_back_error_to_ffi(&error),
                        }
                    };
                *out_result = ffi::HLSLWriteResult {
                    flags,
                    __bindgen_anon_1: error,
                    ..Default::default()
                };
                0
            }
        }
    }
}

#[cfg(feature = "msl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_msl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::MSLBackOptions,
    pipeline_options: ffi::MSLBackPipelineOptions,
    out_result: *mut ffi::MSLWriteResult,
) -> ffi::Bool {
    let module = unsafe {
        let module = &mut *module;
        ManuallyDrop::new(Box::from_raw(module._inner_module as *mut naga::Module))
    };
    let module_info = unsafe {
        let module_info = &mut *module_info;
        ManuallyDrop::new(Box::from_raw(
            module_info._inner_module_info as *mut naga::valid::ModuleInfo,
        ))
    };
    let options = conv::msl_back_options_to_naga(&options);
    let pipeline_options = conv::msl_back_pipeline_options_to_naga(&pipeline_options);

    unsafe {
        let flags = (*out_result).flags;
        match naga::back::msl::write_string(&module, &module_info, &options, &pipeline_options) {
            Ok((result, _translation_info)) => {
                *out_result = ffi::MSLWriteResult {
                    flags,
                    output: conv::string_to_ffi(&result),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error =
                    if (flags & ffi::WriteResultOption_WriteResultOption_FormattedErrorOnly) != 0 {
                        ffi::MSLWriteResult__bindgen_ty_1 {
                            fmt_error: conv::string_to_ffi(&error.to_string()),
                        }
                    } else {
                        ffi::MSLWriteResult__bindgen_ty_1 {
                            error: conv::msl_back_error_to_ffi(&error),
                        }
                    };
                *out_result = ffi::MSLWriteResult {
                    flags,
                    __bindgen_anon_1: error,
                    ..Default::default()
                };
                0
            }
        }
    }
}

#[cfg(feature = "spv-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_spv_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::SPVBackOptions,
    pipeline_options: *mut ffi::SPVBackPipelineOptions,
    out_result: *mut ffi::SPVWriteResult,
) -> ffi::Bool {
    let module = unsafe {
        let module = &mut *module;
        ManuallyDrop::new(Box::from_raw(module._inner_module as *mut naga::Module))
    };
    let module_info = unsafe {
        let module_info = &mut *module_info;
        ManuallyDrop::new(Box::from_raw(
            module_info._inner_module_info as *mut naga::valid::ModuleInfo,
        ))
    };
    let options = conv::spv_back_options_to_naga(&options);
    let pipeline_options = if pipeline_options.is_null() {
        None
    } else {
        unsafe { Some(conv::spv_back_pipeline_options_to_naga(&*pipeline_options)) }
    };

    unsafe {
        let flags = (*out_result).flags;
        match naga::back::spv::write_vec(&module, &module_info, &options, pipeline_options.as_ref())
        {
            Ok(result) => {
                *out_result = ffi::SPVWriteResult {
                    flags,
                    output: conv::slice_to_ffi(&result, |v| *v),
                    output_count: result.len() as u32,
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error =
                    if (flags & ffi::WriteResultOption_WriteResultOption_FormattedErrorOnly) != 0 {
                        ffi::SPVWriteResult__bindgen_ty_1 {
                            fmt_error: conv::string_to_ffi(&error.to_string()),
                        }
                    } else {
                        ffi::SPVWriteResult__bindgen_ty_1 {
                            error: conv::spv_back_error_to_ffi(&error),
                        }
                    };
                *out_result = ffi::SPVWriteResult {
                    flags,
                    __bindgen_anon_1: error,
                    ..Default::default()
                };
                0
            }
        }
    }
}

#[cfg(feature = "wgsl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_wgsl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    writer_flags: ffi::WGSLBackWriterFlagsFlags,
    out_result: *mut ffi::WGSLWriteResult,
) -> ffi::Bool {
    let module = unsafe {
        let module = &mut *module;
        ManuallyDrop::new(Box::from_raw(module._inner_module as *mut naga::Module))
    };
    let module_info = unsafe {
        let module_info = &mut *module_info;
        ManuallyDrop::new(Box::from_raw(
            module_info._inner_module_info as *mut naga::valid::ModuleInfo,
        ))
    };
    let writer_flags = conv::wgsl_back_writer_flags_to_naga(writer_flags);

    unsafe {
        let flags = (*out_result).flags;
        match naga::back::wgsl::write_string(&module, &module_info, writer_flags) {
            Ok(result) => {
                *out_result = ffi::WGSLWriteResult {
                    output: conv::string_to_ffi(&result),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error =
                    if (flags & ffi::WriteResultOption_WriteResultOption_FormattedErrorOnly) != 0 {
                        ffi::WGSLWriteResult__bindgen_ty_1 {
                            fmt_error: conv::string_to_ffi(&error.to_string()),
                        }
                    } else {
                        ffi::WGSLWriteResult__bindgen_ty_1 {
                            error: conv::wgsl_back_error_to_ffi(&error),
                        }
                    };
                *out_result = ffi::WGSLWriteResult {
                    flags,
                    __bindgen_anon_1: error,
                    ..Default::default()
                };
                0
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_process_overrides(
    module: *mut ffi::Module,
    module_flags: ffi::ModuleFillFlags,
    module_info: *mut ffi::ModuleInfo,
    module_fill_flags: ffi::ModuleInfoFillFlags,
    entry_point_stage: ffi::ShaderStage,
    entry_point_name: *const std::ffi::c_char,
    constants: *mut ffi::PipelineConstant,
    constants_count: u32,
    out_result: *mut ffi::ProcessOverridesResult,
) -> ffi::Bool {
    let module = unsafe {
        let module = &mut *module;
        ManuallyDrop::new(Box::from_raw(module._inner_module as *mut naga::Module))
    };
    let module_info = unsafe {
        let module_info = &mut *module_info;
        ManuallyDrop::new(Box::from_raw(
            module_info._inner_module_info as *mut naga::valid::ModuleInfo,
        ))
    };
    todo!()
}
