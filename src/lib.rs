pub mod conv;
pub mod ffi;

use naga::{back, front, valid};
use static_assertions as sa;
use std::mem::ManuallyDrop;

#[cfg(feature = "glsl-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_glsl_parse(
    options: ffi::GLSLFrontOptions,
    source: *const ::std::os::raw::c_char,
    fill_flags: ffi::ModuleFillFlags,
) -> ffi::GLSLFrontendResult {
    let mut frontend = naga::front::glsl::Frontend::default();
    let options = conv::glsl_front_options_to_naga(&options);
    let source = conv::string_to_naga(source);
    match frontend.parse(&options, &source) {
        Ok(module) => ffi::GLSLFrontendResult {
            success: 1,
            module: conv::module_to_ffi(module, fill_flags),
            ..Default::default()
        },
        Err(error) => ffi::GLSLFrontendResult {
            success: 0,
            errors: conv::glsl_front_parse_errors_to_ffi(&error),
            ..Default::default()
        },
    }
}

#[cfg(feature = "spv-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_spv_parse(
    options: ffi::SPVFrontOptions,
    source: *mut u32,
    source_length: u32,
    fill_flags: ffi::ModuleFillFlags,
) -> ffi::SPVFrontendResult {
    let options = conv::spv_front_options_to_naga(&options);
    let spv_slice = unsafe { std::slice::from_raw_parts(source, source_length as usize) };
    let frontend = naga::front::spv::Frontend::new(spv_slice.iter().cloned(), &options);
    match frontend.parse() {
        Ok(module) => ffi::SPVFrontendResult {
            success: 1,
            module: conv::module_to_ffi(module, fill_flags),
            ..Default::default()
        },
        Err(error) => ffi::SPVFrontendResult {
            success: 0,
            error: conv::spv_front_error_to_ffi(&error),
            ..Default::default()
        },
    }
}

#[cfg(feature = "wgsl-in")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_front_wgsl_parse(
    options: ffi::WGSLFrontOptions,
    source: *const ::std::os::raw::c_char,
    fill_flags: ffi::ModuleFillFlags,
) -> ffi::WGSLFrontendResult {
    let options = conv::wgsl_front_options_to_naga(&options);
    let mut frontend = naga::front::wgsl::Frontend::new_with_options(options);
    let source = conv::string_to_naga(source);
    match frontend.parse(&source) {
        Ok(module) => ffi::WGSLFrontendResult {
            success: 1,
            module: conv::module_to_ffi(module, fill_flags),
            ..Default::default()
        },
        Err(error) => ffi::WGSLFrontendResult {
            success: 0,
            error: conv::wgsl_front_parse_error_to_ffi(&error),
            ..Default::default()
        },
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_valid_validator_new(
    flags: ffi::ValidationFlags,
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
) -> ffi::ValidateResult {
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
    match validator.validate(&module) {
        Ok(module_info) => ffi::ValidateResult {
            module_info: unsafe { conv::module_info_to_ffi(module_info) },
            ..Default::default()
        },
        Err(_error) => ffi::ValidateResult {
            error: conv::EMPTY_MUT,
            ..Default::default()
        },
    }
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
) -> ffi::DOTWriteResult {
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

    match result {
        Ok(result) => ffi::DOTWriteResult {
            output: unsafe { conv::string_to_ffi(&result) },
            ..Default::default()
        },
        Err(error) => ffi::DOTWriteResult {
            error: unsafe { conv::string_to_ffi(&error.to_string()) },
            ..Default::default()
        },
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
) -> ffi::GLSLWriteResult {
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
    let mut writer = match writer {
        Ok(writer) => writer,
        Err(error) => {
            return ffi::GLSLWriteResult {
                error: conv::glsl_back_error_to_ffi(&error),
                ..Default::default()
            };
        }
    };
    match writer.write() {
        Ok(_reflection_info) => ffi::GLSLWriteResult {
            output: unsafe { conv::string_to_ffi(&buf) },
            ..Default::default()
        },
        Err(error) => ffi::GLSLWriteResult {
            error: conv::glsl_back_error_to_ffi(&error),
            ..Default::default()
        },
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
) -> ffi::HLSLWriteResult {
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
    match writer.write(&module, &module_info, None) {
        Ok(_reflection_info) => ffi::HLSLWriteResult {
            output: unsafe { conv::string_to_ffi(&buf) },
            ..Default::default()
        },
        Err(error) => ffi::HLSLWriteResult {
            error: conv::hlsl_back_error_to_ffi(&error),
            ..Default::default()
        },
    }
}

#[cfg(feature = "msl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_msl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::MSLBackOptions,
    pipeline_options: ffi::MSLBackPipelineOptions,
) -> ffi::MSLWriteResult {
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

    match naga::back::msl::write_string(&module, &module_info, &options, &pipeline_options) {
        Ok((result, _translation_info)) => ffi::MSLWriteResult {
            output: unsafe { conv::string_to_ffi(&result) },
            ..Default::default()
        },
        Err(error) => ffi::MSLWriteResult {
            error: conv::msl_back_error_to_ffi(&error),
            ..Default::default()
        },
    }
}

#[cfg(feature = "spv-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_spv_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    options: ffi::SPVBackOptions,
    pipeline_options: *mut ffi::SPVBackPipelineOptions,
) -> ffi::SPVWriteResult {
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

    match naga::back::spv::write_vec(&module, &module_info, &options, pipeline_options.as_ref()) {
        Ok(result) => ffi::SPVWriteResult {
            output: unsafe { conv::slice_to_ffi(&result, |v| *v) },
            output_count: result.len() as u32,
            ..Default::default()
        },
        Err(error) => ffi::SPVWriteResult {
            error: conv::spv_back_error_to_ffi(&error),
            ..Default::default()
        },
    }
}

#[cfg(feature = "wgsl-out")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn naga_back_wgsl_write(
    module: *mut ffi::Module,
    module_info: *mut ffi::ModuleInfo,
    writer_flags: ffi::WGSLBackWriterFlags,
) -> ffi::WGSLWriteResult {
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

    match naga::back::wgsl::write_string(&module, &module_info, writer_flags) {
        Ok(result) => ffi::WGSLWriteResult {
            output: unsafe { conv::string_to_ffi(&result) },
            ..Default::default()
        },
        Err(error) => ffi::WGSLWriteResult {
            error: conv::wgsl_back_error_to_ffi(&error),
            ..Default::default()
        },
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
) -> ffi::ProcessOverridesResult {
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
