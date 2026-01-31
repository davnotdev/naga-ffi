use crate::ffi::ArraySize__bindgen_ty_1;

use super::*;

pub fn scalar_kind_to_ffi(scalar_kind: &naga::ir::ScalarKind) -> ffi::ScalarKind {
    match scalar_kind {
        naga::ScalarKind::Sint => ffi::ScalarKind_ScalarKind_Sint,
        naga::ScalarKind::Uint => ffi::ScalarKind_ScalarKind_Uint,
        naga::ScalarKind::Float => ffi::ScalarKind_ScalarKind_Float,
        naga::ScalarKind::Bool => ffi::ScalarKind_ScalarKind_Bool,
        naga::ScalarKind::AbstractInt => ffi::ScalarKind_ScalarKind_AbstractInt,
        naga::ScalarKind::AbstractFloat => ffi::ScalarKind_ScalarKind_AbstractFloat,
    }
}

pub fn scalar_to_ffi(scalar: &naga::ir::Scalar) -> ffi::Scalar {
    ffi::Scalar {
        kind: scalar_kind_to_ffi(&scalar.kind),
        width: scalar.width,
    }
}

pub fn vector_size_to_ffi(vector_size: &naga::ir::VectorSize) -> ffi::VectorSize {
    match vector_size {
        naga::VectorSize::Bi => ffi::VectorSize_VectorSize_Bi,
        naga::VectorSize::Tri => ffi::VectorSize_VectorSize_Tri,
        naga::VectorSize::Quad => ffi::VectorSize_VectorSize_Quad,
    }
}

pub fn array_size_to_ffi(array_size: &naga::ir::ArraySize) -> ffi::ArraySize {
    match array_size {
        naga::ArraySize::Constant(non_zero) => ffi::ArraySize {
            tag: ffi::ArraySizeTag_ArraySizeTag_Constant,
            data: ArraySize__bindgen_ty_1 {
                constant: non_zero.get(),
            },
        },
        naga::ArraySize::Pending(handle) => ffi::ArraySize {
            tag: ffi::ArraySizeTag_ArraySizeTag_Pending,
            data: ArraySize__bindgen_ty_1 {
                pending: handle.index(),
            },
        },
        naga::ArraySize::Dynamic => ffi::ArraySize {
            tag: ffi::ArraySizeTag_ArraySizeTag_Dynamic,
            data: ArraySize__bindgen_ty_1 { constant: 0 },
        },
    }
}

pub fn storage_format_to_ffi(storage_format: &naga::ir::StorageFormat) -> ffi::StorageFormat {
    match storage_format {
        naga::StorageFormat::R8Unorm => ffi::StorageFormat_StorageFormat_R8Unorm,
        naga::StorageFormat::R8Snorm => ffi::StorageFormat_StorageFormat_R8Snorm,
        naga::StorageFormat::R8Uint => ffi::StorageFormat_StorageFormat_R8Uint,
        naga::StorageFormat::R8Sint => ffi::StorageFormat_StorageFormat_R8Sint,
        naga::StorageFormat::R16Uint => ffi::StorageFormat_StorageFormat_R16Uint,
        naga::StorageFormat::R16Sint => ffi::StorageFormat_StorageFormat_R16Sint,
        naga::StorageFormat::R16Float => ffi::StorageFormat_StorageFormat_R16Float,
        naga::StorageFormat::Rg8Unorm => ffi::StorageFormat_StorageFormat_Rg8Unorm,
        naga::StorageFormat::Rg8Snorm => ffi::StorageFormat_StorageFormat_Rg8Snorm,
        naga::StorageFormat::Rg8Uint => ffi::StorageFormat_StorageFormat_Rg8Uint,
        naga::StorageFormat::Rg8Sint => ffi::StorageFormat_StorageFormat_Rg8Sint,
        naga::StorageFormat::R32Uint => ffi::StorageFormat_StorageFormat_R32Uint,
        naga::StorageFormat::R32Sint => ffi::StorageFormat_StorageFormat_R32Sint,
        naga::StorageFormat::R32Float => ffi::StorageFormat_StorageFormat_R32Float,
        naga::StorageFormat::Rg16Uint => ffi::StorageFormat_StorageFormat_Rg16Uint,
        naga::StorageFormat::Rg16Sint => ffi::StorageFormat_StorageFormat_Rg16Sint,
        naga::StorageFormat::Rg16Float => ffi::StorageFormat_StorageFormat_Rg16Float,
        naga::StorageFormat::Rgba8Unorm => ffi::StorageFormat_StorageFormat_Rgba8Unorm,
        naga::StorageFormat::Rgba8Snorm => ffi::StorageFormat_StorageFormat_Rgba8Snorm,
        naga::StorageFormat::Rgba8Uint => ffi::StorageFormat_StorageFormat_Rgba8Uint,
        naga::StorageFormat::Rgba8Sint => ffi::StorageFormat_StorageFormat_Rgba8Sint,
        naga::StorageFormat::Bgra8Unorm => ffi::StorageFormat_StorageFormat_Bgra8Unorm,
        naga::StorageFormat::Rgb10a2Uint => ffi::StorageFormat_StorageFormat_Rgb10a2Uint,
        naga::StorageFormat::Rgb10a2Unorm => ffi::StorageFormat_StorageFormat_Rgb10a2Unorm,
        naga::StorageFormat::Rg11b10Ufloat => ffi::StorageFormat_StorageFormat_Rg11b10Ufloat,
        naga::StorageFormat::R64Uint => ffi::StorageFormat_StorageFormat_R64Uint,
        naga::StorageFormat::Rg32Uint => ffi::StorageFormat_StorageFormat_Rg32Uint,
        naga::StorageFormat::Rg32Sint => ffi::StorageFormat_StorageFormat_Rg32Sint,
        naga::StorageFormat::Rg32Float => ffi::StorageFormat_StorageFormat_Rg32Float,
        naga::StorageFormat::Rgba16Uint => ffi::StorageFormat_StorageFormat_Rgba16Uint,
        naga::StorageFormat::Rgba16Sint => ffi::StorageFormat_StorageFormat_Rgba16Sint,
        naga::StorageFormat::Rgba16Float => ffi::StorageFormat_StorageFormat_Rgba16Float,
        naga::StorageFormat::Rgba32Uint => ffi::StorageFormat_StorageFormat_Rgba32Uint,
        naga::StorageFormat::Rgba32Sint => ffi::StorageFormat_StorageFormat_Rgba32Sint,
        naga::StorageFormat::Rgba32Float => ffi::StorageFormat_StorageFormat_Rgba32Float,
        naga::StorageFormat::R16Unorm => ffi::StorageFormat_StorageFormat_R16Unorm,
        naga::StorageFormat::R16Snorm => ffi::StorageFormat_StorageFormat_R16Snorm,
        naga::StorageFormat::Rg16Unorm => ffi::StorageFormat_StorageFormat_Rg16Unorm,
        naga::StorageFormat::Rg16Snorm => ffi::StorageFormat_StorageFormat_Rg16Snorm,
        naga::StorageFormat::Rgba16Unorm => ffi::StorageFormat_StorageFormat_Rgba16Unorm,
        naga::StorageFormat::Rgba16Snorm => ffi::StorageFormat_StorageFormat_Rgba16Snorm,
    }
}

pub fn storage_access_to_ffi(storage_access: &naga::ir::StorageAccess) -> ffi::StorageAccess {
    let mut result: ffi::StorageAccess = 0;
    if storage_access.contains(naga::ir::StorageAccess::LOAD) {
        result |= ffi::StorageAccess_StorageAccess_LOAD;
    }
    if storage_access.contains(naga::ir::StorageAccess::STORE) {
        result |= ffi::StorageAccess_StorageAccess_STORE;
    }
    if storage_access.contains(naga::ir::StorageAccess::ATOMIC) {
        result |= ffi::StorageAccess_StorageAccess_ATOMIC;
    }
    sa::const_assert_eq!(
        naga::ir::StorageAccess::all().bits(),
        naga::ir::StorageAccess::LOAD.bits()
            | naga::ir::StorageAccess::STORE.bits()
            | naga::ir::StorageAccess::ATOMIC.bits()
    );

    result
}

pub fn image_class_to_ffi(image_class: &naga::ir::ImageClass) -> ffi::ImageClass {
    match image_class {
        naga::ImageClass::Sampled { kind, multi } => ffi::ImageClass {
            tag: ffi::ImageClassTag_ImageClassTag_Sampled,
            data: ffi::ImageClass__bindgen_ty_1 {
                sampled: ffi::ImageClass__bindgen_ty_1__bindgen_ty_1 {
                    kind: scalar_kind_to_ffi(kind),
                    multi: bool_to_ffi(*multi),
                },
            },
        },
        naga::ImageClass::Depth { multi } => ffi::ImageClass {
            tag: ffi::ImageClassTag_ImageClassTag_Depth,
            data: ffi::ImageClass__bindgen_ty_1 {
                depth: ffi::ImageClass__bindgen_ty_1__bindgen_ty_2 {
                    multi: bool_to_ffi(*multi),
                },
            },
        },
        naga::ImageClass::External => ffi::ImageClass {
            tag: ffi::ImageClassTag_ImageClassTag_External,
            data: ffi::ImageClass__bindgen_ty_1 {
                storage: ffi::ImageClass__bindgen_ty_1__bindgen_ty_3 {
                    format: 0,
                    access: 0,
                },
            },
        },
        naga::ImageClass::Storage { format, access } => ffi::ImageClass {
            tag: ffi::ImageClassTag_ImageClassTag_Storage,
            data: ffi::ImageClass__bindgen_ty_1 {
                storage: ffi::ImageClass__bindgen_ty_1__bindgen_ty_3 {
                    format: storage_format_to_ffi(format),
                    access: storage_access_to_ffi(access),
                },
            },
        },
    }
}

pub fn address_space_to_ffi(address_space: &naga::ir::AddressSpace) -> ffi::AddressSpace {
    let default_data = ffi::AddressSpace__bindgen_ty_1 {
        storage: ffi::AddressSpace__bindgen_ty_1__bindgen_ty_1 { access: 0 },
    };
    match address_space {
        naga::AddressSpace::Function => ffi::AddressSpace {
            tag: ffi::AddressSpaceTag_AddressSpaceTag_Function,
            data: default_data,
        },
        naga::AddressSpace::Private => ffi::AddressSpace {
            tag: ffi::AddressSpaceTag_AddressSpaceTag_Private,
            data: default_data,
        },
        naga::AddressSpace::WorkGroup => ffi::AddressSpace {
            tag: ffi::AddressSpaceTag_AddressSpaceTag_WorkGroup,
            data: default_data,
        },
        naga::AddressSpace::Uniform => ffi::AddressSpace {
            tag: ffi::AddressSpaceTag_AddressSpaceTag_Uniform,
            data: default_data,
        },
        naga::AddressSpace::Storage { access } => ffi::AddressSpace {
            tag: ffi::AddressSpaceTag_AddressSpaceTag_Storage,
            data: ffi::AddressSpace__bindgen_ty_1 {
                storage: ffi::AddressSpace__bindgen_ty_1__bindgen_ty_1 {
                    access: storage_access_to_ffi(access),
                },
            },
        },
        naga::AddressSpace::Handle => ffi::AddressSpace {
            tag: ffi::AddressSpaceTag_AddressSpaceTag_Handle,
            data: default_data,
        },
        naga::AddressSpace::Immediate => ffi::AddressSpace {
            tag: ffi::AddressSpaceTag_AddressSpaceTag_Immediate,
            data: default_data,
        },
        naga::AddressSpace::TaskPayload => ffi::AddressSpace {
            tag: ffi::AddressSpaceTag_AddressSpaceTag_TaskPayload,
            data: default_data,
        },
    }
}

pub fn math_function_to_ffi(math_function: &naga::ir::MathFunction) -> ffi::MathFunction {
    match math_function {
        naga::MathFunction::Abs => ffi::MathFunction_MathFunction_Abs,
        naga::MathFunction::Min => ffi::MathFunction_MathFunction_Min,
        naga::MathFunction::Max => ffi::MathFunction_MathFunction_Max,
        naga::MathFunction::Clamp => ffi::MathFunction_MathFunction_Clamp,
        naga::MathFunction::Saturate => ffi::MathFunction_MathFunction_Saturate,
        naga::MathFunction::Cos => ffi::MathFunction_MathFunction_Cos,
        naga::MathFunction::Cosh => ffi::MathFunction_MathFunction_Cosh,
        naga::MathFunction::Sin => ffi::MathFunction_MathFunction_Sin,
        naga::MathFunction::Sinh => ffi::MathFunction_MathFunction_Sinh,
        naga::MathFunction::Tan => ffi::MathFunction_MathFunction_Tan,
        naga::MathFunction::Tanh => ffi::MathFunction_MathFunction_Tanh,
        naga::MathFunction::Acos => ffi::MathFunction_MathFunction_Acos,
        naga::MathFunction::Asin => ffi::MathFunction_MathFunction_Asin,
        naga::MathFunction::Atan => ffi::MathFunction_MathFunction_Atan,
        naga::MathFunction::Atan2 => ffi::MathFunction_MathFunction_Atan2,
        naga::MathFunction::Asinh => ffi::MathFunction_MathFunction_Asinh,
        naga::MathFunction::Acosh => ffi::MathFunction_MathFunction_Acosh,
        naga::MathFunction::Atanh => ffi::MathFunction_MathFunction_Atanh,
        naga::MathFunction::Radians => ffi::MathFunction_MathFunction_Radians,
        naga::MathFunction::Degrees => ffi::MathFunction_MathFunction_Degrees,
        naga::MathFunction::Ceil => ffi::MathFunction_MathFunction_Ceil,
        naga::MathFunction::Floor => ffi::MathFunction_MathFunction_Floor,
        naga::MathFunction::Round => ffi::MathFunction_MathFunction_Round,
        naga::MathFunction::Fract => ffi::MathFunction_MathFunction_Fract,
        naga::MathFunction::Trunc => ffi::MathFunction_MathFunction_Trunc,
        naga::MathFunction::Modf => ffi::MathFunction_MathFunction_Modf,
        naga::MathFunction::Frexp => ffi::MathFunction_MathFunction_Frexp,
        naga::MathFunction::Ldexp => ffi::MathFunction_MathFunction_Ldexp,
        naga::MathFunction::Exp => ffi::MathFunction_MathFunction_Exp,
        naga::MathFunction::Exp2 => ffi::MathFunction_MathFunction_Exp2,
        naga::MathFunction::Log => ffi::MathFunction_MathFunction_Log,
        naga::MathFunction::Log2 => ffi::MathFunction_MathFunction_Log2,
        naga::MathFunction::Pow => ffi::MathFunction_MathFunction_Pow,
        naga::MathFunction::Dot => ffi::MathFunction_MathFunction_Dot,
        naga::MathFunction::Dot4I8Packed => ffi::MathFunction_MathFunction_Dot4I8Packed,
        naga::MathFunction::Dot4U8Packed => ffi::MathFunction_MathFunction_Dot4U8Packed,
        naga::MathFunction::Outer => ffi::MathFunction_MathFunction_Outer,
        naga::MathFunction::Cross => ffi::MathFunction_MathFunction_Cross,
        naga::MathFunction::Distance => ffi::MathFunction_MathFunction_Distance,
        naga::MathFunction::Length => ffi::MathFunction_MathFunction_Length,
        naga::MathFunction::Normalize => ffi::MathFunction_MathFunction_Normalize,
        naga::MathFunction::FaceForward => ffi::MathFunction_MathFunction_FaceForward,
        naga::MathFunction::Reflect => ffi::MathFunction_MathFunction_Reflect,
        naga::MathFunction::Refract => ffi::MathFunction_MathFunction_Refract,
        naga::MathFunction::Sign => ffi::MathFunction_MathFunction_Sign,
        naga::MathFunction::Fma => ffi::MathFunction_MathFunction_Fma,
        naga::MathFunction::Mix => ffi::MathFunction_MathFunction_Mix,
        naga::MathFunction::Step => ffi::MathFunction_MathFunction_Step,
        naga::MathFunction::SmoothStep => ffi::MathFunction_MathFunction_SmoothStep,
        naga::MathFunction::Sqrt => ffi::MathFunction_MathFunction_Sqrt,
        naga::MathFunction::InverseSqrt => ffi::MathFunction_MathFunction_InverseSqrt,
        naga::MathFunction::Inverse => ffi::MathFunction_MathFunction_Inverse,
        naga::MathFunction::Transpose => ffi::MathFunction_MathFunction_Transpose,
        naga::MathFunction::Determinant => ffi::MathFunction_MathFunction_Determinant,
        naga::MathFunction::QuantizeToF16 => ffi::MathFunction_MathFunction_QuantizeToF16,
        naga::MathFunction::CountTrailingZeros => ffi::MathFunction_MathFunction_CountTrailingZeros,
        naga::MathFunction::CountLeadingZeros => ffi::MathFunction_MathFunction_CountLeadingZeros,
        naga::MathFunction::CountOneBits => ffi::MathFunction_MathFunction_CountOneBits,
        naga::MathFunction::ReverseBits => ffi::MathFunction_MathFunction_ReverseBits,
        naga::MathFunction::ExtractBits => ffi::MathFunction_MathFunction_ExtractBits,
        naga::MathFunction::InsertBits => ffi::MathFunction_MathFunction_InsertBits,
        naga::MathFunction::FirstTrailingBit => ffi::MathFunction_MathFunction_FirstTrailingBit,
        naga::MathFunction::FirstLeadingBit => ffi::MathFunction_MathFunction_FirstLeadingBit,
        naga::MathFunction::Pack4x8snorm => ffi::MathFunction_MathFunction_Pack4x8snorm,
        naga::MathFunction::Pack4x8unorm => ffi::MathFunction_MathFunction_Pack4x8unorm,
        naga::MathFunction::Pack2x16snorm => ffi::MathFunction_MathFunction_Pack2x16snorm,
        naga::MathFunction::Pack2x16unorm => ffi::MathFunction_MathFunction_Pack2x16unorm,
        naga::MathFunction::Pack2x16float => ffi::MathFunction_MathFunction_Pack2x16float,
        naga::MathFunction::Pack4xI8 => ffi::MathFunction_MathFunction_Pack4xI8,
        naga::MathFunction::Pack4xU8 => ffi::MathFunction_MathFunction_Pack4xU8,
        naga::MathFunction::Pack4xI8Clamp => ffi::MathFunction_MathFunction_Pack4xI8Clamp,
        naga::MathFunction::Pack4xU8Clamp => ffi::MathFunction_MathFunction_Pack4xU8Clamp,
        naga::MathFunction::Unpack4x8snorm => ffi::MathFunction_MathFunction_Unpack4x8snorm,
        naga::MathFunction::Unpack4x8unorm => ffi::MathFunction_MathFunction_Unpack4x8unorm,
        naga::MathFunction::Unpack2x16snorm => ffi::MathFunction_MathFunction_Unpack2x16snorm,
        naga::MathFunction::Unpack2x16unorm => ffi::MathFunction_MathFunction_Unpack2x16unorm,
        naga::MathFunction::Unpack2x16float => ffi::MathFunction_MathFunction_Unpack2x16float,
        naga::MathFunction::Unpack4xI8 => ffi::MathFunction_MathFunction_Unpack4xI8,
        naga::MathFunction::Unpack4xU8 => ffi::MathFunction_MathFunction_Unpack4xU8,
    }
}

pub fn interpolation_to_ffi(interpolation: &naga::ir::Interpolation) -> ffi::Interpolation {
    match interpolation {
        naga::Interpolation::Perspective => ffi::Interpolation_Interpolation_Perspective,
        naga::Interpolation::Linear => ffi::Interpolation_Interpolation_Linear,
        naga::Interpolation::Flat => ffi::Interpolation_Interpolation_Flat,
    }
}

pub fn sampling_to_ffi(sampling: &naga::ir::Sampling) -> ffi::Sampling {
    match sampling {
        naga::Sampling::Center => ffi::Sampling_Sampling_Center,
        naga::Sampling::Centroid => ffi::Sampling_Sampling_Centroid,
        naga::Sampling::Sample => ffi::Sampling_Sampling_Sample,
        naga::Sampling::First => ffi::Sampling_Sampling_First,
        naga::Sampling::Either => ffi::Sampling_Sampling_Either,
    }
}

pub fn built_in_to_ffi(built_in: &naga::ir::BuiltIn) -> ffi::BuiltIn {
    let default_data = ffi::BuiltIn__bindgen_ty_1 {
        position: ffi::BuiltIn__bindgen_ty_1__bindgen_ty_1 { invariant: 0 },
    };

    match built_in {
        naga::BuiltIn::Position { invariant } => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_Position,
            data: ffi::BuiltIn__bindgen_ty_1 {
                position: ffi::BuiltIn__bindgen_ty_1__bindgen_ty_1 {
                    invariant: bool_to_ffi(*invariant),
                },
            },
        },
        naga::BuiltIn::ViewIndex => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_ViewIndex,
            data: default_data,
        },
        naga::BuiltIn::BaseInstance => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_BaseInstance,
            data: default_data,
        },
        naga::BuiltIn::BaseVertex => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_BaseVertex,
            data: default_data,
        },
        naga::BuiltIn::ClipDistance => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_ClipDistance,
            data: default_data,
        },
        naga::BuiltIn::CullDistance => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_CullDistance,
            data: default_data,
        },
        naga::BuiltIn::InstanceIndex => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_InstanceIndex,
            data: default_data,
        },
        naga::BuiltIn::PointSize => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_PointSize,
            data: default_data,
        },
        naga::BuiltIn::VertexIndex => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_VertexIndex,
            data: default_data,
        },
        naga::BuiltIn::DrawID => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_DrawID,
            data: default_data,
        },
        naga::BuiltIn::FragDepth => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_FragDepth,
            data: default_data,
        },
        naga::BuiltIn::PointCoord => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_PointCoord,
            data: default_data,
        },
        naga::BuiltIn::FrontFacing => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_FrontFacing,
            data: default_data,
        },
        naga::BuiltIn::PrimitiveIndex => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_PrimitiveIndex,
            data: default_data,
        },
        naga::BuiltIn::Barycentric => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_Barycentric,
            data: default_data,
        },
        naga::BuiltIn::SampleIndex => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_SampleIndex,
            data: default_data,
        },
        naga::BuiltIn::SampleMask => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_SampleMask,
            data: default_data,
        },
        naga::BuiltIn::GlobalInvocationId => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_GlobalInvocationId,
            data: default_data,
        },
        naga::BuiltIn::LocalInvocationId => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_LocalInvocationId,
            data: default_data,
        },
        naga::BuiltIn::LocalInvocationIndex => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_LocalInvocationIndex,
            data: default_data,
        },
        naga::BuiltIn::WorkGroupId => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_WorkGroupId,
            data: default_data,
        },
        naga::BuiltIn::WorkGroupSize => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_WorkGroupSize,
            data: default_data,
        },
        naga::BuiltIn::NumWorkGroups => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_NumWorkGroups,
            data: default_data,
        },
        naga::BuiltIn::NumSubgroups => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_NumSubgroups,
            data: default_data,
        },
        naga::BuiltIn::SubgroupId => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_SubgroupId,
            data: default_data,
        },
        naga::BuiltIn::SubgroupSize => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_SubgroupSize,
            data: default_data,
        },
        naga::BuiltIn::SubgroupInvocationId => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_SubgroupInvocationId,
            data: default_data,
        },
        naga::BuiltIn::MeshTaskSize => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_MeshTaskSize,
            data: default_data,
        },
        naga::BuiltIn::CullPrimitive => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_CullPrimitive,
            data: default_data,
        },
        naga::BuiltIn::PointIndex => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_PointIndex,
            data: default_data,
        },
        naga::BuiltIn::LineIndices => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_LineIndices,
            data: default_data,
        },
        naga::BuiltIn::TriangleIndices => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_TriangleIndices,
            data: default_data,
        },
        naga::BuiltIn::VertexCount => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_VertexCount,
            data: default_data,
        },
        naga::BuiltIn::Vertices => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_Vertices,
            data: default_data,
        },
        naga::BuiltIn::PrimitiveCount => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_PrimitiveCount,
            data: default_data,
        },
        naga::BuiltIn::Primitives => ffi::BuiltIn {
            tag: ffi::BuiltInTag_BuiltInTag_Primitives,
            data: default_data,
        },
    }
}

pub fn binding_to_ffi(binding: &naga::ir::Binding) -> ffi::Binding {
    match binding {
        naga::Binding::BuiltIn(built_in) => ffi::Binding {
            tag: ffi::BindingTag_BindingTag_BuiltIn,
            data: ffi::Binding__bindgen_ty_1 {
                built_in: built_in_to_ffi(built_in),
            },
        },
        naga::Binding::Location {
            location,
            interpolation,
            sampling,
            blend_src,
            per_primitive,
        } => ffi::Binding {
            tag: ffi::BindingTag_BindingTag_Location,
            data: ffi::Binding__bindgen_ty_1 {
                location: ffi::Binding__bindgen_ty_1__bindgen_ty_1 {
                    location: *location,
                    interpolation: interpolation
                        .as_ref()
                        .map(interpolation_to_ffi)
                        .unwrap_or_default(),
                    sampling: sampling.as_ref().map(sampling_to_ffi).unwrap_or_default(),
                    blend_src: blend_src.as_ref().copied().unwrap_or_default(),
                    per_primitive: bool_to_ffi(*per_primitive),
                },
            },
        },
    }
}

pub fn struct_member_to_ffi(struct_member: &naga::ir::StructMember) -> ffi::StructMember {
    ffi::StructMember {
        name: struct_member
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        ty: struct_member.ty.index(),
        binding: ffi::StructMember__bindgen_ty_1 {
            some: bool_to_ffi(struct_member.binding.is_some()),
            value: struct_member
                .binding
                .as_ref()
                .map(binding_to_ffi)
                .unwrap_or_default(),
        },
        offset: struct_member.offset,
    }
}

pub fn image_dimension_to_ffi(image_dimension: &naga::ir::ImageDimension) -> ffi::ImageDimension {
    match image_dimension {
        naga::ImageDimension::D1 => ffi::ImageDimension_D1,
        naga::ImageDimension::D2 => ffi::ImageDimension_D2,
        naga::ImageDimension::D3 => ffi::ImageDimension_D3,
        naga::ImageDimension::Cube => ffi::ImageDimension_Cube,
    }
}

pub fn type_inner_to_ffi(type_inner: &naga::ir::TypeInner) -> ffi::TypeInner {
    match type_inner {
        naga::TypeInner::Scalar(scalar) => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_Scalar,
            data: ffi::TypeInner__bindgen_ty_1 {
                scalar: scalar_to_ffi(scalar),
            },
        },
        naga::TypeInner::Vector { size, scalar } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_Vector,
            data: ffi::TypeInner__bindgen_ty_1 {
                vector: ffi::TypeInner__bindgen_ty_1__bindgen_ty_1 {
                    size: vector_size_to_ffi(size),
                    scalar: scalar_to_ffi(scalar),
                },
            },
        },
        naga::TypeInner::Matrix {
            columns,
            rows,
            scalar,
        } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_Matrix,
            data: ffi::TypeInner__bindgen_ty_1 {
                matrix: ffi::TypeInner__bindgen_ty_1__bindgen_ty_2 {
                    columns: vector_size_to_ffi(columns),
                    rows: vector_size_to_ffi(rows),
                    scalar: scalar_to_ffi(scalar),
                },
            },
        },
        naga::TypeInner::Atomic(scalar) => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_Atomic,
            data: ffi::TypeInner__bindgen_ty_1 {
                atomic: scalar_to_ffi(scalar),
            },
        },
        naga::TypeInner::Pointer { base, space } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_Pointer,
            data: ffi::TypeInner__bindgen_ty_1 {
                pointer: ffi::TypeInner__bindgen_ty_1__bindgen_ty_3 {
                    base: base.index(),
                    space: address_space_to_ffi(space),
                },
            },
        },
        naga::TypeInner::ValuePointer {
            size,
            scalar,
            space,
        } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_ValuePointer,
            data: ffi::TypeInner__bindgen_ty_1 {
                value_pointer: ffi::TypeInner__bindgen_ty_1__bindgen_ty_4 {
                    size: size
                        .as_ref()
                        .map(|s| vector_size_to_ffi(s))
                        .unwrap_or_default(),
                    scalar: scalar_to_ffi(scalar),
                    space: address_space_to_ffi(space),
                },
            },
        },
        naga::TypeInner::Array { base, size, stride } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_Array,
            data: ffi::TypeInner__bindgen_ty_1 {
                array: ffi::TypeInner__bindgen_ty_1__bindgen_ty_5 {
                    base: base.index(),
                    size: array_size_to_ffi(size),
                    stride: *stride,
                },
            },
        },
        naga::TypeInner::Struct { members, span } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_Struct,
            data: ffi::TypeInner__bindgen_ty_1 {
                struct_: ffi::TypeInner__bindgen_ty_1__bindgen_ty_6 {
                    members: unsafe { slice_to_ffi(members.as_slice(), struct_member_to_ffi) },
                    members_len: members.len(),
                    span: *span,
                },
            },
        },
        naga::TypeInner::Image {
            dim,
            arrayed,
            class,
        } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_Image,
            data: ffi::TypeInner__bindgen_ty_1 {
                image: ffi::TypeInner__bindgen_ty_1__bindgen_ty_7 {
                    dim: image_dimension_to_ffi(dim),
                    arrayed: bool_to_ffi(*arrayed),
                    class_: image_class_to_ffi(class),
                },
            },
        },
        naga::TypeInner::Sampler { comparison } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_Sampler,
            data: ffi::TypeInner__bindgen_ty_1 {
                sampler: ffi::TypeInner__bindgen_ty_1__bindgen_ty_8 {
                    comparison: bool_to_ffi(*comparison),
                },
            },
        },
        naga::TypeInner::AccelerationStructure { vertex_return } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_AccelerationStructure,
            data: ffi::TypeInner__bindgen_ty_1 {
                acceleration_structure: ffi::TypeInner__bindgen_ty_1__bindgen_ty_9 {
                    vertex_return: bool_to_ffi(*vertex_return),
                },
            },
        },
        naga::TypeInner::RayQuery { vertex_return } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_RayQuery,
            data: ffi::TypeInner__bindgen_ty_1 {
                ray_query: ffi::TypeInner__bindgen_ty_1__bindgen_ty_10 {
                    vertex_return: bool_to_ffi(*vertex_return),
                },
            },
        },
        naga::TypeInner::BindingArray { base, size } => ffi::TypeInner {
            tag: ffi::TypeInnerTag_TypeInnerTag_BindingArray,
            data: ffi::TypeInner__bindgen_ty_1 {
                binding_array: ffi::TypeInner__bindgen_ty_1__bindgen_ty_11 {
                    base: base.index(),
                    size: array_size_to_ffi(size),
                },
            },
        },
    }
}

pub fn type_to_ffi(ty: &naga::ir::Type) -> ffi::Type {
    ffi::Type {
        name: ty
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        inner: type_inner_to_ffi(&ty.inner),
    }
}

pub fn constant_to_ffi(constant: &naga::ir::Constant) -> ffi::Constant {
    ffi::Constant {
        name: constant
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        ty: constant.ty.index(),
        init: EMPTY_MUT,
    }
}

pub fn override_to_ffi(override_: &naga::ir::Override) -> ffi::Override {
    ffi::Override {
        name: override_
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        id: ffi::Override__bindgen_ty_1 {
            some: bool_to_ffi(override_.id.is_some()),
            value: override_.id.unwrap_or_default(),
        },
        ty: override_.ty.index(),
        init: EMPTY_MUT,
    }
}

pub fn resource_binding_to_ffi(
    resource_binding: &naga::ir::ResourceBinding,
) -> ffi::ResourceBinding {
    ffi::ResourceBinding {
        group: resource_binding.group,
        binding: resource_binding.binding,
    }
}

pub fn global_variable_to_ffi(global_variable: &naga::ir::GlobalVariable) -> ffi::GlobalVariable {
    ffi::GlobalVariable {
        name: global_variable
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        space: address_space_to_ffi(&global_variable.space),
        binding: ffi::GlobalVariable__bindgen_ty_1 {
            some: bool_to_ffi(global_variable.binding.is_some()),
            value: global_variable
                .binding
                .as_ref()
                .map(resource_binding_to_ffi)
                .unwrap_or_default(),
        },
        ty: global_variable.ty.index(),
        init: EMPTY_MUT,
    }
}

pub fn conservative_depth_to_ffi(
    conservative_depth: &naga::ir::ConservativeDepth,
) -> ffi::ConservativeDepth {
    match conservative_depth {
        naga::ConservativeDepth::GreaterEqual => {
            ffi::ConservativeDepth_ConservativeDepth_GreaterEqual
        }
        naga::ConservativeDepth::LessEqual => ffi::ConservativeDepth_ConservativeDepth_LessEqual,
        naga::ConservativeDepth::Unchanged => ffi::ConservativeDepth_ConservativeDepth_Unchanged,
    }
}

pub fn early_depth_test_to_ffi(early_depth_test: &naga::ir::EarlyDepthTest) -> ffi::EarlyDepthTest {
    match early_depth_test {
        naga::EarlyDepthTest::Force => ffi::EarlyDepthTest {
            tag: ffi::EarlyDepthTestTag_EarlyDepthTestTag_Force,
            data: ffi::EarlyDepthTest__bindgen_ty_1 {
                allow: ffi::EarlyDepthTest__bindgen_ty_1__bindgen_ty_1 { conservative: 0 },
            },
        },
        naga::EarlyDepthTest::Allow { conservative } => ffi::EarlyDepthTest {
            tag: ffi::EarlyDepthTestTag_EarlyDepthTestTag_Allow,
            data: ffi::EarlyDepthTest__bindgen_ty_1 {
                allow: ffi::EarlyDepthTest__bindgen_ty_1__bindgen_ty_1 {
                    conservative: conservative_depth_to_ffi(conservative),
                },
            },
        },
    }
}

pub fn shader_stage_to_ffi(shader_stage: &naga::ir::ShaderStage) -> ffi::ShaderStage {
    match shader_stage {
        naga::ShaderStage::Vertex => ffi::ShaderStage_ShaderStage_Vertex,
        naga::ShaderStage::Task => ffi::ShaderStage_ShaderStage_Task,
        naga::ShaderStage::Mesh => ffi::ShaderStage_ShaderStage_Mesh,
        naga::ShaderStage::Fragment => ffi::ShaderStage_ShaderStage_Fragment,
        naga::ShaderStage::Compute => ffi::ShaderStage_ShaderStage_Compute,
    }
}

pub fn entry_point_to_ffi(entry_point: &naga::ir::EntryPoint) -> ffi::EntryPoint {
    ffi::EntryPoint {
        name: unsafe { string_to_ffi(&entry_point.name) },
        stage: shader_stage_to_ffi(&entry_point.stage),
        early_depth_test: ffi::EntryPoint__bindgen_ty_1 {
            some: bool_to_ffi(entry_point.early_depth_test.is_some()),
            value: entry_point
                .early_depth_test
                .map(|edt| early_depth_test_to_ffi(&edt))
                .unwrap_or_default(),
        },
        workgroup_size: entry_point.workgroup_size,
        workgroup_size_overrides: [EMPTY_MUT; 3],
        function: EMPTY_MUT,
        mesh_info: EMPTY_MUT,
        task_payload: EMPTY_MUT,
    }
}

pub fn module_to_ffi(module: naga::ir::Module, flags: ffi::ModuleFillFlags) -> ffi::Module {
    let use_types = (flags & ffi::ModuleFillFlags_ModuleFillFlags_Types) != 0;
    let use_constants = (flags & ffi::ModuleFillFlags_ModuleFillFlags_Constants) != 0;
    let use_overrides = (flags & ffi::ModuleFillFlags_ModuleFillFlags_Overrides) != 0;
    let use_global_variables = (flags & ffi::ModuleFillFlags_ModuleFillFlags_GlobalVariables) != 0;
    let use_entry_points = (flags & ffi::ModuleFillFlags_ModuleFillFlags_EntryPoints) != 0;

    let module = Box::new(module);
    let module = Box::leak(module);
    let module_ptr = module as *mut naga::ir::Module;

    unsafe {
        ffi::Module {
            _inner_module: module_ptr as *mut c_void,

            types: if use_types {
                unique_arena_to_ffi(&module.types, type_to_ffi)
            } else {
                Default::default()
            },
            types_len: if use_types { module.types.len() } else { 0 },
            special_types: EMPTY_MUT,
            constants: if use_constants {
                arena_to_ffi(&module.constants, constant_to_ffi)
            } else {
                Default::default()
            },
            constants_len: if use_constants {
                module.constants.len()
            } else {
                0
            },
            overrides: if use_overrides {
                arena_to_ffi(&module.overrides, override_to_ffi)
            } else {
                Default::default()
            },
            overrides_len: if use_overrides {
                module.overrides.len()
            } else {
                0
            },
            global_variables: if use_global_variables {
                arena_to_ffi(&module.global_variables, global_variable_to_ffi)
            } else {
                Default::default()
            },
            global_variables_len: if use_global_variables {
                module.global_variables.len()
            } else {
                0
            },
            global_expressions: EMPTY_MUT,
            global_expressions_len: 0,
            functions: EMPTY_MUT,
            functions_len: 0,
            entry_points: if use_entry_points {
                slice_to_ffi(module.entry_points.as_slice(), entry_point_to_ffi)
            } else {
                Default::default()
            },
            entry_points_len: if use_entry_points {
                module.entry_points.len()
            } else {
                0
            },
            diagnostic_filters: EMPTY_MUT,
            diagnostic_filters_len: 0,
            diagnostic_filter_leaf: EMPTY_MUT,
            doc_comments: EMPTY_MUT,
        }
    }
}

