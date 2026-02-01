use super::*;

pub fn glsl_back_version_to_ffi(version: &naga::back::glsl::Version) -> ffi::GLSLBackVersion {
    match version {
        naga::back::glsl::Version::Desktop(version) => ffi::GLSLBackVersion {
            tag: ffi::GLSLBackVersionTag_GLSLBackVersionTag_Desktop,
            data: ffi::GLSLBackVersion__bindgen_ty_1 { desktop: *version },
        },
        naga::back::glsl::Version::Embedded { version, is_webgl } => ffi::GLSLBackVersion {
            tag: ffi::GLSLBackVersionTag_GLSLBackVersionTag_Desktop,
            data: ffi::GLSLBackVersion__bindgen_ty_1 {
                embedded: ffi::GLSLBackVersion__bindgen_ty_1__bindgen_ty_1 {
                    version: *version,
                    is_webgl: bool_to_ffi(*is_webgl),
                },
            },
        },
    }
}

pub fn glsl_back_writer_flags_to_ffi(
    flags: naga::back::glsl::WriterFlags,
) -> ffi::GLSLBackWriterFlags {
    let mut result: ffi::GLSLBackWriterFlags = 0;

    if flags.contains(naga::back::glsl::WriterFlags::ADJUST_COORDINATE_SPACE) {
        result |= ffi::GLSLBackWriterFlags_GLSLBackWriterFlags_ADJUST_COORDINATE_SPACE;
    }
    if flags.contains(naga::back::glsl::WriterFlags::TEXTURE_SHADOW_LOD) {
        result |= ffi::GLSLBackWriterFlags_GLSLBackWriterFlags_TEXTURE_SHADOW_LOD;
    }
    if flags.contains(naga::back::glsl::WriterFlags::DRAW_PARAMETERS) {
        result |= ffi::GLSLBackWriterFlags_GLSLBackWriterFlags_DRAW_PARAMETERS;
    }
    if flags.contains(naga::back::glsl::WriterFlags::INCLUDE_UNUSED_ITEMS) {
        result |= ffi::GLSLBackWriterFlags_GLSLBackWriterFlags_INCLUDE_UNUSED_ITEMS;
    }
    if flags.contains(naga::back::glsl::WriterFlags::FORCE_POINT_SIZE) {
        result |= ffi::GLSLBackWriterFlags_GLSLBackWriterFlags_FORCE_POINT_SIZE;
    }

    sa::const_assert_eq!(
        naga::back::glsl::WriterFlags::all().bits(),
        naga::back::glsl::WriterFlags::ADJUST_COORDINATE_SPACE.bits()
            | naga::back::glsl::WriterFlags::TEXTURE_SHADOW_LOD.bits()
            | naga::back::glsl::WriterFlags::DRAW_PARAMETERS.bits()
            | naga::back::glsl::WriterFlags::INCLUDE_UNUSED_ITEMS.bits()
            | naga::back::glsl::WriterFlags::FORCE_POINT_SIZE.bits()
    );

    result
}

pub fn glsl_back_binding_map_to_ffi(map: &naga::back::glsl::BindingMap) -> ffi::GLSLBackBindingMap {
    ffi::GLSLBackBindingMap {
        entries: unsafe {
            slice_to_ffi(&map.iter().collect::<Vec<_>>(), |(resource, binding)| {
                ffi::GLSLBackBindingMapEntry {
                    key: resource_binding_to_ffi(resource),
                    value: **binding,
                }
            })
        },
        entries_len: map.len(),
    }
}

pub fn glsl_back_options_to_ffi(options: &naga::back::glsl::Options) -> ffi::GLSLBackOptions {
    ffi::GLSLBackOptions {
        version: glsl_back_version_to_ffi(&options.version),
        writer_flags: glsl_back_writer_flags_to_ffi(options.writer_flags),
        binding_map: glsl_back_binding_map_to_ffi(&options.binding_map),
        zero_initialize_workgroup_memory: bool_to_ffi(options.zero_initialize_workgroup_memory),
    }
}

pub fn glsl_back_pipeline_options_to_ffi(
    options: &naga::back::glsl::PipelineOptions,
) -> ffi::GLSLBackPipelineOptions {
    ffi::GLSLBackPipelineOptions {
        shader_stage: shader_stage_to_ffi(&options.shader_stage),
        entry_point: unsafe { string_to_ffi(&options.entry_point) },
        multiview: ffi::GLSLBackPipelineOptions__bindgen_ty_1 {
            some: bool_to_ffi(options.multiview.is_some()),
            value: options
                .multiview
                .map(|multiview| multiview.get())
                .unwrap_or_default(),
        },
    }
}

pub fn glsl_back_features_to_ffi(features: naga::back::glsl::Features) -> ffi::GLSLBackFeatures {
    let mut result: ffi::GLSLBackFeatures = 0;

    if features.contains(naga::back::glsl::Features::BUFFER_STORAGE) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_BUFFER_STORAGE;
    }
    if features.contains(naga::back::glsl::Features::ARRAY_OF_ARRAYS) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_ARRAY_OF_ARRAYS;
    }
    if features.contains(naga::back::glsl::Features::DOUBLE_TYPE) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_DOUBLE_TYPE;
    }
    if features.contains(naga::back::glsl::Features::FULL_IMAGE_FORMATS) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_FULL_IMAGE_FORMATS;
    }
    if features.contains(naga::back::glsl::Features::MULTISAMPLED_TEXTURES) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_MULTISAMPLED_TEXTURES;
    }
    if features.contains(naga::back::glsl::Features::MULTISAMPLED_TEXTURE_ARRAYS) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_MULTISAMPLED_TEXTURE_ARRAYS;
    }
    if features.contains(naga::back::glsl::Features::CUBE_TEXTURES_ARRAY) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_CUBE_TEXTURES_ARRAY;
    }
    if features.contains(naga::back::glsl::Features::COMPUTE_SHADER) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_COMPUTE_SHADER;
    }
    if features.contains(naga::back::glsl::Features::IMAGE_LOAD_STORE) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_IMAGE_LOAD_STORE;
    }
    if features.contains(naga::back::glsl::Features::CONSERVATIVE_DEPTH) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_CONSERVATIVE_DEPTH;
    }
    if features.contains(naga::back::glsl::Features::NOPERSPECTIVE_QUALIFIER) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_NOPERSPECTIVE_QUALIFIER;
    }
    if features.contains(naga::back::glsl::Features::SAMPLE_QUALIFIER) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_SAMPLE_QUALIFIER;
    }
    if features.contains(naga::back::glsl::Features::CLIP_DISTANCE) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_CLIP_DISTANCE;
    }
    if features.contains(naga::back::glsl::Features::CULL_DISTANCE) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_CULL_DISTANCE;
    }
    if features.contains(naga::back::glsl::Features::SAMPLE_VARIABLES) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_SAMPLE_VARIABLES;
    }
    if features.contains(naga::back::glsl::Features::DYNAMIC_ARRAY_SIZE) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_DYNAMIC_ARRAY_SIZE;
    }
    if features.contains(naga::back::glsl::Features::MULTI_VIEW) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_MULTI_VIEW;
    }
    if features.contains(naga::back::glsl::Features::TEXTURE_SAMPLES) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_TEXTURE_SAMPLES;
    }
    if features.contains(naga::back::glsl::Features::TEXTURE_LEVELS) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_TEXTURE_LEVELS;
    }
    if features.contains(naga::back::glsl::Features::IMAGE_SIZE) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_IMAGE_SIZE;
    }
    if features.contains(naga::back::glsl::Features::DUAL_SOURCE_BLENDING) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_DUAL_SOURCE_BLENDING;
    }
    if features.contains(naga::back::glsl::Features::INSTANCE_INDEX) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_INSTANCE_INDEX;
    }
    if features.contains(naga::back::glsl::Features::TEXTURE_SHADOW_LOD) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_TEXTURE_SHADOW_LOD;
    }
    if features.contains(naga::back::glsl::Features::SUBGROUP_OPERATIONS) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_SUBGROUP_OPERATIONS;
    }
    if features.contains(naga::back::glsl::Features::TEXTURE_ATOMICS) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_TEXTURE_ATOMICS;
    }
    if features.contains(naga::back::glsl::Features::SHADER_BARYCENTRICS) {
        result |= ffi::GLSLBackFeatures_GLSLBackFeatures_SHADER_BARYCENTRICS;
    }

    sa::const_assert_eq!(
        naga::back::glsl::Features::all().bits(),
        naga::back::glsl::Features::BUFFER_STORAGE.bits()
            | naga::back::glsl::Features::ARRAY_OF_ARRAYS.bits()
            | naga::back::glsl::Features::DOUBLE_TYPE.bits()
            | naga::back::glsl::Features::FULL_IMAGE_FORMATS.bits()
            | naga::back::glsl::Features::MULTISAMPLED_TEXTURES.bits()
            | naga::back::glsl::Features::MULTISAMPLED_TEXTURE_ARRAYS.bits()
            | naga::back::glsl::Features::CUBE_TEXTURES_ARRAY.bits()
            | naga::back::glsl::Features::COMPUTE_SHADER.bits()
            | naga::back::glsl::Features::IMAGE_LOAD_STORE.bits()
            | naga::back::glsl::Features::CONSERVATIVE_DEPTH.bits()
            | naga::back::glsl::Features::NOPERSPECTIVE_QUALIFIER.bits()
            | naga::back::glsl::Features::SAMPLE_QUALIFIER.bits()
            | naga::back::glsl::Features::CLIP_DISTANCE.bits()
            | naga::back::glsl::Features::CULL_DISTANCE.bits()
            | naga::back::glsl::Features::SAMPLE_VARIABLES.bits()
            | naga::back::glsl::Features::DYNAMIC_ARRAY_SIZE.bits()
            | naga::back::glsl::Features::MULTI_VIEW.bits()
            | naga::back::glsl::Features::TEXTURE_SAMPLES.bits()
            | naga::back::glsl::Features::TEXTURE_LEVELS.bits()
            | naga::back::glsl::Features::IMAGE_SIZE.bits()
            | naga::back::glsl::Features::DUAL_SOURCE_BLENDING.bits()
            | naga::back::glsl::Features::INSTANCE_INDEX.bits()
            | naga::back::glsl::Features::TEXTURE_SHADOW_LOD.bits()
            | naga::back::glsl::Features::SUBGROUP_OPERATIONS.bits()
            | naga::back::glsl::Features::TEXTURE_ATOMICS.bits()
            | naga::back::glsl::Features::SHADER_BARYCENTRICS.bits()
    );

    result
}

pub fn glsl_back_error_to_ffi(error: &naga::back::glsl::Error) -> ffi::GLSLBackError {
    let default_data = ffi::GLSLBackError__bindgen_ty_1::default();

    match error {
        naga::back::glsl::Error::FmtError(error) => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_FmtError,
            data: ffi::GLSLBackError__bindgen_ty_1 {
                fmt_error: unsafe { string_to_ffi(&error.to_string()) },
            },
        },
        naga::back::glsl::Error::MissingFeatures(features) => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_MissingFeatures,
            data: ffi::GLSLBackError__bindgen_ty_1 {
                missing_features: glsl_back_features_to_ffi(*features),
            },
        },
        naga::back::glsl::Error::MultipleImmediateData => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_MultipleImmediateData,
            data: default_data,
        },
        naga::back::glsl::Error::VersionNotSupported => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_VersionNotSupported,
            data: default_data,
        },
        naga::back::glsl::Error::EntryPointNotFound => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_EntryPointNotFound,
            data: default_data,
        },
        naga::back::glsl::Error::UnsupportedExternal(external) => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_UnsupportedExternal,
            data: ffi::GLSLBackError__bindgen_ty_1 {
                unsupported_external: unsafe { string_to_ffi(external) },
            },
        },
        naga::back::glsl::Error::UnsupportedScalar(scalar) => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_UnsupportedScalar,
            data: ffi::GLSLBackError__bindgen_ty_1 {
                unsupported_scalar: scalar_to_ffi(scalar),
            },
        },
        naga::back::glsl::Error::ImageMultipleSamplers => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_ImageMultipleSamplers,
            data: default_data,
        },
        naga::back::glsl::Error::Custom(custom) => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_Custom,
            data: ffi::GLSLBackError__bindgen_ty_1 {
                custom: unsafe { string_to_ffi(custom) },
            },
        },
        naga::back::glsl::Error::Override => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_Override,
            data: default_data,
        },
        naga::back::glsl::Error::FirstSamplingNotSupported => ffi::GLSLBackError {
            tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_FirstSamplingNotSupported,
            data: default_data,
        },
        naga::back::glsl::Error::ResolveArraySizeError(resolve_array_size_error) => {
            ffi::GLSLBackError {
                tag: ffi::GLSLBackErrorTag_GLSLBackErrorTag_ResolveArraySizeError,
                data: ffi::GLSLBackError__bindgen_ty_1 {
                    resolve_array_size_error: resolve_array_size_error_to_ffi(
                        resolve_array_size_error,
                    ),
                },
            }
        }
    }
}

pub fn glsl_back_reflection_info_to_ffi(
    _info: &naga::back::glsl::ReflectionInfo,
) -> ffi::GLSLBackReflectionInfo {
    ffi::GLSLBackReflectionInfo::default()
}
