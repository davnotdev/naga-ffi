use super::*;

pub fn hlsl_back_shader_model_to_ffi(
    model: &naga::back::hlsl::ShaderModel,
) -> ffi::HLSLBackShaderModel {
    match model {
        naga::back::hlsl::ShaderModel::V5_0 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V5_0,
        naga::back::hlsl::ShaderModel::V5_1 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V5_1,
        naga::back::hlsl::ShaderModel::V6_0 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_0,
        naga::back::hlsl::ShaderModel::V6_1 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_1,
        naga::back::hlsl::ShaderModel::V6_2 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_2,
        naga::back::hlsl::ShaderModel::V6_3 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_3,
        naga::back::hlsl::ShaderModel::V6_4 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_4,
        naga::back::hlsl::ShaderModel::V6_5 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_5,
        naga::back::hlsl::ShaderModel::V6_6 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_6,
        naga::back::hlsl::ShaderModel::V6_7 => ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_7,
    }
}

pub fn hlsl_back_bind_target_to_ffi(
    target: &naga::back::hlsl::BindTarget,
) -> ffi::HLSLBackBindTarget {
    ffi::HLSLBackBindTarget {
        space: target.space,
        register_: target.register,
        binding_array_size: ffi::HLSLBackBindTarget__bindgen_ty_1 {
            some: bool_to_ffi(target.binding_array_size.is_some()),
            value: target.binding_array_size.unwrap_or_default(),
        },
        dynamic_storage_buffer_offsets_index: ffi::HLSLBackBindTarget__bindgen_ty_2 {
            some: bool_to_ffi(target.dynamic_storage_buffer_offsets_index.is_some()),
            value: target
                .dynamic_storage_buffer_offsets_index
                .unwrap_or_default(),
        },
        restrict_indexing: bool_to_ffi(target.restrict_indexing),
    }
}

pub fn hlsl_back_binding_map_to_ffi(map: &naga::back::hlsl::BindingMap) -> ffi::HLSLBackBindingMap {
    ffi::HLSLBackBindingMap {
        entries: unsafe {
            slice_to_ffi(&map.iter().collect::<Vec<_>>(), |&(k, v)| {
                ffi::HLSLBackBindingMapEntry {
                    key: resource_binding_to_ffi(k),
                    value: hlsl_back_bind_target_to_ffi(v),
                }
            })
        },
        entries_len: map.len(),
    }
}

pub fn hlsl_back_sampler_heap_bind_targets_to_ffi(
    targets: &naga::back::hlsl::SamplerHeapBindTargets,
) -> ffi::HLSLBackSamplerHeapBindTargets {
    ffi::HLSLBackSamplerHeapBindTargets {
        standard_samplers: hlsl_back_bind_target_to_ffi(&targets.standard_samplers),
        comparison_samplers: hlsl_back_bind_target_to_ffi(&targets.comparison_samplers),
    }
}

pub fn hlsl_back_sampler_index_buffer_key_to_ffi(
    key: &naga::back::hlsl::SamplerIndexBufferKey,
) -> ffi::HLSLBackSamplerIndexBufferKey {
    ffi::HLSLBackSamplerIndexBufferKey { group: key.group }
}

pub fn hlsl_back_sampler_index_buffer_binding_map_to_ffi(
    map: &naga::back::hlsl::SamplerIndexBufferBindingMap,
) -> ffi::HLSLBackSamplerIndexBufferBindingMap {
    ffi::HLSLBackSamplerIndexBufferBindingMap {
        entries: unsafe {
            slice_to_ffi(&map.iter().collect::<Vec<_>>(), |&(k, v)| {
                ffi::HLSLBackSamplerIndexBufferBindingMapEntry {
                    key: hlsl_back_sampler_index_buffer_key_to_ffi(k),
                    value: hlsl_back_bind_target_to_ffi(v),
                }
            })
        },
        entries_len: map.len(),
    }
}

pub fn hlsl_back_offsets_bind_target_to_ffi(
    target: &naga::back::hlsl::OffsetsBindTarget,
) -> ffi::HLSLBackOffsetsBindTarget {
    ffi::HLSLBackOffsetsBindTarget {
        space: target.space,
        register_: target.register,
        size: target.size,
    }
}

pub fn hlsl_back_dynamic_storage_buffer_offsets_targets_to_ffi(
    targets: &naga::back::hlsl::DynamicStorageBufferOffsetsTargets,
) -> ffi::HLSLBackDynamicStorageBufferOffsetsTargets {
    ffi::HLSLBackDynamicStorageBufferOffsetsTargets {
        entries: unsafe {
            slice_to_ffi(&targets.iter().collect::<Vec<_>>(), |&(k, v)| {
                ffi::HLSLBackDynamicStorageBufferOffsetsTargetsEntry {
                    key: *k,
                    value: hlsl_back_offsets_bind_target_to_ffi(v),
                }
            })
        },
        entries_len: targets.len(),
    }
}

pub fn hlsl_back_external_texture_bind_target(
    target: naga::back::hlsl::ExternalTextureBindTarget,
) -> ffi::HLSLBackExternalTextureBindTarget {
    ffi::HLSLBackExternalTextureBindTarget {
        planes: [
            hlsl_back_bind_target_to_ffi(&target.planes[0]),
            hlsl_back_bind_target_to_ffi(&target.planes[1]),
            hlsl_back_bind_target_to_ffi(&target.planes[2]),
        ],
        params: hlsl_back_bind_target_to_ffi(&target.params),
    }
}

pub fn hlsl_back_external_texture_binding_map_to_ffi(
    map: &naga::back::hlsl::ExternalTextureBindingMap,
) -> ffi::HLSLBackExternalTextureBindingMap {
    ffi::HLSLBackExternalTextureBindingMap {
        entries: unsafe {
            slice_to_ffi(&map.iter().collect::<Vec<_>>(), |&(k, v)| {
                ffi::HLSLBackExternalTextureBindingMapEntry {
                    key: resource_binding_to_ffi(k),
                    value: hlsl_back_external_texture_bind_target(*v),
                }
            })
        },
        entries_len: map.len(),
    }
}

pub fn hlsl_back_options_to_ffi(options: &naga::back::hlsl::Options) -> ffi::HLSLBackOptions {
    ffi::HLSLBackOptions {
        shader_model: hlsl_back_shader_model_to_ffi(&options.shader_model),
        binding_map: hlsl_back_binding_map_to_ffi(&options.binding_map),
        fake_missing_bindings: bool_to_ffi(options.fake_missing_bindings),
        special_constants_binding: ffi::HLSLBackOptions__bindgen_ty_1 {
            some: bool_to_ffi(options.special_constants_binding.is_some()),
            value: options
                .special_constants_binding
                .as_ref()
                .map(hlsl_back_bind_target_to_ffi)
                .unwrap_or_default(),
        },
        immediates_target: ffi::HLSLBackOptions__bindgen_ty_2 {
            some: bool_to_ffi(options.immediates_target.is_some()),
            value: options
                .immediates_target
                .as_ref()
                .map(hlsl_back_bind_target_to_ffi)
                .unwrap_or_default(),
        },
        sampler_heap_target: hlsl_back_sampler_heap_bind_targets_to_ffi(
            &options.sampler_heap_target,
        ),
        sampler_buffer_binding_map: hlsl_back_sampler_index_buffer_binding_map_to_ffi(
            &options.sampler_buffer_binding_map,
        ),
        dynamic_storage_buffer_offsets_targets:
            hlsl_back_dynamic_storage_buffer_offsets_targets_to_ffi(
                &options.dynamic_storage_buffer_offsets_targets,
            ),
        external_texture_binding_map: hlsl_back_external_texture_binding_map_to_ffi(
            &options.external_texture_binding_map,
        ),
        zero_initialize_workgroup_memory: bool_to_ffi(options.zero_initialize_workgroup_memory),
        restrict_indexing: bool_to_ffi(options.restrict_indexing),
        force_loop_bounding: bool_to_ffi(options.force_loop_bounding),
    }
}

pub fn hlsl_back_pipeline_options_to_ffi(
    options: &naga::back::hlsl::PipelineOptions,
) -> ffi::HLSLBackPipelineOptions {
    ffi::HLSLBackPipelineOptions {
        entry_point: ffi::HLSLBackPipelineOptions__bindgen_ty_1 {
            some: bool_to_ffi(options.entry_point.is_some()),
            value: options
                .entry_point
                .as_ref()
                .map(|entry_point| ffi::ShaderStageString {
                    shader_stage: shader_stage_to_ffi(&entry_point.0),
                    string: unsafe { string_to_ffi(&entry_point.1) },
                })
                .unwrap_or_default(),
        },
    }
}

pub fn hlsl_back_error_to_ffi(error: &naga::back::hlsl::Error) -> ffi::HLSLBackError {
    let default_data = ffi::HLSLBackError__bindgen_ty_1 {
        io_error: std::ptr::null_mut(),
    };

    match error {
        naga::back::hlsl::Error::IoError(error) => ffi::HLSLBackError {
            tag: ffi::HLSLBackErrorTag_HLSLBackErrorTag_IoError,
            data: ffi::HLSLBackError__bindgen_ty_1 {
                io_error: unsafe { string_to_ffi(&error.to_string()) },
            },
        },
        naga::back::hlsl::Error::UnsupportedScalar(scalar) => ffi::HLSLBackError {
            tag: ffi::HLSLBackErrorTag_HLSLBackErrorTag_UnsupportedScalar,
            data: ffi::HLSLBackError__bindgen_ty_1 {
                unsupported_scalar: scalar_to_ffi(scalar),
            },
        },
        naga::back::hlsl::Error::Unimplemented(error) => ffi::HLSLBackError {
            tag: ffi::HLSLBackErrorTag_HLSLBackErrorTag_Unimplemented,
            data: ffi::HLSLBackError__bindgen_ty_1 {
                unimplemented: unsafe { string_to_ffi(error) },
            },
        },
        naga::back::hlsl::Error::Custom(error) => ffi::HLSLBackError {
            tag: ffi::HLSLBackErrorTag_HLSLBackErrorTag_Custom,
            data: ffi::HLSLBackError__bindgen_ty_1 {
                custom: unsafe { string_to_ffi(error) },
            },
        },
        naga::back::hlsl::Error::Override => ffi::HLSLBackError {
            tag: ffi::HLSLBackErrorTag_HLSLBackErrorTag_Override,
            data: default_data,
        },
        naga::back::hlsl::Error::ResolveArraySizeError(resolve_array_size_error) => {
            ffi::HLSLBackError {
                tag: ffi::HLSLBackErrorTag_HLSLBackErrorTag_ResolveArraySizeError,
                data: ffi::HLSLBackError__bindgen_ty_1 {
                    resolve_array_size_error: resolve_array_size_error_to_ffi(
                        resolve_array_size_error,
                    ),
                },
            }
        }
        naga::back::hlsl::Error::EntryPointNotFound(shader_stage, name) => ffi::HLSLBackError {
            tag: ffi::HLSLBackErrorTag_HLSLBackErrorTag_EntryPointNotFound,
            data: ffi::HLSLBackError__bindgen_ty_1 {
                entry_point_not_found: ffi::HLSLBackError__bindgen_ty_1__bindgen_ty_1 {
                    stage: shader_stage_to_ffi(shader_stage),
                    name: unsafe { string_to_ffi(name) },
                },
            },
        },
        naga::back::hlsl::Error::ShaderModelTooLow(message, shader_model) => ffi::HLSLBackError {
            tag: ffi::HLSLBackErrorTag_HLSLBackErrorTag_ShaderModelTooLow,
            data: ffi::HLSLBackError__bindgen_ty_1 {
                shader_model_too_low: ffi::HLSLBackError__bindgen_ty_1__bindgen_ty_2 {
                    message: unsafe { string_to_ffi(message) },
                    model: hlsl_back_shader_model_to_ffi(shader_model),
                },
            },
        },
    }
}

fn hlsl_back_fragment_entry_point_to_ffi(
    entry_point: &naga::back::hlsl::FragmentEntryPoint,
) -> ffi::HLSLBackFragmentEntryPoint {
    ffi::HLSLBackFragmentEntryPoint {
        module: EMPTY_MUT,
        func: EMPTY_MUT,
    }
}
