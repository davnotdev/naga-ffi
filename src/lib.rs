pub mod conv;
pub mod ffi;

use static_assertions as sa;
use std::mem::ManuallyDrop;

#[cfg(feature = "glsl-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_glsl_parse(
    options: ffi::NagaGLSLFrontOptions,
    source: *const ::std::os::raw::c_char,
    fill_flags: ffi::NagaModuleFillFlags,
    out_result: *mut ffi::NagaGLSLFrontResult,
) -> ffi::NagaBool {
    let mut frontend = naga::front::glsl::Frontend::default();
    let options = conv::glsl_front_options_to_naga(&options);
    unsafe {
        let flags = (*out_result).flags;
        let source = conv::string_to_naga(source);
        match frontend.parse(&options, &source) {
            Ok(module) => {
                *out_result = ffi::NagaGLSLFrontResult {
                    flags,
                    module: conv::module_to_ffi(module, fill_flags),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error = if (flags
                    & ffi::NagaFrontResultOption_NagaFrontResultOption_FormattedErrorOnly)
                    != 0
                {
                    ffi::NagaGLSLFrontResult__bindgen_ty_1 {
                        fmt_error: conv::string_to_ffi(&error.to_string()),
                    }
                } else {
                    ffi::NagaGLSLFrontResult__bindgen_ty_1 {
                        errors: conv::glsl_front_parse_errors_to_ffi(&error),
                    }
                };
                *out_result = ffi::NagaGLSLFrontResult {
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
    options: ffi::NagaSPVFrontOptions,
    source: *mut u32,
    source_length: u32,
    fill_flags: ffi::NagaModuleFillFlags,
    out_result: *mut ffi::NagaSPVFrontResult,
) -> ffi::NagaBool {
    let options = conv::spv_front_options_to_naga(&options);
    let spv_slice = unsafe { std::slice::from_raw_parts(source, source_length as usize) };
    let frontend = naga::front::spv::Frontend::new(spv_slice.iter().cloned(), &options);
    unsafe {
        let flags = (*out_result).flags;
        match frontend.parse() {
            Ok(module) => {
                *out_result = ffi::NagaSPVFrontResult {
                    flags,
                    module: conv::module_to_ffi(module, fill_flags),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error = if (flags
                    & ffi::NagaFrontResultOption_NagaFrontResultOption_FormattedErrorOnly)
                    != 0
                {
                    ffi::NagaSPVFrontResult__bindgen_ty_1 {
                        fmt_error: conv::string_to_ffi(&error.to_string()),
                    }
                } else {
                    ffi::NagaSPVFrontResult__bindgen_ty_1 {
                        error: conv::spv_front_error_to_ffi(&error),
                    }
                };
                *out_result = ffi::NagaSPVFrontResult {
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
    options: ffi::NagaWGSLFrontOptions,
    source: *const ::std::os::raw::c_char,
    fill_flags: ffi::NagaModuleFillFlags,
    out_result: *mut ffi::NagaWGSLFrontResult,
) -> ffi::NagaBool {
    let options = conv::wgsl_front_options_to_naga(&options);
    let mut frontend = naga::front::wgsl::Frontend::new_with_options(options);
    let source = unsafe { conv::string_to_naga(source) };
    unsafe {
        let flags = (*out_result).flags;
        match frontend.parse(&source) {
            Ok(module) => {
                *out_result = ffi::NagaWGSLFrontResult {
                    flags,
                    module: conv::module_to_ffi(module, fill_flags),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error = if (flags
                    & ffi::NagaFrontResultOption_NagaFrontResultOption_FormattedErrorOnly)
                    != 0
                {
                    ffi::NagaWGSLFrontResult__bindgen_ty_1 {
                        fmt_error: conv::string_to_ffi(&error.to_string()),
                    }
                } else {
                    ffi::NagaWGSLFrontResult__bindgen_ty_1 {
                        error: conv::wgsl_front_parse_error_to_ffi(&error),
                    }
                };
                *out_result = ffi::NagaWGSLFrontResult {
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
    flags: ffi::NagaValidationFlagsFlags,
    capabilities: ffi::NagaCapabilities,
) -> ffi::NagaValidator {
    let flags = conv::validation_flags_to_naga(flags);
    let capabilities = conv::capabilities_to_naga(capabilities);

    let validator = naga::valid::Validator::new(flags, capabilities);
    unsafe { conv::validator_to_ffi(validator) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_reset(validator: *mut ffi::NagaValidator) {
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
    validator: *mut ffi::NagaValidator,
    module: *mut ffi::NagaModule,
    out_result: *mut ffi::NagaValidateResult,
) -> ffi::NagaBool {
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
                *out_result = ffi::NagaValidateResult {
                    flags,
                    module_info: conv::module_info_to_ffi(module_info),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error = if (flags
                    & ffi::NagaValidateResultOption_NagaValidateResultOption_FormattedErrorOnly)
                    != 0
                {
                    ffi::NagaValidateResult__bindgen_ty_1 {
                        fmt_error: conv::string_to_ffi(&error.to_string()),
                    }
                } else {
                    ffi::NagaValidateResult__bindgen_ty_1 {
                        error: conv::EMPTY_MUT,
                    }
                };
                *out_result = ffi::NagaValidateResult {
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
    validator: *mut ffi::NagaValidator,
    module: *mut ffi::NagaModule,
    out_result: *mut ffi::NagaValidateResult,
) -> ffi::NagaBool {
    todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_compact_compact(
    module: *mut ffi::NagaModule,
    keep_unused: ffi::NagaKeepUnused,
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
    module: *mut ffi::NagaModule,
    module_info: *mut ffi::NagaModuleInfo,
    options: ffi::NagaDOTBackOptions,
    out_result: *mut ffi::NagaDOTWriteResult,
) -> ffi::NagaBool {
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
                *out_result = ffi::NagaDOTWriteResult {
                    flags,
                    output: conv::string_to_ffi(&result),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                *out_result = ffi::NagaDOTWriteResult {
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
    module: *mut ffi::NagaModule,
    module_info: *mut ffi::NagaModuleInfo,
    options: ffi::NagaGLSLBackOptions,
    pipeline_options: ffi::NagaGLSLBackPipelineOptions,
    policies: ffi::NagaBoundsCheckPolicies,
    out_result: *mut ffi::NagaGLSLWriteResult,
) -> ffi::NagaBool {
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
        let error =
            if (flags & ffi::NagaWriteResultOption_NagaWriteResultOption_FormattedErrorOnly) != 0 {
                ffi::NagaGLSLWriteResult__bindgen_ty_1 {
                    fmt_error: conv::string_to_ffi(&error.to_string()),
                }
            } else {
                ffi::NagaGLSLWriteResult__bindgen_ty_1 {
                    error: conv::glsl_back_error_to_ffi(&error),
                }
            };
        ffi::NagaGLSLWriteResult {
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
                *out_result = ffi::NagaGLSLWriteResult {
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
    module: *mut ffi::NagaModule,
    module_info: *mut ffi::NagaModuleInfo,
    options: ffi::NagaHLSLBackOptions,
    pipeline_options: ffi::NagaHLSLBackPipelineOptions,
    fragment_entry_point: *mut ffi::NagaHLSLBackFragmentEntryPoint,
    out_result: *mut ffi::NagaHLSLWriteResult,
) -> ffi::NagaBool {
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
                *out_result = ffi::NagaHLSLWriteResult {
                    flags,
                    output: conv::string_to_ffi(&buf),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error = if (flags
                    & ffi::NagaWriteResultOption_NagaWriteResultOption_FormattedErrorOnly)
                    != 0
                {
                    ffi::NagaHLSLWriteResult__bindgen_ty_1 {
                        fmt_error: conv::string_to_ffi(&error.to_string()),
                    }
                } else {
                    ffi::NagaHLSLWriteResult__bindgen_ty_1 {
                        error: conv::hlsl_back_error_to_ffi(&error),
                    }
                };
                *out_result = ffi::NagaHLSLWriteResult {
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
    module: *mut ffi::NagaModule,
    module_info: *mut ffi::NagaModuleInfo,
    options: ffi::NagaMSLBackOptions,
    pipeline_options: ffi::NagaMSLBackPipelineOptions,
    out_result: *mut ffi::NagaMSLWriteResult,
) -> ffi::NagaBool {
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
                *out_result = ffi::NagaMSLWriteResult {
                    flags,
                    output: conv::string_to_ffi(&result),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error = if (flags
                    & ffi::NagaWriteResultOption_NagaWriteResultOption_FormattedErrorOnly)
                    != 0
                {
                    ffi::NagaMSLWriteResult__bindgen_ty_1 {
                        fmt_error: conv::string_to_ffi(&error.to_string()),
                    }
                } else {
                    ffi::NagaMSLWriteResult__bindgen_ty_1 {
                        error: conv::msl_back_error_to_ffi(&error),
                    }
                };
                *out_result = ffi::NagaMSLWriteResult {
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
    module: *mut ffi::NagaModule,
    module_info: *mut ffi::NagaModuleInfo,
    options: ffi::NagaSPVBackOptions,
    pipeline_options: *mut ffi::NagaSPVBackPipelineOptions,
    out_result: *mut ffi::NagaSPVWriteResult,
) -> ffi::NagaBool {
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
                *out_result = ffi::NagaSPVWriteResult {
                    flags,
                    output: conv::slice_to_ffi(&result, |v| *v),
                    output_count: result.len() as u32,
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error = if (flags
                    & ffi::NagaWriteResultOption_NagaWriteResultOption_FormattedErrorOnly)
                    != 0
                {
                    ffi::NagaSPVWriteResult__bindgen_ty_1 {
                        fmt_error: conv::string_to_ffi(&error.to_string()),
                    }
                } else {
                    ffi::NagaSPVWriteResult__bindgen_ty_1 {
                        error: conv::spv_back_error_to_ffi(&error),
                    }
                };
                *out_result = ffi::NagaSPVWriteResult {
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
    module: *mut ffi::NagaModule,
    module_info: *mut ffi::NagaModuleInfo,
    writer_flags: ffi::NagaWGSLBackWriterFlagsFlags,
    out_result: *mut ffi::NagaWGSLWriteResult,
) -> ffi::NagaBool {
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
                *out_result = ffi::NagaWGSLWriteResult {
                    output: conv::string_to_ffi(&result),
                    ..Default::default()
                };
                1
            }
            Err(error) => {
                let error = if (flags
                    & ffi::NagaWriteResultOption_NagaWriteResultOption_FormattedErrorOnly)
                    != 0
                {
                    ffi::NagaWGSLWriteResult__bindgen_ty_1 {
                        fmt_error: conv::string_to_ffi(&error.to_string()),
                    }
                } else {
                    ffi::NagaWGSLWriteResult__bindgen_ty_1 {
                        error: conv::wgsl_back_error_to_ffi(&error),
                    }
                };
                *out_result = ffi::NagaWGSLWriteResult {
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
    module: *mut ffi::NagaModule,
    module_flags: ffi::NagaModuleFillFlags,
    module_info: *mut ffi::NagaModuleInfo,
    module_fill_flags: ffi::NagaModuleInfoFillFlags,
    entry_point_stage: ffi::NagaShaderStage,
    entry_point_name: *const std::ffi::c_char,
    constants: *mut ffi::NagaPipelineConstant,
    constants_count: u32,
    out_result: *mut ffi::NagaProcessOverridesResult,
) -> ffi::NagaBool {
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
