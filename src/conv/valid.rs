use super::*;

pub unsafe fn module_info_to_ffi(module_info: naga::valid::ModuleInfo) -> ffi::ModuleInfo {
    let module_info = Box::new(module_info);
    let module_info = Box::leak(module_info);
    let module_info_ptr = module_info as *mut naga::valid::ModuleInfo;
    ffi::ModuleInfo {
        _inner_module_info: module_info_ptr as *mut c_void,
    }
}

pub unsafe fn validator_to_ffi(validator: naga::valid::Validator) -> ffi::Validator {
    let validator = Box::new(validator);
    let validator = Box::leak(validator);
    let validator_ptr = validator as *mut naga::valid::Validator;
    ffi::Validator {
        _inner_validator: validator_ptr as *mut c_void,
    }
}

pub fn capabilities_to_ffi(capabilities: &naga::valid::Capabilities) -> ffi::Capabilities {
    let mut result: ffi::Capabilities = 0;
    if capabilities.contains(naga::valid::Capabilities::IMMEDIATES) {
        result |= ffi::Capabilities_Capabilities_IMMEDIATES;
    }
    if capabilities.contains(naga::valid::Capabilities::FLOAT64) {
        result |= ffi::Capabilities_Capabilities_FLOAT64;
    }
    if capabilities.contains(naga::valid::Capabilities::PRIMITIVE_INDEX) {
        result |= ffi::Capabilities_Capabilities_PRIMITIVE_INDEX;
    }
    if capabilities.contains(naga::valid::Capabilities::TEXTURE_AND_SAMPLER_BINDING_ARRAY) {
        result |= ffi::Capabilities_Capabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY;
    }
    if capabilities.contains(naga::valid::Capabilities::BUFFER_BINDING_ARRAY) {
        result |= ffi::Capabilities_Capabilities_BUFFER_BINDING_ARRAY;
    }
    if capabilities.contains(naga::valid::Capabilities::STORAGE_TEXTURE_BINDING_ARRAY) {
        result |= ffi::Capabilities_Capabilities_STORAGE_TEXTURE_BINDING_ARRAY;
    }
    if capabilities.contains(naga::valid::Capabilities::STORAGE_BUFFER_BINDING_ARRAY) {
        result |= ffi::Capabilities_Capabilities_STORAGE_BUFFER_BINDING_ARRAY;
    }
    if capabilities.contains(naga::valid::Capabilities::CLIP_DISTANCE) {
        result |= ffi::Capabilities_Capabilities_CLIP_DISTANCE;
    }
    if capabilities.contains(naga::valid::Capabilities::CULL_DISTANCE) {
        result |= ffi::Capabilities_Capabilities_CULL_DISTANCE;
    }
    if capabilities.contains(naga::valid::Capabilities::STORAGE_TEXTURE_16BIT_NORM_FORMATS) {
        result |= ffi::Capabilities_Capabilities_STORAGE_TEXTURE_16BIT_NORM_FORMATS;
    }
    if capabilities.contains(naga::valid::Capabilities::MULTIVIEW) {
        result |= ffi::Capabilities_Capabilities_MULTIVIEW;
    }
    if capabilities.contains(naga::valid::Capabilities::EARLY_DEPTH_TEST) {
        result |= ffi::Capabilities_Capabilities_EARLY_DEPTH_TEST;
    }
    if capabilities.contains(naga::valid::Capabilities::MULTISAMPLED_SHADING) {
        result |= ffi::Capabilities_Capabilities_MULTISAMPLED_SHADING;
    }
    if capabilities.contains(naga::valid::Capabilities::RAY_QUERY) {
        result |= ffi::Capabilities_Capabilities_RAY_QUERY;
    }
    if capabilities.contains(naga::valid::Capabilities::DUAL_SOURCE_BLENDING) {
        result |= ffi::Capabilities_Capabilities_DUAL_SOURCE_BLENDING;
    }
    if capabilities.contains(naga::valid::Capabilities::CUBE_ARRAY_TEXTURES) {
        result |= ffi::Capabilities_Capabilities_CUBE_ARRAY_TEXTURES;
    }
    if capabilities.contains(naga::valid::Capabilities::SHADER_INT64) {
        result |= ffi::Capabilities_Capabilities_SHADER_INT64;
    }
    if capabilities.contains(naga::valid::Capabilities::SUBGROUP) {
        result |= ffi::Capabilities_Capabilities_SUBGROUP;
    }
    if capabilities.contains(naga::valid::Capabilities::SUBGROUP_BARRIER) {
        result |= ffi::Capabilities_Capabilities_SUBGROUP_BARRIER;
    }
    if capabilities.contains(naga::valid::Capabilities::SUBGROUP_VERTEX_STAGE) {
        result |= ffi::Capabilities_Capabilities_SUBGROUP_VERTEX_STAGE;
    }
    if capabilities.contains(naga::valid::Capabilities::SHADER_INT64_ATOMIC_MIN_MAX) {
        result |= ffi::Capabilities_Capabilities_SHADER_INT64_ATOMIC_MIN_MAX;
    }
    if capabilities.contains(naga::valid::Capabilities::SHADER_INT64_ATOMIC_ALL_OPS) {
        result |= ffi::Capabilities_Capabilities_SHADER_INT64_ATOMIC_ALL_OPS;
    }
    if capabilities.contains(naga::valid::Capabilities::SHADER_FLOAT32_ATOMIC) {
        result |= ffi::Capabilities_Capabilities_SHADER_FLOAT32_ATOMIC;
    }
    if capabilities.contains(naga::valid::Capabilities::TEXTURE_ATOMIC) {
        result |= ffi::Capabilities_Capabilities_TEXTURE_ATOMIC;
    }
    if capabilities.contains(naga::valid::Capabilities::TEXTURE_INT64_ATOMIC) {
        result |= ffi::Capabilities_Capabilities_TEXTURE_INT64_ATOMIC;
    }
    if capabilities.contains(naga::valid::Capabilities::RAY_HIT_VERTEX_POSITION) {
        result |= ffi::Capabilities_Capabilities_RAY_HIT_VERTEX_POSITION;
    }
    if capabilities.contains(naga::valid::Capabilities::SHADER_FLOAT16) {
        result |= ffi::Capabilities_Capabilities_SHADER_FLOAT16;
    }
    if capabilities.contains(naga::valid::Capabilities::TEXTURE_EXTERNAL) {
        result |= ffi::Capabilities_Capabilities_TEXTURE_EXTERNAL;
    }
    if capabilities.contains(naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32) {
        result |= ffi::Capabilities_Capabilities_SHADER_FLOAT16_IN_FLOAT32;
    }
    if capabilities.contains(naga::valid::Capabilities::SHADER_BARYCENTRICS) {
        result |= ffi::Capabilities_Capabilities_SHADER_BARYCENTRICS;
    }
    if capabilities.contains(naga::valid::Capabilities::MESH_SHADER) {
        result |= ffi::Capabilities_Capabilities_MESH_SHADER;
    }
    if capabilities.contains(naga::valid::Capabilities::MESH_SHADER_POINT_TOPOLOGY) {
        result |= ffi::Capabilities_Capabilities_MESH_SHADER_POINT_TOPOLOGY;
    }
    if capabilities
        .contains(naga::valid::Capabilities::TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING)
    {
        result |=
            ffi::Capabilities_Capabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING;
    }
    if capabilities.contains(naga::valid::Capabilities::BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING) {
        result |= ffi::Capabilities_Capabilities_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING;
    }
    if capabilities
        .contains(naga::valid::Capabilities::STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING)
    {
        result |= ffi::Capabilities_Capabilities_STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING;
    }
    if capabilities
        .contains(naga::valid::Capabilities::STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING)
    {
        result |= ffi::Capabilities_Capabilities_STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING;
    }
    sa::const_assert_eq!(
        naga::valid::Capabilities::all().bits(),
        naga::valid::Capabilities::IMMEDIATES.bits()
            | naga::valid::Capabilities::FLOAT64.bits()
            | naga::valid::Capabilities::PRIMITIVE_INDEX.bits()
            | naga::valid::Capabilities::TEXTURE_AND_SAMPLER_BINDING_ARRAY.bits()
            | naga::valid::Capabilities::BUFFER_BINDING_ARRAY.bits()
            | naga::valid::Capabilities::STORAGE_TEXTURE_BINDING_ARRAY.bits()
            | naga::valid::Capabilities::STORAGE_BUFFER_BINDING_ARRAY.bits()
            | naga::valid::Capabilities::CLIP_DISTANCE.bits()
            | naga::valid::Capabilities::CULL_DISTANCE.bits()
            | naga::valid::Capabilities::STORAGE_TEXTURE_16BIT_NORM_FORMATS.bits()
            | naga::valid::Capabilities::MULTIVIEW.bits()
            | naga::valid::Capabilities::EARLY_DEPTH_TEST.bits()
            | naga::valid::Capabilities::MULTISAMPLED_SHADING.bits()
            | naga::valid::Capabilities::RAY_QUERY.bits()
            | naga::valid::Capabilities::DUAL_SOURCE_BLENDING.bits()
            | naga::valid::Capabilities::CUBE_ARRAY_TEXTURES.bits()
            | naga::valid::Capabilities::SHADER_INT64.bits()
            | naga::valid::Capabilities::SUBGROUP.bits()
            | naga::valid::Capabilities::SUBGROUP_BARRIER.bits()
            | naga::valid::Capabilities::SUBGROUP_VERTEX_STAGE.bits()
            | naga::valid::Capabilities::SHADER_INT64_ATOMIC_MIN_MAX.bits()
            | naga::valid::Capabilities::SHADER_INT64_ATOMIC_ALL_OPS.bits()
            | naga::valid::Capabilities::SHADER_FLOAT32_ATOMIC.bits()
            | naga::valid::Capabilities::TEXTURE_ATOMIC.bits()
            | naga::valid::Capabilities::TEXTURE_INT64_ATOMIC.bits()
            | naga::valid::Capabilities::RAY_HIT_VERTEX_POSITION.bits()
            | naga::valid::Capabilities::SHADER_FLOAT16.bits()
            | naga::valid::Capabilities::TEXTURE_EXTERNAL.bits()
            | naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32.bits()
            | naga::valid::Capabilities::SHADER_BARYCENTRICS.bits()
            | naga::valid::Capabilities::MESH_SHADER.bits()
            | naga::valid::Capabilities::MESH_SHADER_POINT_TOPOLOGY.bits()
            | naga::valid::Capabilities::TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING
                .bits()
            | naga::valid::Capabilities::BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING.bits()
            | naga::valid::Capabilities::STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING.bits()
            | naga::valid::Capabilities::STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING.bits()
    );
    result
}

pub fn capabilities_to_naga(capabilities: ffi::Capabilities) -> naga::valid::Capabilities {
    let mut result = naga::valid::Capabilities::empty();

    if capabilities & ffi::Capabilities_Capabilities_IMMEDIATES != 0 {
        result |= naga::valid::Capabilities::IMMEDIATES;
    }
    if capabilities & ffi::Capabilities_Capabilities_FLOAT64 != 0 {
        result |= naga::valid::Capabilities::FLOAT64;
    }
    if capabilities & ffi::Capabilities_Capabilities_PRIMITIVE_INDEX != 0 {
        result |= naga::valid::Capabilities::PRIMITIVE_INDEX;
    }
    if capabilities & ffi::Capabilities_Capabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY != 0 {
        result |= naga::valid::Capabilities::TEXTURE_AND_SAMPLER_BINDING_ARRAY;
    }
    if capabilities & ffi::Capabilities_Capabilities_BUFFER_BINDING_ARRAY != 0 {
        result |= naga::valid::Capabilities::BUFFER_BINDING_ARRAY;
    }
    if capabilities & ffi::Capabilities_Capabilities_STORAGE_TEXTURE_BINDING_ARRAY != 0 {
        result |= naga::valid::Capabilities::STORAGE_TEXTURE_BINDING_ARRAY;
    }
    if capabilities & ffi::Capabilities_Capabilities_STORAGE_BUFFER_BINDING_ARRAY != 0 {
        result |= naga::valid::Capabilities::STORAGE_BUFFER_BINDING_ARRAY;
    }
    if capabilities & ffi::Capabilities_Capabilities_CLIP_DISTANCE != 0 {
        result |= naga::valid::Capabilities::CLIP_DISTANCE;
    }
    if capabilities & ffi::Capabilities_Capabilities_CULL_DISTANCE != 0 {
        result |= naga::valid::Capabilities::CULL_DISTANCE;
    }
    if capabilities & ffi::Capabilities_Capabilities_STORAGE_TEXTURE_16BIT_NORM_FORMATS != 0 {
        result |= naga::valid::Capabilities::STORAGE_TEXTURE_16BIT_NORM_FORMATS;
    }
    if capabilities & ffi::Capabilities_Capabilities_MULTIVIEW != 0 {
        result |= naga::valid::Capabilities::MULTIVIEW;
    }
    if capabilities & ffi::Capabilities_Capabilities_EARLY_DEPTH_TEST != 0 {
        result |= naga::valid::Capabilities::EARLY_DEPTH_TEST;
    }
    if capabilities & ffi::Capabilities_Capabilities_MULTISAMPLED_SHADING != 0 {
        result |= naga::valid::Capabilities::MULTISAMPLED_SHADING;
    }
    if capabilities & ffi::Capabilities_Capabilities_RAY_QUERY != 0 {
        result |= naga::valid::Capabilities::RAY_QUERY;
    }
    if capabilities & ffi::Capabilities_Capabilities_DUAL_SOURCE_BLENDING != 0 {
        result |= naga::valid::Capabilities::DUAL_SOURCE_BLENDING;
    }
    if capabilities & ffi::Capabilities_Capabilities_CUBE_ARRAY_TEXTURES != 0 {
        result |= naga::valid::Capabilities::CUBE_ARRAY_TEXTURES;
    }
    if capabilities & ffi::Capabilities_Capabilities_SHADER_INT64 != 0 {
        result |= naga::valid::Capabilities::SHADER_INT64;
    }
    if capabilities & ffi::Capabilities_Capabilities_SUBGROUP != 0 {
        result |= naga::valid::Capabilities::SUBGROUP;
    }
    if capabilities & ffi::Capabilities_Capabilities_SUBGROUP_BARRIER != 0 {
        result |= naga::valid::Capabilities::SUBGROUP_BARRIER;
    }
    if capabilities & ffi::Capabilities_Capabilities_SUBGROUP_VERTEX_STAGE != 0 {
        result |= naga::valid::Capabilities::SUBGROUP_VERTEX_STAGE;
    }
    if capabilities & ffi::Capabilities_Capabilities_SHADER_INT64_ATOMIC_MIN_MAX != 0 {
        result |= naga::valid::Capabilities::SHADER_INT64_ATOMIC_MIN_MAX;
    }
    if capabilities & ffi::Capabilities_Capabilities_SHADER_INT64_ATOMIC_ALL_OPS != 0 {
        result |= naga::valid::Capabilities::SHADER_INT64_ATOMIC_ALL_OPS;
    }
    if capabilities & ffi::Capabilities_Capabilities_SHADER_FLOAT32_ATOMIC != 0 {
        result |= naga::valid::Capabilities::SHADER_FLOAT32_ATOMIC;
    }
    if capabilities & ffi::Capabilities_Capabilities_TEXTURE_ATOMIC != 0 {
        result |= naga::valid::Capabilities::TEXTURE_ATOMIC;
    }
    if capabilities & ffi::Capabilities_Capabilities_TEXTURE_INT64_ATOMIC != 0 {
        result |= naga::valid::Capabilities::TEXTURE_INT64_ATOMIC;
    }
    if capabilities & ffi::Capabilities_Capabilities_RAY_HIT_VERTEX_POSITION != 0 {
        result |= naga::valid::Capabilities::RAY_HIT_VERTEX_POSITION;
    }
    if capabilities & ffi::Capabilities_Capabilities_SHADER_FLOAT16 != 0 {
        result |= naga::valid::Capabilities::SHADER_FLOAT16;
    }
    if capabilities & ffi::Capabilities_Capabilities_TEXTURE_EXTERNAL != 0 {
        result |= naga::valid::Capabilities::TEXTURE_EXTERNAL;
    }
    if capabilities & ffi::Capabilities_Capabilities_SHADER_FLOAT16_IN_FLOAT32 != 0 {
        result |= naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32;
    }
    if capabilities & ffi::Capabilities_Capabilities_SHADER_BARYCENTRICS != 0 {
        result |= naga::valid::Capabilities::SHADER_BARYCENTRICS;
    }
    if capabilities & ffi::Capabilities_Capabilities_MESH_SHADER != 0 {
        result |= naga::valid::Capabilities::MESH_SHADER;
    }
    if capabilities & ffi::Capabilities_Capabilities_MESH_SHADER_POINT_TOPOLOGY != 0 {
        result |= naga::valid::Capabilities::MESH_SHADER_POINT_TOPOLOGY;
    }
    if capabilities
        & ffi::Capabilities_Capabilities_TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING
        != 0
    {
        result |= naga::valid::Capabilities::TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING;
    }
    if capabilities & ffi::Capabilities_Capabilities_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING != 0
    {
        result |= naga::valid::Capabilities::BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING;
    }
    if capabilities
        & ffi::Capabilities_Capabilities_STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING
        != 0
    {
        result |= naga::valid::Capabilities::STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING;
    }
    if capabilities
        & ffi::Capabilities_Capabilities_STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING
        != 0
    {
        result |= naga::valid::Capabilities::STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING;
    }

    sa::const_assert_eq!(
        naga::valid::Capabilities::all().bits(),
        naga::valid::Capabilities::IMMEDIATES.bits()
            | naga::valid::Capabilities::FLOAT64.bits()
            | naga::valid::Capabilities::PRIMITIVE_INDEX.bits()
            | naga::valid::Capabilities::TEXTURE_AND_SAMPLER_BINDING_ARRAY.bits()
            | naga::valid::Capabilities::BUFFER_BINDING_ARRAY.bits()
            | naga::valid::Capabilities::STORAGE_TEXTURE_BINDING_ARRAY.bits()
            | naga::valid::Capabilities::STORAGE_BUFFER_BINDING_ARRAY.bits()
            | naga::valid::Capabilities::CLIP_DISTANCE.bits()
            | naga::valid::Capabilities::CULL_DISTANCE.bits()
            | naga::valid::Capabilities::STORAGE_TEXTURE_16BIT_NORM_FORMATS.bits()
            | naga::valid::Capabilities::MULTIVIEW.bits()
            | naga::valid::Capabilities::EARLY_DEPTH_TEST.bits()
            | naga::valid::Capabilities::MULTISAMPLED_SHADING.bits()
            | naga::valid::Capabilities::RAY_QUERY.bits()
            | naga::valid::Capabilities::DUAL_SOURCE_BLENDING.bits()
            | naga::valid::Capabilities::CUBE_ARRAY_TEXTURES.bits()
            | naga::valid::Capabilities::SHADER_INT64.bits()
            | naga::valid::Capabilities::SUBGROUP.bits()
            | naga::valid::Capabilities::SUBGROUP_BARRIER.bits()
            | naga::valid::Capabilities::SUBGROUP_VERTEX_STAGE.bits()
            | naga::valid::Capabilities::SHADER_INT64_ATOMIC_MIN_MAX.bits()
            | naga::valid::Capabilities::SHADER_INT64_ATOMIC_ALL_OPS.bits()
            | naga::valid::Capabilities::SHADER_FLOAT32_ATOMIC.bits()
            | naga::valid::Capabilities::TEXTURE_ATOMIC.bits()
            | naga::valid::Capabilities::TEXTURE_INT64_ATOMIC.bits()
            | naga::valid::Capabilities::RAY_HIT_VERTEX_POSITION.bits()
            | naga::valid::Capabilities::SHADER_FLOAT16.bits()
            | naga::valid::Capabilities::TEXTURE_EXTERNAL.bits()
            | naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32.bits()
            | naga::valid::Capabilities::SHADER_BARYCENTRICS.bits()
            | naga::valid::Capabilities::MESH_SHADER.bits()
            | naga::valid::Capabilities::MESH_SHADER_POINT_TOPOLOGY.bits()
            | naga::valid::Capabilities::TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING
                .bits()
            | naga::valid::Capabilities::BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING.bits()
            | naga::valid::Capabilities::STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING.bits()
            | naga::valid::Capabilities::STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING.bits()
    );

    result
}

pub fn validation_flags_to_naga(flags: ffi::ValidationFlags) -> naga::valid::ValidationFlags {
    let mut result = naga::valid::ValidationFlags::empty();

    if flags & ffi::ValidationFlags_ValidationFlags_EXPRESSIONS != 0 {
        result |= naga::valid::ValidationFlags::EXPRESSIONS;
    }
    if flags & ffi::ValidationFlags_ValidationFlags_BLOCKS != 0 {
        result |= naga::valid::ValidationFlags::BLOCKS;
    }
    if flags & ffi::ValidationFlags_ValidationFlags_CONTROL_FLOW_UNIFORMITY != 0 {
        result |= naga::valid::ValidationFlags::CONTROL_FLOW_UNIFORMITY;
    }
    if flags & ffi::ValidationFlags_ValidationFlags_STRUCT_LAYOUTS != 0 {
        result |= naga::valid::ValidationFlags::STRUCT_LAYOUTS;
    }
    if flags & ffi::ValidationFlags_ValidationFlags_CONSTANTS != 0 {
        result |= naga::valid::ValidationFlags::CONSTANTS;
    }
    if flags & ffi::ValidationFlags_ValidationFlags_BINDINGS != 0 {
        result |= naga::valid::ValidationFlags::BINDINGS;
    }

    sa::const_assert_eq!(
        naga::valid::ValidationFlags::all().bits(),
        naga::valid::ValidationFlags::EXPRESSIONS.bits()
            | naga::valid::ValidationFlags::BLOCKS.bits()
            | naga::valid::ValidationFlags::CONTROL_FLOW_UNIFORMITY.bits()
            | naga::valid::ValidationFlags::STRUCT_LAYOUTS.bits()
            | naga::valid::ValidationFlags::CONSTANTS.bits()
            | naga::valid::ValidationFlags::BINDINGS.bits()
    );

    result
}
