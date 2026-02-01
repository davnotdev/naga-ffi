use super::*;

fn msl_back_sampler_address_to_ffi(
    address: &naga::back::msl::sampler::Address,
) -> ffi::MSLBackSamplerAddress {
    match address {
        naga::back::msl::sampler::Address::Repeat => {
            ffi::MSLBackSamplerAddress_MSLBackSamplerAddress_Repeat
        }
        naga::back::msl::sampler::Address::MirroredRepeat => {
            ffi::MSLBackSamplerAddress_MSLBackSamplerAddress_MirroredRepeat
        }
        naga::back::msl::sampler::Address::ClampToEdge => {
            ffi::MSLBackSamplerAddress_MSLBackSamplerAddress_ClampToEdge
        }
        naga::back::msl::sampler::Address::ClampToZero => {
            ffi::MSLBackSamplerAddress_MSLBackSamplerAddress_ClampToZero
        }
        naga::back::msl::sampler::Address::ClampToBorder => {
            ffi::MSLBackSamplerAddress_MSLBackSamplerAddress_ClampToBorder
        }
    }
}

fn msl_back_sampler_border_color_to_ffi(
    color: &naga::back::msl::sampler::BorderColor,
) -> ffi::MSLBackSamplerBorderColor {
    match color {
        naga::back::msl::sampler::BorderColor::TransparentBlack => {
            ffi::MSLBackSamplerBorderColor_MSLBackSamplerBorderColor_TransparentBlack
        }
        naga::back::msl::sampler::BorderColor::OpaqueBlack => {
            ffi::MSLBackSamplerBorderColor_MSLBackSamplerBorderColor_OpaqueBlack
        }
        naga::back::msl::sampler::BorderColor::OpaqueWhite => {
            ffi::MSLBackSamplerBorderColor_MSLBackSamplerBorderColor_OpaqueWhite
        }
    }
}

fn msl_back_sampler_compare_func_to_ffi(
    func: &naga::back::msl::sampler::CompareFunc,
) -> ffi::MSLBackSamplerCompareFunc {
    match func {
        naga::back::msl::sampler::CompareFunc::Never => {
            ffi::MSLBackSamplerCompareFunc_MSLBackSamplerCompareFunc_Never
        }
        naga::back::msl::sampler::CompareFunc::Less => {
            ffi::MSLBackSamplerCompareFunc_MSLBackSamplerCompareFunc_Less
        }
        naga::back::msl::sampler::CompareFunc::LessEqual => {
            ffi::MSLBackSamplerCompareFunc_MSLBackSamplerCompareFunc_LessEqual
        }
        naga::back::msl::sampler::CompareFunc::Greater => {
            ffi::MSLBackSamplerCompareFunc_MSLBackSamplerCompareFunc_Greater
        }
        naga::back::msl::sampler::CompareFunc::GreaterEqual => {
            ffi::MSLBackSamplerCompareFunc_MSLBackSamplerCompareFunc_GreaterEqual
        }
        naga::back::msl::sampler::CompareFunc::Equal => {
            ffi::MSLBackSamplerCompareFunc_MSLBackSamplerCompareFunc_Equal
        }
        naga::back::msl::sampler::CompareFunc::NotEqual => {
            ffi::MSLBackSamplerCompareFunc_MSLBackSamplerCompareFunc_NotEqual
        }
        naga::back::msl::sampler::CompareFunc::Always => {
            ffi::MSLBackSamplerCompareFunc_MSLBackSamplerCompareFunc_Always
        }
    }
}

fn msl_back_sampler_coord_to_ffi(
    coord: &naga::back::msl::sampler::Coord,
) -> ffi::MSLBackSamplerCoord {
    match coord {
        naga::back::msl::sampler::Coord::Normalized => {
            ffi::MSLBackSamplerCoord_MSLBackSamplerCoord_Normalized
        }
        naga::back::msl::sampler::Coord::Pixel => {
            ffi::MSLBackSamplerCoord_MSLBackSamplerCoord_Pixel
        }
    }
}

fn msl_back_sampler_filter_to_ffi(
    filter: &naga::back::msl::sampler::Filter,
) -> ffi::MSLBackSamplerFilter {
    match filter {
        naga::back::msl::sampler::Filter::Nearest => {
            ffi::MSLBackSamplerFilter_MSLBackSamplerFilter_Nearest
        }
        naga::back::msl::sampler::Filter::Linear => {
            ffi::MSLBackSamplerFilter_MSLBackSamplerFilter_Linear
        }
    }
}

fn msl_back_inline_sampler_to_ffi(
    sampler: &naga::back::msl::sampler::InlineSampler,
) -> ffi::MSLBackInlineSampler {
    ffi::MSLBackInlineSampler {
        coord: msl_back_sampler_coord_to_ffi(&sampler.coord),
        address: [
            msl_back_sampler_address_to_ffi(&sampler.address[0]),
            msl_back_sampler_address_to_ffi(&sampler.address[1]),
            msl_back_sampler_address_to_ffi(&sampler.address[2]),
        ],
        border_color: msl_back_sampler_border_color_to_ffi(&sampler.border_color),
        mag_filter: msl_back_sampler_filter_to_ffi(&sampler.mag_filter),
        min_filter: msl_back_sampler_filter_to_ffi(&sampler.min_filter),
        mip_filter: ffi::MSLBackInlineSampler__bindgen_ty_1 {
            some: bool_to_ffi(sampler.mip_filter.is_some()),
            value: sampler
                .mip_filter
                .as_ref()
                .map(msl_back_sampler_filter_to_ffi)
                .unwrap_or_default(),
        },
        lod_clamp: ffi::MSLBackInlineSampler__bindgen_ty_2 {
            some: bool_to_ffi(sampler.lod_clamp.is_some()),
            value: sampler
                .lod_clamp
                .as_ref()
                .map(|range| ffi::MSLBackFloatRange {
                    start: range.start,
                    end: range.end,
                })
                .unwrap_or_default(),
        },
        max_anisotropy: ffi::MSLBackInlineSampler__bindgen_ty_3 {
            some: bool_to_ffi(sampler.max_anisotropy.is_some()),
            value: sampler.max_anisotropy.map(|n| n.get()).unwrap_or_default(),
        },
        compare_func: msl_back_sampler_compare_func_to_ffi(&sampler.compare_func),
    }
}

pub fn msl_back_vertex_format_to_ffi(
    format: &naga::back::msl::VertexFormat,
) -> ffi::MSLBackVertexFormat {
    match format {
        naga::back::msl::VertexFormat::Uint8 => ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint8,
        naga::back::msl::VertexFormat::Uint8x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint8x2
        }
        naga::back::msl::VertexFormat::Uint8x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint8x4
        }
        naga::back::msl::VertexFormat::Sint8 => ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint8,
        naga::back::msl::VertexFormat::Sint8x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint8x2
        }
        naga::back::msl::VertexFormat::Sint8x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint8x4
        }
        naga::back::msl::VertexFormat::Unorm8 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Unorm8
        }
        naga::back::msl::VertexFormat::Unorm8x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Unorm8x2
        }
        naga::back::msl::VertexFormat::Unorm8x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Unorm8x4
        }
        naga::back::msl::VertexFormat::Snorm8 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Snorm8
        }
        naga::back::msl::VertexFormat::Snorm8x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Snorm8x2
        }
        naga::back::msl::VertexFormat::Snorm8x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Snorm8x4
        }
        naga::back::msl::VertexFormat::Uint16 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint16
        }
        naga::back::msl::VertexFormat::Uint16x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint16x2
        }
        naga::back::msl::VertexFormat::Uint16x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint16x4
        }
        naga::back::msl::VertexFormat::Sint16 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint16
        }
        naga::back::msl::VertexFormat::Sint16x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint16x2
        }
        naga::back::msl::VertexFormat::Sint16x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint16x4
        }
        naga::back::msl::VertexFormat::Unorm16 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Unorm16
        }
        naga::back::msl::VertexFormat::Unorm16x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Unorm16x2
        }
        naga::back::msl::VertexFormat::Unorm16x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Unorm16x4
        }
        naga::back::msl::VertexFormat::Snorm16 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Snorm16
        }
        naga::back::msl::VertexFormat::Snorm16x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Snorm16x2
        }
        naga::back::msl::VertexFormat::Snorm16x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Snorm16x4
        }
        naga::back::msl::VertexFormat::Float16 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Float16
        }
        naga::back::msl::VertexFormat::Float16x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Float16x2
        }
        naga::back::msl::VertexFormat::Float16x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Float16x4
        }
        naga::back::msl::VertexFormat::Float32 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Float32
        }
        naga::back::msl::VertexFormat::Float32x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Float32x2
        }
        naga::back::msl::VertexFormat::Float32x3 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Float32x3
        }
        naga::back::msl::VertexFormat::Float32x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Float32x4
        }
        naga::back::msl::VertexFormat::Uint32 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint32
        }
        naga::back::msl::VertexFormat::Uint32x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint32x2
        }
        naga::back::msl::VertexFormat::Uint32x3 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint32x3
        }
        naga::back::msl::VertexFormat::Uint32x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Uint32x4
        }
        naga::back::msl::VertexFormat::Sint32 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint32
        }
        naga::back::msl::VertexFormat::Sint32x2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint32x2
        }
        naga::back::msl::VertexFormat::Sint32x3 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint32x3
        }
        naga::back::msl::VertexFormat::Sint32x4 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Sint32x4
        }
        naga::back::msl::VertexFormat::Unorm10_10_10_2 => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Unorm10_10_10_2
        }
        naga::back::msl::VertexFormat::Unorm8x4Bgra => {
            ffi::MSLBackVertexFormat_MSLBackVertexFormat_Unorm8x4Bgra
        }
    }
}

pub fn msl_back_vertex_buffer_step_mode_to_ffi(
    mode: &naga::back::msl::VertexBufferStepMode,
) -> ffi::MSLBackVertexBufferStepMode {
    match mode {
        naga::back::msl::VertexBufferStepMode::Constant => {
            ffi::MSLBackVertexBufferStepMode_MSLBackVertexBufferStepMode_Constant
        }
        naga::back::msl::VertexBufferStepMode::ByVertex => {
            ffi::MSLBackVertexBufferStepMode_MSLBackVertexBufferStepMode_ByVertex
        }
        naga::back::msl::VertexBufferStepMode::ByInstance => {
            ffi::MSLBackVertexBufferStepMode_MSLBackVertexBufferStepMode_ByInstance
        }
    }
}

pub fn msl_back_attribute_mapping_to_ffi(
    mapping: &naga::back::msl::AttributeMapping,
) -> ffi::MSLBackAttributeMapping {
    ffi::MSLBackAttributeMapping {
        shader_location: mapping.shader_location,
        offset: mapping.offset,
        format: msl_back_vertex_format_to_ffi(&mapping.format),
    }
}

pub fn msl_back_bind_external_texture_target_to_ffi(
    target: &naga::back::msl::BindExternalTextureTarget,
) -> ffi::MSLBackBindExternalTextureTarget {
    ffi::MSLBackBindExternalTextureTarget {
        planes: target.planes,
        params: target.params,
    }
}

pub fn msl_back_bind_sampler_target_to_ffi(
    target: &naga::back::msl::BindSamplerTarget,
) -> ffi::MSLBackBindSamplerTarget {
    match target {
        naga::back::msl::BindSamplerTarget::Resource(slot) => ffi::MSLBackBindSamplerTarget {
            tag: ffi::MSLBackBindSamplerTargetTag_MSLBackBindSamplerTargetTag_Resource,
            data: ffi::MSLBackBindSamplerTarget__bindgen_ty_1 { resource: *slot },
        },
        naga::back::msl::BindSamplerTarget::Inline(inline) => ffi::MSLBackBindSamplerTarget {
            tag: ffi::MSLBackBindSamplerTargetTag_MSLBackBindSamplerTargetTag_Inline,
            data: ffi::MSLBackBindSamplerTarget__bindgen_ty_1 { inline_: *inline },
        },
    }
}

pub fn msl_back_bind_target_to_ffi(target: &naga::back::msl::BindTarget) -> ffi::MSLBackBindTarget {
    ffi::MSLBackBindTarget {
        buffer: ffi::MSLBackBindTarget__bindgen_ty_1 {
            some: bool_to_ffi(target.buffer.is_some()),
            value: target.buffer.unwrap_or_default(),
        },
        texture: ffi::MSLBackBindTarget__bindgen_ty_2 {
            some: bool_to_ffi(target.texture.is_some()),
            value: target.texture.unwrap_or_default(),
        },
        sampler: ffi::MSLBackBindTarget__bindgen_ty_3 {
            some: bool_to_ffi(target.sampler.is_some()),
            value: target
                .sampler
                .as_ref()
                .map(msl_back_bind_sampler_target_to_ffi)
                .unwrap_or_default(),
        },
        external_texture: ffi::MSLBackBindTarget__bindgen_ty_4 {
            some: bool_to_ffi(target.external_texture.is_some()),
            value: target
                .external_texture
                .as_ref()
                .map(msl_back_bind_external_texture_target_to_ffi)
                .unwrap_or_default(),
        },
        mutable_: bool_to_ffi(target.mutable),
    }
}

pub fn msl_back_binding_map_to_ffi(map: &naga::back::msl::BindingMap) -> ffi::MSLBackBindingMap {
    let entries = map.iter().collect::<Vec<_>>();
    ffi::MSLBackBindingMap {
        entries: unsafe {
            slice_to_ffi(&entries, |(key, value)| ffi::MSLBackBindingMapEntry {
                key: resource_binding_to_ffi(key),
                value: msl_back_bind_target_to_ffi(value),
            })
        },
        entries_len: entries.len(),
    }
}

pub fn msl_back_entry_point_resources_to_ffi(
    resources: &naga::back::msl::EntryPointResources,
) -> ffi::MSLBackEntryPointResources {
    ffi::MSLBackEntryPointResources {
        resources: msl_back_binding_map_to_ffi(&resources.resources),
        immediates_buffer: ffi::MSLBackEntryPointResources__bindgen_ty_1 {
            some: bool_to_ffi(resources.immediates_buffer.is_some()),
            value: resources.immediates_buffer.unwrap_or_default(),
        },
        sizes_buffer: ffi::MSLBackEntryPointResources__bindgen_ty_2 {
            some: bool_to_ffi(resources.sizes_buffer.is_some()),
            value: resources.sizes_buffer.unwrap_or_default(),
        },
    }
}

pub fn msl_back_entry_point_resource_map_to_ffi(
    map: &naga::back::msl::EntryPointResourceMap,
) -> ffi::MSLBackEntryPointResourceMap {
    let entries = map.iter().collect::<Vec<_>>();
    ffi::MSLBackEntryPointResourceMap {
        entries: unsafe {
            slice_to_ffi(&entries, |(key, value)| {
                ffi::MSLBackEntryPointResourceMapEntry {
                    key: string_to_ffi(key),
                    value: msl_back_entry_point_resources_to_ffi(value),
                }
            })
        },
        entries_len: entries.len(),
    }
}

pub fn msl_back_options_to_ffi(options: &naga::back::msl::Options) -> ffi::MSLBackOptions {
    ffi::MSLBackOptions {
        lang_version: [options.lang_version.0, options.lang_version.1],
        per_entry_point_map: msl_back_entry_point_resource_map_to_ffi(&options.per_entry_point_map),
        inline_samplers: unsafe {
            slice_to_ffi(&options.inline_samplers, msl_back_inline_sampler_to_ffi)
        },
        inline_samplers_len: options.inline_samplers.len(),
        spirv_cross_compatibility: bool_to_ffi(options.spirv_cross_compatibility),
        fake_missing_bindings: bool_to_ffi(options.fake_missing_bindings),
        bounds_check_policies: bound_check_policies_to_ffi(&options.bounds_check_policies),
        zero_initialize_workgroup_memory: bool_to_ffi(options.zero_initialize_workgroup_memory),
        force_loop_bounding: bool_to_ffi(options.force_loop_bounding),
    }
}

pub fn msl_back_vertex_buffer_mapping_to_ffi(
    mapping: &naga::back::msl::VertexBufferMapping,
) -> ffi::MSLBackVertexBufferMapping {
    ffi::MSLBackVertexBufferMapping {
        id: mapping.id,
        stride: mapping.stride,
        step_mode: msl_back_vertex_buffer_step_mode_to_ffi(&mapping.step_mode),
        attributes: unsafe {
            slice_to_ffi(&mapping.attributes, |mapping| {
                msl_back_attribute_mapping_to_ffi(mapping)
            })
        },
        attributes_len: mapping.attributes.len(),
    }
}

pub fn msl_back_pipeline_options_to_ffi(
    options: &naga::back::msl::PipelineOptions,
) -> ffi::MSLBackPipelineOptions {
    ffi::MSLBackPipelineOptions {
        entry_point: ffi::MSLBackPipelineOptions__bindgen_ty_1 {
            some: bool_to_ffi(options.entry_point.is_some()),
            value: options
                .entry_point
                .as_ref()
                .map(|(shader_stage, s)| ffi::MSLBackShaderStageString {
                    shader_stage: shader_stage_to_ffi(shader_stage),
                    string: unsafe { string_to_ffi(s) },
                })
                .unwrap_or_default(),
        },
        allow_and_force_point_size: bool_to_ffi(options.allow_and_force_point_size),
        vertex_pulling_transform: bool_to_ffi(options.vertex_pulling_transform),
        vertex_buffer_mappings: unsafe {
            slice_to_ffi(
                &options.vertex_buffer_mappings,
                msl_back_vertex_buffer_mapping_to_ffi,
            )
        },
        vertex_buffer_mappings_len: options.vertex_buffer_mappings.len(),
    }
}

pub fn msl_back_entry_point_error_to_ffi(
    error: &naga::back::msl::EntryPointError,
) -> ffi::MSLBackEntryPointError {
    let default_data = ffi::MSLBackEntryPointError__bindgen_ty_1::default();

    match error {
        naga::back::msl::EntryPointError::MissingBinding(binding) => ffi::MSLBackEntryPointError {
            tag: ffi::MSLBackEntryPointErrorTag_MSLBackEntryPointErrorTag_MissingBinding,
            data: ffi::MSLBackEntryPointError__bindgen_ty_1 {
                missing_binding: unsafe { string_to_ffi(binding) },
            },
        },
        naga::back::msl::EntryPointError::MissingBindTarget(resource_binding) => {
            ffi::MSLBackEntryPointError {
                tag: ffi::MSLBackEntryPointErrorTag_MSLBackEntryPointErrorTag_MissingBindTarget,
                data: ffi::MSLBackEntryPointError__bindgen_ty_1 {
                    missing_bind_target: resource_binding_to_ffi(resource_binding),
                },
            }
        }
        naga::back::msl::EntryPointError::MissingImmediateData => ffi::MSLBackEntryPointError {
            tag: ffi::MSLBackEntryPointErrorTag_MSLBackEntryPointErrorTag_MissingImmediateData,
            data: default_data,
        },
        naga::back::msl::EntryPointError::MissingSizesBuffer => ffi::MSLBackEntryPointError {
            tag: ffi::MSLBackEntryPointErrorTag_MSLBackEntryPointErrorTag_MissingSizesBuffer,
            data: default_data,
        },
    }
}

pub fn msl_back_error_to_ffi(error: &naga::back::msl::Error) -> ffi::MSLBackError {
    let default_data = ffi::MSLBackError__bindgen_ty_1 {
        format: std::ptr::null_mut(),
    };

    match error {
        naga::back::msl::Error::Format(error) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_Format,
            data: ffi::MSLBackError__bindgen_ty_1 {
                format: unsafe { string_to_ffi(&error.to_string()) },
            },
        },
        naga::back::msl::Error::UnimplementedBindTarget(bind_target) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnimplementedBindTarget,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unimplemented_bind_target: msl_back_bind_target_to_ffi(bind_target),
            },
        },
        naga::back::msl::Error::UnsupportedCompose(handle) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedCompose,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unsupported_compose: handle.index(),
            },
        },
        naga::back::msl::Error::UnsupportedBinaryOp(_binary_operator) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedBinaryOp,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unsupported_binary_op: EMPTY_MUT,
            },
        },
        naga::back::msl::Error::UnsupportedCall(call) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedCall,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unsupported_call: unsafe { string_to_ffi(call) },
            },
        },
        naga::back::msl::Error::FeatureNotImplemented(feature) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_FeatureNotImplemented,
            data: ffi::MSLBackError__bindgen_ty_1 {
                feature_not_implemented: unsafe { string_to_ffi(feature) },
            },
        },
        naga::back::msl::Error::GenericValidation(validation) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_GenericValidation,
            data: ffi::MSLBackError__bindgen_ty_1 {
                generic_validation: unsafe { string_to_ffi(validation) },
            },
        },
        naga::back::msl::Error::UnsupportedBuiltIn(built_in) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedBuiltIn,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unsupported_built_in: built_in_to_ffi(built_in),
            },
        },
        naga::back::msl::Error::CapabilityNotSupported(capabilities) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_CapabilityNotSupported,
            data: ffi::MSLBackError__bindgen_ty_1 {
                capability_not_supported: capabilities_to_ffi(capabilities),
            },
        },
        naga::back::msl::Error::UnsupportedAttribute(attribute) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedAttribute,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unsupported_attribute: unsafe { string_to_ffi(attribute) },
            },
        },
        naga::back::msl::Error::UnsupportedFunction(function) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedFunction,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unsupported_function: unsafe { string_to_ffi(function) },
            },
        },
        naga::back::msl::Error::UnsupportedWriteableStorageBuffer => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedWriteableStorageBuffer,
            data: default_data,
        },
        naga::back::msl::Error::UnsupportedWriteableStorageTexture(shader_stage) => {
            ffi::MSLBackError {
                tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedWriteableStorageTexture,
                data: ffi::MSLBackError__bindgen_ty_1 {
                    unsupported_writeable_storage_texture: shader_stage_to_ffi(shader_stage),
                },
            }
        }
        naga::back::msl::Error::UnsupportedRWStorageTexture => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedRWStorageTexture,
            data: default_data,
        },
        naga::back::msl::Error::UnsupportedArrayOf(array_of) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedArrayOf,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unsupported_array_of: unsafe { string_to_ffi(array_of) },
            },
        },
        naga::back::msl::Error::UnsupportedArrayOfType(handle) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedArrayOfType,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unsupported_array_of_type: handle.index(),
            },
        },
        naga::back::msl::Error::UnsupportedRayTracing => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedRayTracing,
            data: default_data,
        },
        naga::back::msl::Error::Override => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_Override,
            data: default_data,
        },
        naga::back::msl::Error::UnsupportedBitCast(type_inner) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_UnsupportedBitCast,
            data: ffi::MSLBackError__bindgen_ty_1 {
                unsupported_bit_cast: type_inner_to_ffi(type_inner),
            },
        },
        naga::back::msl::Error::ResolveArraySizeError(resolve_array_size_error) => {
            ffi::MSLBackError {
                tag: ffi::MSLBackErrorTag_MSLBackErrorTag_ResolveArraySizeError,
                data: ffi::MSLBackError__bindgen_ty_1 {
                    resolve_array_size_error: resolve_array_size_error_to_ffi(
                        resolve_array_size_error,
                    ),
                },
            }
        }
        naga::back::msl::Error::EntryPointNotFound(shader_stage, s) => ffi::MSLBackError {
            tag: ffi::MSLBackErrorTag_MSLBackErrorTag_EntryPointNotFound,
            data: ffi::MSLBackError__bindgen_ty_1 {
                entry_point_not_found: ffi::MSLBackShaderStageString {
                    shader_stage: shader_stage_to_ffi(shader_stage),
                    string: unsafe { string_to_ffi(s) },
                },
            },
        },
    }
}

pub fn msl_back_translation_info_to_ffi(
    _info: &naga::back::msl::TranslationInfo,
) -> ffi::MSLBackTranslationInfo {
    ffi::MSLBackTranslationInfo::default()
}
