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

pub fn hlsl_back_shader_model_to_naga(
    model: ffi::HLSLBackShaderModel,
) -> naga::back::hlsl::ShaderModel {
    match model {
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V5_0 => naga::back::hlsl::ShaderModel::V5_0,
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V5_1 => naga::back::hlsl::ShaderModel::V5_1,
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_0 => naga::back::hlsl::ShaderModel::V6_0,
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_1 => naga::back::hlsl::ShaderModel::V6_1,
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_2 => naga::back::hlsl::ShaderModel::V6_2,
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_3 => naga::back::hlsl::ShaderModel::V6_3,
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_4 => naga::back::hlsl::ShaderModel::V6_4,
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_5 => naga::back::hlsl::ShaderModel::V6_5,
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_6 => naga::back::hlsl::ShaderModel::V6_6,
        ffi::HLSLBackShaderModel_HLSLBackShaderModel_V6_7 => naga::back::hlsl::ShaderModel::V6_7,
        _ => panic!("Unknown HLSLBackShaderModel"),
    }
}

pub fn hlsl_back_bind_target_to_naga(
    target: &ffi::HLSLBackBindTarget,
) -> naga::back::hlsl::BindTarget {
    naga::back::hlsl::BindTarget {
        space: target.space,
        register: target.register_,
        binding_array_size: if bool_to_naga(target.binding_array_size.some) {
            Some(target.binding_array_size.value)
        } else {
            None
        },
        dynamic_storage_buffer_offsets_index: if bool_to_naga(
            target.dynamic_storage_buffer_offsets_index.some,
        ) {
            Some(target.dynamic_storage_buffer_offsets_index.value)
        } else {
            None
        },
        restrict_indexing: bool_to_naga(target.restrict_indexing),
    }
}

pub fn hlsl_back_binding_map_to_naga(
    map: &ffi::HLSLBackBindingMap,
) -> naga::back::hlsl::BindingMap {
    unsafe {
        std::slice::from_raw_parts(map.entries, map.entries_len)
            .iter()
            .map(|entry| {
                (
                    resource_binding_to_naga(&entry.key),
                    hlsl_back_bind_target_to_naga(&entry.value),
                )
            })
            .collect()
    }
}

pub fn hlsl_back_sampler_heap_bind_targets_to_naga(
    targets: &ffi::HLSLBackSamplerHeapBindTargets,
) -> naga::back::hlsl::SamplerHeapBindTargets {
    naga::back::hlsl::SamplerHeapBindTargets {
        standard_samplers: hlsl_back_bind_target_to_naga(&targets.standard_samplers),
        comparison_samplers: hlsl_back_bind_target_to_naga(&targets.comparison_samplers),
    }
}

pub fn hlsl_back_sampler_index_buffer_key_to_naga(
    key: &ffi::HLSLBackSamplerIndexBufferKey,
) -> naga::back::hlsl::SamplerIndexBufferKey {
    naga::back::hlsl::SamplerIndexBufferKey { group: key.group }
}

pub fn hlsl_back_sampler_index_buffer_binding_map_to_naga(
    map: &ffi::HLSLBackSamplerIndexBufferBindingMap,
) -> naga::back::hlsl::SamplerIndexBufferBindingMap {
    unsafe {
        std::slice::from_raw_parts(map.entries, map.entries_len)
            .iter()
            .map(|entry| {
                (
                    hlsl_back_sampler_index_buffer_key_to_naga(&entry.key),
                    hlsl_back_bind_target_to_naga(&entry.value),
                )
            })
            .collect()
    }
}

pub fn hlsl_back_offsets_bind_target_to_naga(
    target: &ffi::HLSLBackOffsetsBindTarget,
) -> naga::back::hlsl::OffsetsBindTarget {
    naga::back::hlsl::OffsetsBindTarget {
        space: target.space,
        register: target.register_,
        size: target.size,
    }
}

pub fn hlsl_back_dynamic_storage_buffer_offsets_targets_to_naga(
    targets: &ffi::HLSLBackDynamicStorageBufferOffsetsTargets,
) -> naga::back::hlsl::DynamicStorageBufferOffsetsTargets {
    unsafe {
        std::slice::from_raw_parts(targets.entries, targets.entries_len)
            .iter()
            .map(|entry| {
                (
                    entry.key,
                    hlsl_back_offsets_bind_target_to_naga(&entry.value),
                )
            })
            .collect()
    }
}

pub fn hlsl_back_external_texture_bind_target_to_naga(
    target: &ffi::HLSLBackExternalTextureBindTarget,
) -> naga::back::hlsl::ExternalTextureBindTarget {
    naga::back::hlsl::ExternalTextureBindTarget {
        planes: [
            hlsl_back_bind_target_to_naga(&target.planes[0]),
            hlsl_back_bind_target_to_naga(&target.planes[1]),
            hlsl_back_bind_target_to_naga(&target.planes[2]),
        ],
        params: hlsl_back_bind_target_to_naga(&target.params),
    }
}

pub fn hlsl_back_external_texture_binding_map_to_naga(
    map: &ffi::HLSLBackExternalTextureBindingMap,
) -> naga::back::hlsl::ExternalTextureBindingMap {
    unsafe {
        std::slice::from_raw_parts(map.entries, map.entries_len)
            .iter()
            .map(|entry| {
                (
                    resource_binding_to_naga(&entry.key),
                    hlsl_back_external_texture_bind_target_to_naga(&entry.value),
                )
            })
            .collect()
    }
}

pub fn hlsl_back_options_to_naga(options: &ffi::HLSLBackOptions) -> naga::back::hlsl::Options {
    naga::back::hlsl::Options {
        shader_model: hlsl_back_shader_model_to_naga(options.shader_model),
        binding_map: hlsl_back_binding_map_to_naga(&options.binding_map),
        fake_missing_bindings: bool_to_naga(options.fake_missing_bindings),
        special_constants_binding: if bool_to_naga(options.special_constants_binding.some) {
            Some(hlsl_back_bind_target_to_naga(
                &options.special_constants_binding.value,
            ))
        } else {
            None
        },
        immediates_target: if bool_to_naga(options.immediates_target.some) {
            Some(hlsl_back_bind_target_to_naga(
                &options.immediates_target.value,
            ))
        } else {
            None
        },
        sampler_heap_target: hlsl_back_sampler_heap_bind_targets_to_naga(
            &options.sampler_heap_target,
        ),
        sampler_buffer_binding_map: hlsl_back_sampler_index_buffer_binding_map_to_naga(
            &options.sampler_buffer_binding_map,
        ),
        dynamic_storage_buffer_offsets_targets:
            hlsl_back_dynamic_storage_buffer_offsets_targets_to_naga(
                &options.dynamic_storage_buffer_offsets_targets,
            ),
        external_texture_binding_map: hlsl_back_external_texture_binding_map_to_naga(
            &options.external_texture_binding_map,
        ),
        zero_initialize_workgroup_memory: bool_to_naga(options.zero_initialize_workgroup_memory),
        restrict_indexing: bool_to_naga(options.restrict_indexing),
        force_loop_bounding: bool_to_naga(options.force_loop_bounding),
    }
}

pub fn hlsl_back_pipeline_options_to_naga(
    options: &ffi::HLSLBackPipelineOptions,
) -> naga::back::hlsl::PipelineOptions {
    naga::back::hlsl::PipelineOptions {
        entry_point: if bool_to_naga(options.entry_point.some) {
            Some((
                shader_stage_to_naga(&options.entry_point.value.shader_stage),
                string_to_naga(options.entry_point.value.string),
            ))
        } else {
            None
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

// We need a `to_naga` implementation of this
//
// fn hlsl_back_fragment_entry_point_to_ffi(
//     entry_point: &naga::back::hlsl::FragmentEntryPoint,
// ) -> ffi::HLSLBackFragmentEntryPoint {
//     ffi::HLSLBackFragmentEntryPoint {
//         module: EMPTY_MUT,
//         func: EMPTY_MUT,
//     }
// }
