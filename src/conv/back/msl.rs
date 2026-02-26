use super::*;

fn msl_back_sampler_address_to_naga(
    address: ffi::NagaMSLBackSamplerAddress,
) -> naga::back::msl::sampler::Address {
    match address {
        ffi::NagaMSLBackSamplerAddress_NagaMSLBackSamplerAddress_Repeat => {
            naga::back::msl::sampler::Address::Repeat
        }
        ffi::NagaMSLBackSamplerAddress_NagaMSLBackSamplerAddress_MirroredRepeat => {
            naga::back::msl::sampler::Address::MirroredRepeat
        }
        ffi::NagaMSLBackSamplerAddress_NagaMSLBackSamplerAddress_ClampToEdge => {
            naga::back::msl::sampler::Address::ClampToEdge
        }
        ffi::NagaMSLBackSamplerAddress_NagaMSLBackSamplerAddress_ClampToZero => {
            naga::back::msl::sampler::Address::ClampToZero
        }
        ffi::NagaMSLBackSamplerAddress_NagaMSLBackSamplerAddress_ClampToBorder => {
            naga::back::msl::sampler::Address::ClampToBorder
        }
        _ => panic!("Unknown MSLBackSamplerAddress"),
    }
}

fn msl_back_sampler_border_color_to_naga(
    color: ffi::NagaMSLBackSamplerBorderColor,
) -> naga::back::msl::sampler::BorderColor {
    match color {
        ffi::NagaMSLBackSamplerBorderColor_NagaMSLBackSamplerBorderColor_TransparentBlack => {
            naga::back::msl::sampler::BorderColor::TransparentBlack
        }
        ffi::NagaMSLBackSamplerBorderColor_NagaMSLBackSamplerBorderColor_OpaqueBlack => {
            naga::back::msl::sampler::BorderColor::OpaqueBlack
        }
        ffi::NagaMSLBackSamplerBorderColor_NagaMSLBackSamplerBorderColor_OpaqueWhite => {
            naga::back::msl::sampler::BorderColor::OpaqueWhite
        }
        _ => panic!("Unknown MSLBackSamplerBorderColor"),
    }
}

fn msl_back_sampler_compare_func_to_naga(
    func: ffi::NagaMSLBackSamplerCompareFunc,
) -> naga::back::msl::sampler::CompareFunc {
    match func {
        ffi::NagaMSLBackSamplerCompareFunc_NagaMSLBackSamplerCompareFunc_Never => {
            naga::back::msl::sampler::CompareFunc::Never
        }
        ffi::NagaMSLBackSamplerCompareFunc_NagaMSLBackSamplerCompareFunc_Less => {
            naga::back::msl::sampler::CompareFunc::Less
        }
        ffi::NagaMSLBackSamplerCompareFunc_NagaMSLBackSamplerCompareFunc_LessEqual => {
            naga::back::msl::sampler::CompareFunc::LessEqual
        }
        ffi::NagaMSLBackSamplerCompareFunc_NagaMSLBackSamplerCompareFunc_Greater => {
            naga::back::msl::sampler::CompareFunc::Greater
        }
        ffi::NagaMSLBackSamplerCompareFunc_NagaMSLBackSamplerCompareFunc_GreaterEqual => {
            naga::back::msl::sampler::CompareFunc::GreaterEqual
        }
        ffi::NagaMSLBackSamplerCompareFunc_NagaMSLBackSamplerCompareFunc_Equal => {
            naga::back::msl::sampler::CompareFunc::Equal
        }
        ffi::NagaMSLBackSamplerCompareFunc_NagaMSLBackSamplerCompareFunc_NotEqual => {
            naga::back::msl::sampler::CompareFunc::NotEqual
        }
        ffi::NagaMSLBackSamplerCompareFunc_NagaMSLBackSamplerCompareFunc_Always => {
            naga::back::msl::sampler::CompareFunc::Always
        }
        _ => panic!("Unknown MSLBackSamplerCompareFunc"),
    }
}

fn msl_back_sampler_coord_to_naga(
    coord: ffi::NagaMSLBackSamplerCoord,
) -> naga::back::msl::sampler::Coord {
    match coord {
        ffi::NagaMSLBackSamplerCoord_NagaMSLBackSamplerCoord_Normalized => {
            naga::back::msl::sampler::Coord::Normalized
        }
        ffi::NagaMSLBackSamplerCoord_NagaMSLBackSamplerCoord_Pixel => {
            naga::back::msl::sampler::Coord::Pixel
        }
        _ => panic!("Unknown MSLBackSamplerCoord"),
    }
}

fn msl_back_sampler_filter_to_naga(
    filter: ffi::NagaMSLBackSamplerFilter,
) -> naga::back::msl::sampler::Filter {
    match filter {
        ffi::NagaMSLBackSamplerFilter_NagaMSLBackSamplerFilter_Nearest => {
            naga::back::msl::sampler::Filter::Nearest
        }
        ffi::NagaMSLBackSamplerFilter_NagaMSLBackSamplerFilter_Linear => {
            naga::back::msl::sampler::Filter::Linear
        }
        _ => panic!("Unknown MSLBackSamplerFilter"),
    }
}

fn msl_back_inline_sampler_to_naga(
    sampler: &ffi::NagaMSLBackInlineSampler,
) -> naga::back::msl::sampler::InlineSampler {
    naga::back::msl::sampler::InlineSampler {
        coord: msl_back_sampler_coord_to_naga(sampler.coord),
        address: [
            msl_back_sampler_address_to_naga(sampler.address[0]),
            msl_back_sampler_address_to_naga(sampler.address[1]),
            msl_back_sampler_address_to_naga(sampler.address[2]),
        ],
        border_color: msl_back_sampler_border_color_to_naga(sampler.border_color),
        mag_filter: msl_back_sampler_filter_to_naga(sampler.mag_filter),
        min_filter: msl_back_sampler_filter_to_naga(sampler.min_filter),
        mip_filter: if bool_to_naga(sampler.mip_filter.some) {
            Some(msl_back_sampler_filter_to_naga(sampler.mip_filter.value))
        } else {
            None
        },
        lod_clamp: if bool_to_naga(sampler.lod_clamp.some) {
            Some(std::ops::Range {
                start: sampler.lod_clamp.value.start,
                end: sampler.lod_clamp.value.end,
            })
        } else {
            None
        },
        max_anisotropy: if bool_to_naga(sampler.max_anisotropy.some) {
            std::num::NonZeroU32::new(sampler.max_anisotropy.value)
        } else {
            None
        },
        compare_func: msl_back_sampler_compare_func_to_naga(sampler.compare_func),
    }
}

pub fn msl_back_vertex_format_to_ffi(
    format: &naga::back::msl::VertexFormat,
) -> ffi::NagaMSLBackVertexFormat {
    match format {
        naga::back::msl::VertexFormat::Uint8 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint8
        }
        naga::back::msl::VertexFormat::Uint8x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint8x2
        }
        naga::back::msl::VertexFormat::Uint8x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint8x4
        }
        naga::back::msl::VertexFormat::Sint8 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint8
        }
        naga::back::msl::VertexFormat::Sint8x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint8x2
        }
        naga::back::msl::VertexFormat::Sint8x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint8x4
        }
        naga::back::msl::VertexFormat::Unorm8 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm8
        }
        naga::back::msl::VertexFormat::Unorm8x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm8x2
        }
        naga::back::msl::VertexFormat::Unorm8x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm8x4
        }
        naga::back::msl::VertexFormat::Snorm8 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm8
        }
        naga::back::msl::VertexFormat::Snorm8x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm8x2
        }
        naga::back::msl::VertexFormat::Snorm8x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm8x4
        }
        naga::back::msl::VertexFormat::Uint16 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint16
        }
        naga::back::msl::VertexFormat::Uint16x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint16x2
        }
        naga::back::msl::VertexFormat::Uint16x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint16x4
        }
        naga::back::msl::VertexFormat::Sint16 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint16
        }
        naga::back::msl::VertexFormat::Sint16x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint16x2
        }
        naga::back::msl::VertexFormat::Sint16x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint16x4
        }
        naga::back::msl::VertexFormat::Unorm16 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm16
        }
        naga::back::msl::VertexFormat::Unorm16x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm16x2
        }
        naga::back::msl::VertexFormat::Unorm16x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm16x4
        }
        naga::back::msl::VertexFormat::Snorm16 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm16
        }
        naga::back::msl::VertexFormat::Snorm16x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm16x2
        }
        naga::back::msl::VertexFormat::Snorm16x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm16x4
        }
        naga::back::msl::VertexFormat::Float16 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float16
        }
        naga::back::msl::VertexFormat::Float16x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float16x2
        }
        naga::back::msl::VertexFormat::Float16x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float16x4
        }
        naga::back::msl::VertexFormat::Float32 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float32
        }
        naga::back::msl::VertexFormat::Float32x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float32x2
        }
        naga::back::msl::VertexFormat::Float32x3 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float32x3
        }
        naga::back::msl::VertexFormat::Float32x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float32x4
        }
        naga::back::msl::VertexFormat::Uint32 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint32
        }
        naga::back::msl::VertexFormat::Uint32x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint32x2
        }
        naga::back::msl::VertexFormat::Uint32x3 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint32x3
        }
        naga::back::msl::VertexFormat::Uint32x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint32x4
        }
        naga::back::msl::VertexFormat::Sint32 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint32
        }
        naga::back::msl::VertexFormat::Sint32x2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint32x2
        }
        naga::back::msl::VertexFormat::Sint32x3 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint32x3
        }
        naga::back::msl::VertexFormat::Sint32x4 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint32x4
        }
        naga::back::msl::VertexFormat::Unorm10_10_10_2 => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm10_10_10_2
        }
        naga::back::msl::VertexFormat::Unorm8x4Bgra => {
            ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm8x4Bgra
        }
    }
}

pub fn msl_back_bind_external_texture_target_to_naga(
    target: &ffi::NagaMSLBackBindExternalTextureTarget,
) -> naga::back::msl::BindExternalTextureTarget {
    naga::back::msl::BindExternalTextureTarget {
        planes: target.planes,
        params: target.params,
    }
}

pub fn msl_back_vertex_buffer_step_mode_to_ffi(
    mode: &naga::back::msl::VertexBufferStepMode,
) -> ffi::NagaMSLBackVertexBufferStepMode {
    match mode {
        naga::back::msl::VertexBufferStepMode::Constant => {
            ffi::NagaMSLBackVertexBufferStepMode_NagaMSLBackVertexBufferStepMode_Constant
        }
        naga::back::msl::VertexBufferStepMode::ByVertex => {
            ffi::NagaMSLBackVertexBufferStepMode_NagaMSLBackVertexBufferStepMode_ByVertex
        }
        naga::back::msl::VertexBufferStepMode::ByInstance => {
            ffi::NagaMSLBackVertexBufferStepMode_NagaMSLBackVertexBufferStepMode_ByInstance
        }
    }
}

pub fn msl_back_bind_sampler_target_to_naga(
    target: &ffi::NagaMSLBackBindSamplerTarget,
) -> naga::back::msl::BindSamplerTarget {
    match target.tag {
        ffi::NagaMSLBackBindSamplerTargetTag_NagaMSLBackBindSamplerTargetTag_Resource => {
            naga::back::msl::BindSamplerTarget::Resource(unsafe { target.data.resource })
        }
        ffi::NagaMSLBackBindSamplerTargetTag_NagaMSLBackBindSamplerTargetTag_Inline => {
            naga::back::msl::BindSamplerTarget::Inline(unsafe { target.data.inline_ })
        }
        _ => panic!("Unknown MSLBackBindSamplerTarget"),
    }
}

pub fn msl_back_attribute_mapping_to_ffi(
    mapping: &naga::back::msl::AttributeMapping,
) -> ffi::NagaMSLBackAttributeMapping {
    ffi::NagaMSLBackAttributeMapping {
        shader_location: mapping.shader_location,
        offset: mapping.offset,
        format: msl_back_vertex_format_to_ffi(&mapping.format),
    }
}

pub fn msl_back_bind_target_to_naga(
    target: &ffi::NagaMSLBackBindTarget,
) -> naga::back::msl::BindTarget {
    naga::back::msl::BindTarget {
        buffer: if bool_to_naga(target.buffer.some) {
            Some(target.buffer.value)
        } else {
            None
        },
        texture: if bool_to_naga(target.texture.some) {
            Some(target.texture.value)
        } else {
            None
        },
        sampler: if bool_to_naga(target.sampler.some) {
            Some(msl_back_bind_sampler_target_to_naga(&target.sampler.value))
        } else {
            None
        },
        external_texture: if bool_to_naga(target.external_texture.some) {
            Some(msl_back_bind_external_texture_target_to_naga(
                &target.external_texture.value,
            ))
        } else {
            None
        },
        mutable: bool_to_naga(target.mutable_),
    }
}

pub fn msl_back_bind_external_texture_target_to_ffi(
    target: &naga::back::msl::BindExternalTextureTarget,
) -> ffi::NagaMSLBackBindExternalTextureTarget {
    ffi::NagaMSLBackBindExternalTextureTarget {
        planes: target.planes,
        params: target.params,
    }
}

pub fn msl_back_binding_map_to_naga(
    map: &ffi::NagaMSLBackBindingMap,
) -> naga::back::msl::BindingMap {
    unsafe {
        std::slice::from_raw_parts(map.entries, map.entries_len)
            .iter()
            .map(|entry| {
                (
                    resource_binding_to_naga(&entry.key),
                    msl_back_bind_target_to_naga(&entry.value),
                )
            })
            .collect()
    }
}

pub fn msl_back_bind_sampler_target_to_ffi(
    target: &naga::back::msl::BindSamplerTarget,
) -> ffi::NagaMSLBackBindSamplerTarget {
    match target {
        naga::back::msl::BindSamplerTarget::Resource(slot) => ffi::NagaMSLBackBindSamplerTarget {
            tag: ffi::NagaMSLBackBindSamplerTargetTag_NagaMSLBackBindSamplerTargetTag_Resource,
            data: ffi::NagaMSLBackBindSamplerTarget__bindgen_ty_1 { resource: *slot },
        },
        naga::back::msl::BindSamplerTarget::Inline(inline) => ffi::NagaMSLBackBindSamplerTarget {
            tag: ffi::NagaMSLBackBindSamplerTargetTag_NagaMSLBackBindSamplerTargetTag_Inline,
            data: ffi::NagaMSLBackBindSamplerTarget__bindgen_ty_1 { inline_: *inline },
        },
    }
}

pub fn msl_back_entry_point_resources_to_naga(
    resources: &ffi::NagaMSLBackEntryPointResources,
) -> naga::back::msl::EntryPointResources {
    naga::back::msl::EntryPointResources {
        resources: msl_back_binding_map_to_naga(&resources.resources),
        immediates_buffer: if bool_to_naga(resources.immediates_buffer.some) {
            Some(resources.immediates_buffer.value)
        } else {
            None
        },
        sizes_buffer: if bool_to_naga(resources.sizes_buffer.some) {
            Some(resources.sizes_buffer.value)
        } else {
            None
        },
    }
}

pub fn msl_back_entry_point_resource_map_to_naga(
    map: &ffi::NagaMSLBackEntryPointResourceMap,
) -> naga::back::msl::EntryPointResourceMap {
    unsafe {
        std::slice::from_raw_parts(map.entries, map.entries_len)
            .iter()
            .map(|entry| {
                (
                    string_to_naga(entry.key),
                    msl_back_entry_point_resources_to_naga(&entry.value),
                )
            })
            .collect()
    }
}

pub fn msl_back_bind_target_to_ffi(
    target: &naga::back::msl::BindTarget,
) -> ffi::NagaMSLBackBindTarget {
    ffi::NagaMSLBackBindTarget {
        buffer: ffi::NagaMSLBackBindTarget__bindgen_ty_1 {
            some: bool_to_ffi(target.buffer.is_some()),
            value: target.buffer.unwrap_or_default(),
        },
        texture: ffi::NagaMSLBackBindTarget__bindgen_ty_2 {
            some: bool_to_ffi(target.texture.is_some()),
            value: target.texture.unwrap_or_default(),
        },
        sampler: ffi::NagaMSLBackBindTarget__bindgen_ty_3 {
            some: bool_to_ffi(target.sampler.is_some()),
            value: target
                .sampler
                .as_ref()
                .map(msl_back_bind_sampler_target_to_ffi)
                .unwrap_or_default(),
        },
        external_texture: ffi::NagaMSLBackBindTarget__bindgen_ty_4 {
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

pub fn msl_back_options_to_naga(options: &ffi::NagaMSLBackOptions) -> naga::back::msl::Options {
    naga::back::msl::Options {
        lang_version: (options.lang_version[0], options.lang_version[1]),
        per_entry_point_map: msl_back_entry_point_resource_map_to_naga(
            &options.per_entry_point_map,
        ),
        inline_samplers: unsafe {
            std::slice::from_raw_parts(options.inline_samplers, options.inline_samplers_len)
                .iter()
                .map(msl_back_inline_sampler_to_naga)
                .collect()
        },
        spirv_cross_compatibility: bool_to_naga(options.spirv_cross_compatibility),
        fake_missing_bindings: bool_to_naga(options.fake_missing_bindings),
        bounds_check_policies: bound_check_policies_to_naga(&options.bounds_check_policies),
        zero_initialize_workgroup_memory: bool_to_naga(options.zero_initialize_workgroup_memory),
        force_loop_bounding: bool_to_naga(options.force_loop_bounding),
    }
}

pub fn msl_back_vertex_buffer_mapping_to_ffi(
    mapping: &naga::back::msl::VertexBufferMapping,
) -> ffi::NagaMSLBackVertexBufferMapping {
    ffi::NagaMSLBackVertexBufferMapping {
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

pub fn msl_back_vertex_buffer_step_mode_to_naga(
    mode: ffi::NagaMSLBackVertexBufferStepMode,
) -> naga::back::msl::VertexBufferStepMode {
    match mode {
        ffi::NagaMSLBackVertexBufferStepMode_NagaMSLBackVertexBufferStepMode_Constant => {
            naga::back::msl::VertexBufferStepMode::Constant
        }
        ffi::NagaMSLBackVertexBufferStepMode_NagaMSLBackVertexBufferStepMode_ByVertex => {
            naga::back::msl::VertexBufferStepMode::ByVertex
        }
        ffi::NagaMSLBackVertexBufferStepMode_NagaMSLBackVertexBufferStepMode_ByInstance => {
            naga::back::msl::VertexBufferStepMode::ByInstance
        }
        _ => panic!("Unknown MSLBackVertexBufferStepMode"),
    }
}

pub fn msl_back_attribute_mapping_to_naga(
    mapping: &ffi::NagaMSLBackAttributeMapping,
) -> naga::back::msl::AttributeMapping {
    naga::back::msl::AttributeMapping {
        shader_location: mapping.shader_location,
        offset: mapping.offset,
        format: msl_back_vertex_format_to_naga(mapping.format),
    }
}

pub fn msl_back_vertex_format_to_naga(
    format: ffi::NagaMSLBackVertexFormat,
) -> naga::back::msl::VertexFormat {
    match format {
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint8 => {
            naga::back::msl::VertexFormat::Uint8
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint8x2 => {
            naga::back::msl::VertexFormat::Uint8x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint8x4 => {
            naga::back::msl::VertexFormat::Uint8x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint8 => {
            naga::back::msl::VertexFormat::Sint8
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint8x2 => {
            naga::back::msl::VertexFormat::Sint8x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint8x4 => {
            naga::back::msl::VertexFormat::Sint8x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm8 => {
            naga::back::msl::VertexFormat::Unorm8
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm8x2 => {
            naga::back::msl::VertexFormat::Unorm8x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm8x4 => {
            naga::back::msl::VertexFormat::Unorm8x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm8 => {
            naga::back::msl::VertexFormat::Snorm8
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm8x2 => {
            naga::back::msl::VertexFormat::Snorm8x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm8x4 => {
            naga::back::msl::VertexFormat::Snorm8x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint16 => {
            naga::back::msl::VertexFormat::Uint16
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint16x2 => {
            naga::back::msl::VertexFormat::Uint16x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint16x4 => {
            naga::back::msl::VertexFormat::Uint16x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint16 => {
            naga::back::msl::VertexFormat::Sint16
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint16x2 => {
            naga::back::msl::VertexFormat::Sint16x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint16x4 => {
            naga::back::msl::VertexFormat::Sint16x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm16 => {
            naga::back::msl::VertexFormat::Unorm16
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm16x2 => {
            naga::back::msl::VertexFormat::Unorm16x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm16x4 => {
            naga::back::msl::VertexFormat::Unorm16x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm16 => {
            naga::back::msl::VertexFormat::Snorm16
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm16x2 => {
            naga::back::msl::VertexFormat::Snorm16x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Snorm16x4 => {
            naga::back::msl::VertexFormat::Snorm16x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float16 => {
            naga::back::msl::VertexFormat::Float16
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float16x2 => {
            naga::back::msl::VertexFormat::Float16x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float16x4 => {
            naga::back::msl::VertexFormat::Float16x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float32 => {
            naga::back::msl::VertexFormat::Float32
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float32x2 => {
            naga::back::msl::VertexFormat::Float32x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float32x3 => {
            naga::back::msl::VertexFormat::Float32x3
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Float32x4 => {
            naga::back::msl::VertexFormat::Float32x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint32 => {
            naga::back::msl::VertexFormat::Uint32
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint32x2 => {
            naga::back::msl::VertexFormat::Uint32x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint32x3 => {
            naga::back::msl::VertexFormat::Uint32x3
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Uint32x4 => {
            naga::back::msl::VertexFormat::Uint32x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint32 => {
            naga::back::msl::VertexFormat::Sint32
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint32x2 => {
            naga::back::msl::VertexFormat::Sint32x2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint32x3 => {
            naga::back::msl::VertexFormat::Sint32x3
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Sint32x4 => {
            naga::back::msl::VertexFormat::Sint32x4
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm10_10_10_2 => {
            naga::back::msl::VertexFormat::Unorm10_10_10_2
        }
        ffi::NagaMSLBackVertexFormat_NagaMSLBackVertexFormat_Unorm8x4Bgra => {
            naga::back::msl::VertexFormat::Unorm8x4Bgra
        }
        _ => panic!("Unknown MSLBackVertexFormat"),
    }
}

pub fn msl_back_vertex_buffer_mapping_to_naga(
    mapping: &ffi::NagaMSLBackVertexBufferMapping,
) -> naga::back::msl::VertexBufferMapping {
    naga::back::msl::VertexBufferMapping {
        id: mapping.id,
        stride: mapping.stride,
        step_mode: msl_back_vertex_buffer_step_mode_to_naga(mapping.step_mode),
        attributes: unsafe {
            std::slice::from_raw_parts(mapping.attributes, mapping.attributes_len)
                .iter()
                .map(msl_back_attribute_mapping_to_naga)
                .collect()
        },
    }
}

pub fn msl_back_pipeline_options_to_naga(
    options: &ffi::NagaMSLBackPipelineOptions,
) -> naga::back::msl::PipelineOptions {
    naga::back::msl::PipelineOptions {
        entry_point: if bool_to_naga(options.entry_point.some) {
            Some((
                shader_stage_to_naga(&options.entry_point.value.shader_stage),
                unsafe { string_to_naga(options.entry_point.value.string) },
            ))
        } else {
            None
        },
        allow_and_force_point_size: bool_to_naga(options.allow_and_force_point_size),
        vertex_pulling_transform: bool_to_naga(options.vertex_pulling_transform),
        vertex_buffer_mappings: unsafe {
            std::slice::from_raw_parts(
                options.vertex_buffer_mappings,
                options.vertex_buffer_mappings_len,
            )
            .iter()
            .map(msl_back_vertex_buffer_mapping_to_naga)
            .collect()
        },
    }
}

pub fn msl_back_entry_point_error_to_ffi(
    error: &naga::back::msl::EntryPointError,
) -> ffi::NagaMSLBackEntryPointError {
    let default_data = ffi::NagaMSLBackEntryPointError__bindgen_ty_1::default();

    match error {
        naga::back::msl::EntryPointError::MissingBinding(binding) => ffi::NagaMSLBackEntryPointError {
            tag: ffi::NagaMSLBackEntryPointErrorTag_NagaMSLBackEntryPointErrorTag_MissingBinding,
            data: ffi::NagaMSLBackEntryPointError__bindgen_ty_1 {
                missing_binding: unsafe { string_to_ffi(binding) },
            },
        },
        naga::back::msl::EntryPointError::MissingBindTarget(resource_binding) => {
            ffi::NagaMSLBackEntryPointError {
                tag: ffi::NagaMSLBackEntryPointErrorTag_NagaMSLBackEntryPointErrorTag_MissingBindTarget,
                data: ffi::NagaMSLBackEntryPointError__bindgen_ty_1 {
                    missing_bind_target: resource_binding_to_ffi(resource_binding),
                },
            }
        }
        naga::back::msl::EntryPointError::MissingImmediateData => ffi::NagaMSLBackEntryPointError {
            tag: ffi::NagaMSLBackEntryPointErrorTag_NagaMSLBackEntryPointErrorTag_MissingImmediateData,
            data: default_data,
        },
        naga::back::msl::EntryPointError::MissingSizesBuffer => ffi::NagaMSLBackEntryPointError {
            tag: ffi::NagaMSLBackEntryPointErrorTag_NagaMSLBackEntryPointErrorTag_MissingSizesBuffer,
            data: default_data,
        },
    }
}

pub fn msl_back_error_to_ffi(error: &naga::back::msl::Error) -> ffi::NagaMSLBackError {
    let default_data = ffi::NagaMSLBackError__bindgen_ty_1 {
        format: std::ptr::null_mut(),
    };

    match error {
        naga::back::msl::Error::Format(error) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_Format,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                format: unsafe { string_to_ffi(&error.to_string()) },
            },
        },
        naga::back::msl::Error::UnimplementedBindTarget(bind_target) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnimplementedBindTarget,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unimplemented_bind_target: msl_back_bind_target_to_ffi(bind_target),
            },
        },
        naga::back::msl::Error::UnsupportedCompose(handle) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedCompose,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unsupported_compose: handle.index(),
            },
        },
        naga::back::msl::Error::UnsupportedBinaryOp(_binary_operator) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedBinaryOp,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unsupported_binary_op: EMPTY_MUT,
            },
        },
        naga::back::msl::Error::UnsupportedCall(call) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedCall,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unsupported_call: unsafe { string_to_ffi(call) },
            },
        },
        naga::back::msl::Error::FeatureNotImplemented(feature) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_FeatureNotImplemented,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                feature_not_implemented: unsafe { string_to_ffi(feature) },
            },
        },
        naga::back::msl::Error::GenericValidation(validation) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_GenericValidation,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                generic_validation: unsafe { string_to_ffi(validation) },
            },
        },
        naga::back::msl::Error::UnsupportedBuiltIn(built_in) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedBuiltIn,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unsupported_built_in: built_in_to_ffi(built_in),
            },
        },
        naga::back::msl::Error::CapabilityNotSupported(capabilities) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_CapabilityNotSupported,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                capability_not_supported: capabilities_to_ffi(capabilities),
            },
        },
        naga::back::msl::Error::UnsupportedAttribute(attribute) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedAttribute,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unsupported_attribute: unsafe { string_to_ffi(attribute) },
            },
        },
        naga::back::msl::Error::UnsupportedFunction(function) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedFunction,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unsupported_function: unsafe { string_to_ffi(function) },
            },
        },
        naga::back::msl::Error::UnsupportedWriteableStorageBuffer => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedWriteableStorageBuffer,
            data: default_data,
        },
        naga::back::msl::Error::UnsupportedWriteableStorageTexture(shader_stage) => {
            ffi::NagaMSLBackError {
                tag:
                    ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedWriteableStorageTexture,
                data: ffi::NagaMSLBackError__bindgen_ty_1 {
                    unsupported_writeable_storage_texture: shader_stage_to_ffi(shader_stage),
                },
            }
        }
        naga::back::msl::Error::UnsupportedRWStorageTexture => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedRWStorageTexture,
            data: default_data,
        },
        naga::back::msl::Error::UnsupportedArrayOf(array_of) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedArrayOf,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unsupported_array_of: unsafe { string_to_ffi(array_of) },
            },
        },
        naga::back::msl::Error::UnsupportedArrayOfType(handle) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedArrayOfType,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unsupported_array_of_type: handle.index(),
            },
        },
        naga::back::msl::Error::UnsupportedRayTracing => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedRayTracing,
            data: default_data,
        },
        naga::back::msl::Error::Override => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_Override,
            data: default_data,
        },
        naga::back::msl::Error::UnsupportedBitCast(type_inner) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_UnsupportedBitCast,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                unsupported_bit_cast: type_inner_to_ffi(type_inner),
            },
        },
        naga::back::msl::Error::ResolveArraySizeError(resolve_array_size_error) => {
            ffi::NagaMSLBackError {
                tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_ResolveArraySizeError,
                data: ffi::NagaMSLBackError__bindgen_ty_1 {
                    resolve_array_size_error: resolve_array_size_error_to_ffi(
                        resolve_array_size_error,
                    ),
                },
            }
        }
        naga::back::msl::Error::EntryPointNotFound(shader_stage, s) => ffi::NagaMSLBackError {
            tag: ffi::NagaMSLBackErrorTag_NagaMSLBackErrorTag_EntryPointNotFound,
            data: ffi::NagaMSLBackError__bindgen_ty_1 {
                entry_point_not_found: ffi::NagaShaderStageString {
                    shader_stage: shader_stage_to_ffi(shader_stage),
                    string: unsafe { string_to_ffi(s) },
                },
            },
        },
    }
}

pub fn msl_back_translation_info_to_ffi(
    _info: &naga::back::msl::TranslationInfo,
) -> ffi::NagaMSLBackTranslationInfo {
    ffi::NagaMSLBackTranslationInfo::default()
}
