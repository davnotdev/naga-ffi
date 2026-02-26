use super::*;

pub fn scalar_kind_to_ffi(scalar_kind: &naga::ir::ScalarKind) -> ffi::NagaScalarKind {
    match scalar_kind {
        naga::ScalarKind::Sint => ffi::NagaScalarKind_NagaScalarKind_Sint,
        naga::ScalarKind::Uint => ffi::NagaScalarKind_NagaScalarKind_Uint,
        naga::ScalarKind::Float => ffi::NagaScalarKind_NagaScalarKind_Float,
        naga::ScalarKind::Bool => ffi::NagaScalarKind_NagaScalarKind_Bool,
        naga::ScalarKind::AbstractInt => ffi::NagaScalarKind_NagaScalarKind_AbstractInt,
        naga::ScalarKind::AbstractFloat => ffi::NagaScalarKind_NagaScalarKind_AbstractFloat,
    }
}

pub fn scalar_to_ffi(scalar: &naga::ir::Scalar) -> ffi::NagaScalar {
    ffi::NagaScalar {
        kind: scalar_kind_to_ffi(&scalar.kind),
        width: scalar.width,
    }
}

pub fn vector_size_to_ffi(vector_size: &naga::ir::VectorSize) -> ffi::NagaVectorSize {
    match vector_size {
        naga::VectorSize::Bi => ffi::NagaVectorSize_NagaVectorSize_Bi,
        naga::VectorSize::Tri => ffi::NagaVectorSize_NagaVectorSize_Tri,
        naga::VectorSize::Quad => ffi::NagaVectorSize_NagaVectorSize_Quad,
    }
}

pub fn array_size_to_ffi(array_size: &naga::ir::ArraySize) -> ffi::NagaArraySize {
    match array_size {
        naga::ArraySize::Constant(non_zero) => ffi::NagaArraySize {
            tag: ffi::NagaArraySizeTag_NagaArraySizeTag_Constant,
            data: ffi::NagaArraySize__bindgen_ty_1 {
                constant: non_zero.get(),
            },
        },
        naga::ArraySize::Pending(handle) => ffi::NagaArraySize {
            tag: ffi::NagaArraySizeTag_NagaArraySizeTag_Pending,
            data: ffi::NagaArraySize__bindgen_ty_1 {
                pending: handle.index(),
            },
        },
        naga::ArraySize::Dynamic => ffi::NagaArraySize {
            tag: ffi::NagaArraySizeTag_NagaArraySizeTag_Dynamic,
            data: ffi::NagaArraySize__bindgen_ty_1 { constant: 0 },
        },
    }
}

pub fn storage_format_to_ffi(storage_format: &naga::ir::StorageFormat) -> ffi::NagaStorageFormat {
    match storage_format {
        naga::StorageFormat::R8Unorm => ffi::NagaStorageFormat_NagaStorageFormat_R8Unorm,
        naga::StorageFormat::R8Snorm => ffi::NagaStorageFormat_NagaStorageFormat_R8Snorm,
        naga::StorageFormat::R8Uint => ffi::NagaStorageFormat_NagaStorageFormat_R8Uint,
        naga::StorageFormat::R8Sint => ffi::NagaStorageFormat_NagaStorageFormat_R8Sint,
        naga::StorageFormat::R16Uint => ffi::NagaStorageFormat_NagaStorageFormat_R16Uint,
        naga::StorageFormat::R16Sint => ffi::NagaStorageFormat_NagaStorageFormat_R16Sint,
        naga::StorageFormat::R16Float => ffi::NagaStorageFormat_NagaStorageFormat_R16Float,
        naga::StorageFormat::Rg8Unorm => ffi::NagaStorageFormat_NagaStorageFormat_Rg8Unorm,
        naga::StorageFormat::Rg8Snorm => ffi::NagaStorageFormat_NagaStorageFormat_Rg8Snorm,
        naga::StorageFormat::Rg8Uint => ffi::NagaStorageFormat_NagaStorageFormat_Rg8Uint,
        naga::StorageFormat::Rg8Sint => ffi::NagaStorageFormat_NagaStorageFormat_Rg8Sint,
        naga::StorageFormat::R32Uint => ffi::NagaStorageFormat_NagaStorageFormat_R32Uint,
        naga::StorageFormat::R32Sint => ffi::NagaStorageFormat_NagaStorageFormat_R32Sint,
        naga::StorageFormat::R32Float => ffi::NagaStorageFormat_NagaStorageFormat_R32Float,
        naga::StorageFormat::Rg16Uint => ffi::NagaStorageFormat_NagaStorageFormat_Rg16Uint,
        naga::StorageFormat::Rg16Sint => ffi::NagaStorageFormat_NagaStorageFormat_Rg16Sint,
        naga::StorageFormat::Rg16Float => ffi::NagaStorageFormat_NagaStorageFormat_Rg16Float,
        naga::StorageFormat::Rgba8Unorm => ffi::NagaStorageFormat_NagaStorageFormat_Rgba8Unorm,
        naga::StorageFormat::Rgba8Snorm => ffi::NagaStorageFormat_NagaStorageFormat_Rgba8Snorm,
        naga::StorageFormat::Rgba8Uint => ffi::NagaStorageFormat_NagaStorageFormat_Rgba8Uint,
        naga::StorageFormat::Rgba8Sint => ffi::NagaStorageFormat_NagaStorageFormat_Rgba8Sint,
        naga::StorageFormat::Bgra8Unorm => ffi::NagaStorageFormat_NagaStorageFormat_Bgra8Unorm,
        naga::StorageFormat::Rgb10a2Uint => ffi::NagaStorageFormat_NagaStorageFormat_Rgb10a2Uint,
        naga::StorageFormat::Rgb10a2Unorm => ffi::NagaStorageFormat_NagaStorageFormat_Rgb10a2Unorm,
        naga::StorageFormat::Rg11b10Ufloat => {
            ffi::NagaStorageFormat_NagaStorageFormat_Rg11b10Ufloat
        }
        naga::StorageFormat::R64Uint => ffi::NagaStorageFormat_NagaStorageFormat_R64Uint,
        naga::StorageFormat::Rg32Uint => ffi::NagaStorageFormat_NagaStorageFormat_Rg32Uint,
        naga::StorageFormat::Rg32Sint => ffi::NagaStorageFormat_NagaStorageFormat_Rg32Sint,
        naga::StorageFormat::Rg32Float => ffi::NagaStorageFormat_NagaStorageFormat_Rg32Float,
        naga::StorageFormat::Rgba16Uint => ffi::NagaStorageFormat_NagaStorageFormat_Rgba16Uint,
        naga::StorageFormat::Rgba16Sint => ffi::NagaStorageFormat_NagaStorageFormat_Rgba16Sint,
        naga::StorageFormat::Rgba16Float => ffi::NagaStorageFormat_NagaStorageFormat_Rgba16Float,
        naga::StorageFormat::Rgba32Uint => ffi::NagaStorageFormat_NagaStorageFormat_Rgba32Uint,
        naga::StorageFormat::Rgba32Sint => ffi::NagaStorageFormat_NagaStorageFormat_Rgba32Sint,
        naga::StorageFormat::Rgba32Float => ffi::NagaStorageFormat_NagaStorageFormat_Rgba32Float,
        naga::StorageFormat::R16Unorm => ffi::NagaStorageFormat_NagaStorageFormat_R16Unorm,
        naga::StorageFormat::R16Snorm => ffi::NagaStorageFormat_NagaStorageFormat_R16Snorm,
        naga::StorageFormat::Rg16Unorm => ffi::NagaStorageFormat_NagaStorageFormat_Rg16Unorm,
        naga::StorageFormat::Rg16Snorm => ffi::NagaStorageFormat_NagaStorageFormat_Rg16Snorm,
        naga::StorageFormat::Rgba16Unorm => ffi::NagaStorageFormat_NagaStorageFormat_Rgba16Unorm,
        naga::StorageFormat::Rgba16Snorm => ffi::NagaStorageFormat_NagaStorageFormat_Rgba16Snorm,
    }
}

pub fn storage_access_to_ffi(
    storage_access: &naga::ir::StorageAccess,
) -> ffi::NagaStorageAccessFlags {
    let mut result: ffi::NagaStorageAccessFlags = 0;
    if storage_access.contains(naga::ir::StorageAccess::LOAD) {
        result |= ffi::NagaStorageAccess_NagaStorageAccess_LOAD;
    }
    if storage_access.contains(naga::ir::StorageAccess::STORE) {
        result |= ffi::NagaStorageAccess_NagaStorageAccess_STORE;
    }
    if storage_access.contains(naga::ir::StorageAccess::ATOMIC) {
        result |= ffi::NagaStorageAccess_NagaStorageAccess_ATOMIC;
    }
    sa::const_assert_eq!(
        naga::ir::StorageAccess::all().bits(),
        naga::ir::StorageAccess::LOAD.bits()
            | naga::ir::StorageAccess::STORE.bits()
            | naga::ir::StorageAccess::ATOMIC.bits()
    );

    result
}

pub fn relational_function_to_ffi(
    func: naga::ir::RelationalFunction,
) -> ffi::NagaRelationalFunction {
    match func {
        naga::RelationalFunction::All => ffi::NagaRelationalFunction_NagaRelationalFunction_All,
        naga::RelationalFunction::Any => ffi::NagaRelationalFunction_NagaRelationalFunction_Any,
        naga::RelationalFunction::IsNan => ffi::NagaRelationalFunction_NagaRelationalFunction_IsNan,
        naga::RelationalFunction::IsInf => ffi::NagaRelationalFunction_NagaRelationalFunction_IsInf,
    }
}

pub fn image_class_to_ffi(image_class: &naga::ir::ImageClass) -> ffi::NagaImageClass {
    match image_class {
        naga::ImageClass::Sampled { kind, multi } => ffi::NagaImageClass {
            tag: ffi::NagaImageClassTag_NagaImageClassTag_Sampled,
            data: ffi::NagaImageClass__bindgen_ty_1 {
                sampled: ffi::NagaImageClass__bindgen_ty_1__bindgen_ty_1 {
                    kind: scalar_kind_to_ffi(kind),
                    multi: bool_to_ffi(*multi),
                },
            },
        },
        naga::ImageClass::Depth { multi } => ffi::NagaImageClass {
            tag: ffi::NagaImageClassTag_NagaImageClassTag_Depth,
            data: ffi::NagaImageClass__bindgen_ty_1 {
                depth: ffi::NagaImageClass__bindgen_ty_1__bindgen_ty_2 {
                    multi: bool_to_ffi(*multi),
                },
            },
        },
        naga::ImageClass::External => ffi::NagaImageClass {
            tag: ffi::NagaImageClassTag_NagaImageClassTag_External,
            data: ffi::NagaImageClass__bindgen_ty_1 {
                storage: ffi::NagaImageClass__bindgen_ty_1__bindgen_ty_3 {
                    format: 0,
                    access: 0,
                },
            },
        },
        naga::ImageClass::Storage { format, access } => ffi::NagaImageClass {
            tag: ffi::NagaImageClassTag_NagaImageClassTag_Storage,
            data: ffi::NagaImageClass__bindgen_ty_1 {
                storage: ffi::NagaImageClass__bindgen_ty_1__bindgen_ty_3 {
                    format: storage_format_to_ffi(format),
                    access: storage_access_to_ffi(access),
                },
            },
        },
    }
}

pub fn address_space_to_ffi(address_space: &naga::ir::AddressSpace) -> ffi::NagaAddressSpace {
    let default_data = ffi::NagaAddressSpace__bindgen_ty_1 {
        storage: ffi::NagaAddressSpace__bindgen_ty_1__bindgen_ty_1 { access: 0 },
    };
    match address_space {
        naga::AddressSpace::Function => ffi::NagaAddressSpace {
            tag: ffi::NagaAddressSpaceTag_NagaAddressSpaceTag_Function,
            data: default_data,
        },
        naga::AddressSpace::Private => ffi::NagaAddressSpace {
            tag: ffi::NagaAddressSpaceTag_NagaAddressSpaceTag_Private,
            data: default_data,
        },
        naga::AddressSpace::WorkGroup => ffi::NagaAddressSpace {
            tag: ffi::NagaAddressSpaceTag_NagaAddressSpaceTag_WorkGroup,
            data: default_data,
        },
        naga::AddressSpace::Uniform => ffi::NagaAddressSpace {
            tag: ffi::NagaAddressSpaceTag_NagaAddressSpaceTag_Uniform,
            data: default_data,
        },
        naga::AddressSpace::Storage { access } => ffi::NagaAddressSpace {
            tag: ffi::NagaAddressSpaceTag_NagaAddressSpaceTag_Storage,
            data: ffi::NagaAddressSpace__bindgen_ty_1 {
                storage: ffi::NagaAddressSpace__bindgen_ty_1__bindgen_ty_1 {
                    access: storage_access_to_ffi(access),
                },
            },
        },
        naga::AddressSpace::Handle => ffi::NagaAddressSpace {
            tag: ffi::NagaAddressSpaceTag_NagaAddressSpaceTag_Handle,
            data: default_data,
        },
        naga::AddressSpace::Immediate => ffi::NagaAddressSpace {
            tag: ffi::NagaAddressSpaceTag_NagaAddressSpaceTag_Immediate,
            data: default_data,
        },
        naga::AddressSpace::TaskPayload => ffi::NagaAddressSpace {
            tag: ffi::NagaAddressSpaceTag_NagaAddressSpaceTag_TaskPayload,
            data: default_data,
        },
    }
}

pub fn math_function_to_ffi(math_function: &naga::ir::MathFunction) -> ffi::NagaMathFunction {
    match math_function {
        naga::MathFunction::Abs => ffi::NagaMathFunction_NagaMathFunction_Abs,
        naga::MathFunction::Min => ffi::NagaMathFunction_NagaMathFunction_Min,
        naga::MathFunction::Max => ffi::NagaMathFunction_NagaMathFunction_Max,
        naga::MathFunction::Clamp => ffi::NagaMathFunction_NagaMathFunction_Clamp,
        naga::MathFunction::Saturate => ffi::NagaMathFunction_NagaMathFunction_Saturate,
        naga::MathFunction::Cos => ffi::NagaMathFunction_NagaMathFunction_Cos,
        naga::MathFunction::Cosh => ffi::NagaMathFunction_NagaMathFunction_Cosh,
        naga::MathFunction::Sin => ffi::NagaMathFunction_NagaMathFunction_Sin,
        naga::MathFunction::Sinh => ffi::NagaMathFunction_NagaMathFunction_Sinh,
        naga::MathFunction::Tan => ffi::NagaMathFunction_NagaMathFunction_Tan,
        naga::MathFunction::Tanh => ffi::NagaMathFunction_NagaMathFunction_Tanh,
        naga::MathFunction::Acos => ffi::NagaMathFunction_NagaMathFunction_Acos,
        naga::MathFunction::Asin => ffi::NagaMathFunction_NagaMathFunction_Asin,
        naga::MathFunction::Atan => ffi::NagaMathFunction_NagaMathFunction_Atan,
        naga::MathFunction::Atan2 => ffi::NagaMathFunction_NagaMathFunction_Atan2,
        naga::MathFunction::Asinh => ffi::NagaMathFunction_NagaMathFunction_Asinh,
        naga::MathFunction::Acosh => ffi::NagaMathFunction_NagaMathFunction_Acosh,
        naga::MathFunction::Atanh => ffi::NagaMathFunction_NagaMathFunction_Atanh,
        naga::MathFunction::Radians => ffi::NagaMathFunction_NagaMathFunction_Radians,
        naga::MathFunction::Degrees => ffi::NagaMathFunction_NagaMathFunction_Degrees,
        naga::MathFunction::Ceil => ffi::NagaMathFunction_NagaMathFunction_Ceil,
        naga::MathFunction::Floor => ffi::NagaMathFunction_NagaMathFunction_Floor,
        naga::MathFunction::Round => ffi::NagaMathFunction_NagaMathFunction_Round,
        naga::MathFunction::Fract => ffi::NagaMathFunction_NagaMathFunction_Fract,
        naga::MathFunction::Trunc => ffi::NagaMathFunction_NagaMathFunction_Trunc,
        naga::MathFunction::Modf => ffi::NagaMathFunction_NagaMathFunction_Modf,
        naga::MathFunction::Frexp => ffi::NagaMathFunction_NagaMathFunction_Frexp,
        naga::MathFunction::Ldexp => ffi::NagaMathFunction_NagaMathFunction_Ldexp,
        naga::MathFunction::Exp => ffi::NagaMathFunction_NagaMathFunction_Exp,
        naga::MathFunction::Exp2 => ffi::NagaMathFunction_NagaMathFunction_Exp2,
        naga::MathFunction::Log => ffi::NagaMathFunction_NagaMathFunction_Log,
        naga::MathFunction::Log2 => ffi::NagaMathFunction_NagaMathFunction_Log2,
        naga::MathFunction::Pow => ffi::NagaMathFunction_NagaMathFunction_Pow,
        naga::MathFunction::Dot => ffi::NagaMathFunction_NagaMathFunction_Dot,
        naga::MathFunction::Dot4I8Packed => ffi::NagaMathFunction_NagaMathFunction_Dot4I8Packed,
        naga::MathFunction::Dot4U8Packed => ffi::NagaMathFunction_NagaMathFunction_Dot4U8Packed,
        naga::MathFunction::Outer => ffi::NagaMathFunction_NagaMathFunction_Outer,
        naga::MathFunction::Cross => ffi::NagaMathFunction_NagaMathFunction_Cross,
        naga::MathFunction::Distance => ffi::NagaMathFunction_NagaMathFunction_Distance,
        naga::MathFunction::Length => ffi::NagaMathFunction_NagaMathFunction_Length,
        naga::MathFunction::Normalize => ffi::NagaMathFunction_NagaMathFunction_Normalize,
        naga::MathFunction::FaceForward => ffi::NagaMathFunction_NagaMathFunction_FaceForward,
        naga::MathFunction::Reflect => ffi::NagaMathFunction_NagaMathFunction_Reflect,
        naga::MathFunction::Refract => ffi::NagaMathFunction_NagaMathFunction_Refract,
        naga::MathFunction::Sign => ffi::NagaMathFunction_NagaMathFunction_Sign,
        naga::MathFunction::Fma => ffi::NagaMathFunction_NagaMathFunction_Fma,
        naga::MathFunction::Mix => ffi::NagaMathFunction_NagaMathFunction_Mix,
        naga::MathFunction::Step => ffi::NagaMathFunction_NagaMathFunction_Step,
        naga::MathFunction::SmoothStep => ffi::NagaMathFunction_NagaMathFunction_SmoothStep,
        naga::MathFunction::Sqrt => ffi::NagaMathFunction_NagaMathFunction_Sqrt,
        naga::MathFunction::InverseSqrt => ffi::NagaMathFunction_NagaMathFunction_InverseSqrt,
        naga::MathFunction::Inverse => ffi::NagaMathFunction_NagaMathFunction_Inverse,
        naga::MathFunction::Transpose => ffi::NagaMathFunction_NagaMathFunction_Transpose,
        naga::MathFunction::Determinant => ffi::NagaMathFunction_NagaMathFunction_Determinant,
        naga::MathFunction::QuantizeToF16 => ffi::NagaMathFunction_NagaMathFunction_QuantizeToF16,
        naga::MathFunction::CountTrailingZeros => {
            ffi::NagaMathFunction_NagaMathFunction_CountTrailingZeros
        }
        naga::MathFunction::CountLeadingZeros => {
            ffi::NagaMathFunction_NagaMathFunction_CountLeadingZeros
        }
        naga::MathFunction::CountOneBits => ffi::NagaMathFunction_NagaMathFunction_CountOneBits,
        naga::MathFunction::ReverseBits => ffi::NagaMathFunction_NagaMathFunction_ReverseBits,
        naga::MathFunction::ExtractBits => ffi::NagaMathFunction_NagaMathFunction_ExtractBits,
        naga::MathFunction::InsertBits => ffi::NagaMathFunction_NagaMathFunction_InsertBits,
        naga::MathFunction::FirstTrailingBit => {
            ffi::NagaMathFunction_NagaMathFunction_FirstTrailingBit
        }
        naga::MathFunction::FirstLeadingBit => {
            ffi::NagaMathFunction_NagaMathFunction_FirstLeadingBit
        }
        naga::MathFunction::Pack4x8snorm => ffi::NagaMathFunction_NagaMathFunction_Pack4x8snorm,
        naga::MathFunction::Pack4x8unorm => ffi::NagaMathFunction_NagaMathFunction_Pack4x8unorm,
        naga::MathFunction::Pack2x16snorm => ffi::NagaMathFunction_NagaMathFunction_Pack2x16snorm,
        naga::MathFunction::Pack2x16unorm => ffi::NagaMathFunction_NagaMathFunction_Pack2x16unorm,
        naga::MathFunction::Pack2x16float => ffi::NagaMathFunction_NagaMathFunction_Pack2x16float,
        naga::MathFunction::Pack4xI8 => ffi::NagaMathFunction_NagaMathFunction_Pack4xI8,
        naga::MathFunction::Pack4xU8 => ffi::NagaMathFunction_NagaMathFunction_Pack4xU8,
        naga::MathFunction::Pack4xI8Clamp => ffi::NagaMathFunction_NagaMathFunction_Pack4xI8Clamp,
        naga::MathFunction::Pack4xU8Clamp => ffi::NagaMathFunction_NagaMathFunction_Pack4xU8Clamp,
        naga::MathFunction::Unpack4x8snorm => ffi::NagaMathFunction_NagaMathFunction_Unpack4x8snorm,
        naga::MathFunction::Unpack4x8unorm => ffi::NagaMathFunction_NagaMathFunction_Unpack4x8unorm,
        naga::MathFunction::Unpack2x16snorm => {
            ffi::NagaMathFunction_NagaMathFunction_Unpack2x16snorm
        }
        naga::MathFunction::Unpack2x16unorm => {
            ffi::NagaMathFunction_NagaMathFunction_Unpack2x16unorm
        }
        naga::MathFunction::Unpack2x16float => {
            ffi::NagaMathFunction_NagaMathFunction_Unpack2x16float
        }
        naga::MathFunction::Unpack4xI8 => ffi::NagaMathFunction_NagaMathFunction_Unpack4xI8,
        naga::MathFunction::Unpack4xU8 => ffi::NagaMathFunction_NagaMathFunction_Unpack4xU8,
    }
}

pub fn interpolation_to_ffi(interpolation: &naga::ir::Interpolation) -> ffi::NagaInterpolation {
    match interpolation {
        naga::Interpolation::Perspective => ffi::NagaInterpolation_NagaInterpolation_Perspective,
        naga::Interpolation::Linear => ffi::NagaInterpolation_NagaInterpolation_Linear,
        naga::Interpolation::Flat => ffi::NagaInterpolation_NagaInterpolation_Flat,
    }
}

pub fn sampling_to_ffi(sampling: &naga::ir::Sampling) -> ffi::NagaSampling {
    match sampling {
        naga::Sampling::Center => ffi::NagaSampling_NagaSampling_Center,
        naga::Sampling::Centroid => ffi::NagaSampling_NagaSampling_Centroid,
        naga::Sampling::Sample => ffi::NagaSampling_NagaSampling_Sample,
        naga::Sampling::First => ffi::NagaSampling_NagaSampling_First,
        naga::Sampling::Either => ffi::NagaSampling_NagaSampling_Either,
    }
}

pub fn built_in_to_ffi(built_in: &naga::ir::BuiltIn) -> ffi::NagaBuiltIn {
    let default_data = ffi::NagaBuiltIn__bindgen_ty_1 {
        position: ffi::NagaBuiltIn__bindgen_ty_1__bindgen_ty_1 { invariant: 0 },
    };

    match built_in {
        naga::BuiltIn::Position { invariant } => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_Position,
            data: ffi::NagaBuiltIn__bindgen_ty_1 {
                position: ffi::NagaBuiltIn__bindgen_ty_1__bindgen_ty_1 {
                    invariant: bool_to_ffi(*invariant),
                },
            },
        },
        naga::BuiltIn::ViewIndex => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_ViewIndex,
            data: default_data,
        },
        naga::BuiltIn::BaseInstance => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_BaseInstance,
            data: default_data,
        },
        naga::BuiltIn::BaseVertex => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_BaseVertex,
            data: default_data,
        },
        naga::BuiltIn::ClipDistance => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_ClipDistance,
            data: default_data,
        },
        naga::BuiltIn::CullDistance => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_CullDistance,
            data: default_data,
        },
        naga::BuiltIn::InstanceIndex => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_InstanceIndex,
            data: default_data,
        },
        naga::BuiltIn::PointSize => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_PointSize,
            data: default_data,
        },
        naga::BuiltIn::VertexIndex => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_VertexIndex,
            data: default_data,
        },
        naga::BuiltIn::DrawID => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_DrawID,
            data: default_data,
        },
        naga::BuiltIn::FragDepth => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_FragDepth,
            data: default_data,
        },
        naga::BuiltIn::PointCoord => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_PointCoord,
            data: default_data,
        },
        naga::BuiltIn::FrontFacing => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_FrontFacing,
            data: default_data,
        },
        naga::BuiltIn::PrimitiveIndex => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_PrimitiveIndex,
            data: default_data,
        },
        naga::BuiltIn::Barycentric => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_Barycentric,
            data: default_data,
        },
        naga::BuiltIn::SampleIndex => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_SampleIndex,
            data: default_data,
        },
        naga::BuiltIn::SampleMask => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_SampleMask,
            data: default_data,
        },
        naga::BuiltIn::GlobalInvocationId => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_GlobalInvocationId,
            data: default_data,
        },
        naga::BuiltIn::LocalInvocationId => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_LocalInvocationId,
            data: default_data,
        },
        naga::BuiltIn::LocalInvocationIndex => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_LocalInvocationIndex,
            data: default_data,
        },
        naga::BuiltIn::WorkGroupId => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_WorkGroupId,
            data: default_data,
        },
        naga::BuiltIn::WorkGroupSize => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_WorkGroupSize,
            data: default_data,
        },
        naga::BuiltIn::NumWorkGroups => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_NumWorkGroups,
            data: default_data,
        },
        naga::BuiltIn::NumSubgroups => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_NumSubgroups,
            data: default_data,
        },
        naga::BuiltIn::SubgroupId => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_SubgroupId,
            data: default_data,
        },
        naga::BuiltIn::SubgroupSize => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_SubgroupSize,
            data: default_data,
        },
        naga::BuiltIn::SubgroupInvocationId => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_SubgroupInvocationId,
            data: default_data,
        },
        naga::BuiltIn::MeshTaskSize => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_MeshTaskSize,
            data: default_data,
        },
        naga::BuiltIn::CullPrimitive => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_CullPrimitive,
            data: default_data,
        },
        naga::BuiltIn::PointIndex => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_PointIndex,
            data: default_data,
        },
        naga::BuiltIn::LineIndices => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_LineIndices,
            data: default_data,
        },
        naga::BuiltIn::TriangleIndices => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_TriangleIndices,
            data: default_data,
        },
        naga::BuiltIn::VertexCount => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_VertexCount,
            data: default_data,
        },
        naga::BuiltIn::Vertices => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_Vertices,
            data: default_data,
        },
        naga::BuiltIn::PrimitiveCount => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_PrimitiveCount,
            data: default_data,
        },
        naga::BuiltIn::Primitives => ffi::NagaBuiltIn {
            tag: ffi::NagaBuiltInTag_NagaBuiltInTag_Primitives,
            data: default_data,
        },
    }
}

pub fn binding_to_ffi(binding: &naga::ir::Binding) -> ffi::NagaBinding {
    match binding {
        naga::Binding::BuiltIn(built_in) => ffi::NagaBinding {
            tag: ffi::NagaBindingTag_NagaBindingTag_BuiltIn,
            data: ffi::NagaBinding__bindgen_ty_1 {
                built_in: built_in_to_ffi(built_in),
            },
        },
        naga::Binding::Location {
            location,
            interpolation,
            sampling,
            blend_src,
            per_primitive,
        } => ffi::NagaBinding {
            tag: ffi::NagaBindingTag_NagaBindingTag_Location,
            data: ffi::NagaBinding__bindgen_ty_1 {
                location: ffi::NagaBinding__bindgen_ty_1__bindgen_ty_1 {
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

pub fn struct_member_to_ffi(struct_member: &naga::ir::StructMember) -> ffi::NagaStructMember {
    ffi::NagaStructMember {
        name: struct_member
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        ty: struct_member.ty.index(),
        binding: ffi::NagaStructMember__bindgen_ty_1 {
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

pub fn image_dimension_to_ffi(
    image_dimension: &naga::ir::ImageDimension,
) -> ffi::NagaImageDimension {
    match image_dimension {
        naga::ImageDimension::D1 => ffi::NagaImageDimension_NagaImageDimension_D1,
        naga::ImageDimension::D2 => ffi::NagaImageDimension_NagaImageDimension_D2,
        naga::ImageDimension::D3 => ffi::NagaImageDimension_NagaImageDimension_D3,
        naga::ImageDimension::Cube => ffi::NagaImageDimension_NagaImageDimension_Cube,
    }
}

pub fn type_inner_to_ffi(type_inner: &naga::ir::TypeInner) -> ffi::NagaTypeInner {
    match type_inner {
        naga::TypeInner::Scalar(scalar) => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_Scalar,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                scalar: scalar_to_ffi(scalar),
            },
        },
        naga::TypeInner::Vector { size, scalar } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_Vector,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                vector: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_1 {
                    size: vector_size_to_ffi(size),
                    scalar: scalar_to_ffi(scalar),
                },
            },
        },
        naga::TypeInner::Matrix {
            columns,
            rows,
            scalar,
        } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_Matrix,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                matrix: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_2 {
                    columns: vector_size_to_ffi(columns),
                    rows: vector_size_to_ffi(rows),
                    scalar: scalar_to_ffi(scalar),
                },
            },
        },
        naga::TypeInner::Atomic(scalar) => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_Atomic,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                atomic: scalar_to_ffi(scalar),
            },
        },
        naga::TypeInner::Pointer { base, space } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_Pointer,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                pointer: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_3 {
                    base: base.index(),
                    space: address_space_to_ffi(space),
                },
            },
        },
        naga::TypeInner::ValuePointer {
            size,
            scalar,
            space,
        } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_ValuePointer,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                value_pointer: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_4 {
                    size: size.as_ref().map(vector_size_to_ffi).unwrap_or_default(),
                    scalar: scalar_to_ffi(scalar),
                    space: address_space_to_ffi(space),
                },
            },
        },
        naga::TypeInner::Array { base, size, stride } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_Array,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                array: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_5 {
                    base: base.index(),
                    size: array_size_to_ffi(size),
                    stride: *stride,
                },
            },
        },
        naga::TypeInner::Struct { members, span } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_Struct,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                struct_: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_6 {
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
        } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_Image,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                image: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_7 {
                    dim: image_dimension_to_ffi(dim),
                    arrayed: bool_to_ffi(*arrayed),
                    class_: image_class_to_ffi(class),
                },
            },
        },
        naga::TypeInner::Sampler { comparison } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_Sampler,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                sampler: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_8 {
                    comparison: bool_to_ffi(*comparison),
                },
            },
        },
        naga::TypeInner::AccelerationStructure { vertex_return } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_AccelerationStructure,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                acceleration_structure: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_9 {
                    vertex_return: bool_to_ffi(*vertex_return),
                },
            },
        },
        naga::TypeInner::RayQuery { vertex_return } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_RayQuery,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                ray_query: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_10 {
                    vertex_return: bool_to_ffi(*vertex_return),
                },
            },
        },
        naga::TypeInner::BindingArray { base, size } => ffi::NagaTypeInner {
            tag: ffi::NagaTypeInnerTag_NagaTypeInnerTag_BindingArray,
            data: ffi::NagaTypeInner__bindgen_ty_1 {
                binding_array: ffi::NagaTypeInner__bindgen_ty_1__bindgen_ty_11 {
                    base: base.index(),
                    size: array_size_to_ffi(size),
                },
            },
        },
    }
}

pub fn type_to_ffi(ty: &naga::ir::Type) -> ffi::NagaType {
    ffi::NagaType {
        name: ty
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        inner: type_inner_to_ffi(&ty.inner),
    }
}

pub fn constant_to_ffi(constant: &naga::ir::Constant) -> ffi::NagaConstant {
    ffi::NagaConstant {
        name: constant
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        ty: constant.ty.index(),
        init: EMPTY_MUT,
    }
}

pub fn override_to_ffi(override_: &naga::ir::Override) -> ffi::NagaOverride {
    ffi::NagaOverride {
        name: override_
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        id: ffi::NagaOverride__bindgen_ty_1 {
            some: bool_to_ffi(override_.id.is_some()),
            value: override_.id.unwrap_or_default(),
        },
        ty: override_.ty.index(),
        init: EMPTY_MUT,
    }
}

pub fn resource_binding_to_ffi(
    resource_binding: &naga::ir::ResourceBinding,
) -> ffi::NagaResourceBinding {
    ffi::NagaResourceBinding {
        group: resource_binding.group,
        binding: resource_binding.binding,
    }
}

pub fn resource_binding_to_naga(
    resource_binding: &ffi::NagaResourceBinding,
) -> naga::ir::ResourceBinding {
    naga::ir::ResourceBinding {
        group: resource_binding.group,
        binding: resource_binding.binding,
    }
}

pub fn global_variable_to_ffi(
    global_variable: &naga::ir::GlobalVariable,
) -> ffi::NagaGlobalVariable {
    ffi::NagaGlobalVariable {
        name: global_variable
            .name
            .as_ref()
            .map(|s| unsafe { string_to_ffi(s) })
            .unwrap_or_default(),
        space: address_space_to_ffi(&global_variable.space),
        binding: ffi::NagaGlobalVariable__bindgen_ty_1 {
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
) -> ffi::NagaConservativeDepth {
    match conservative_depth {
        naga::ConservativeDepth::GreaterEqual => {
            ffi::NagaConservativeDepth_NagaConservativeDepth_GreaterEqual
        }
        naga::ConservativeDepth::LessEqual => {
            ffi::NagaConservativeDepth_NagaConservativeDepth_LessEqual
        }
        naga::ConservativeDepth::Unchanged => {
            ffi::NagaConservativeDepth_NagaConservativeDepth_Unchanged
        }
    }
}

pub fn early_depth_test_to_ffi(
    early_depth_test: &naga::ir::EarlyDepthTest,
) -> ffi::NagaEarlyDepthTest {
    match early_depth_test {
        naga::EarlyDepthTest::Force => ffi::NagaEarlyDepthTest {
            tag: ffi::NagaEarlyDepthTestTag_NagaEarlyDepthTestTag_Force,
            data: ffi::NagaEarlyDepthTest__bindgen_ty_1 {
                allow: ffi::NagaEarlyDepthTest__bindgen_ty_1__bindgen_ty_1 { conservative: 0 },
            },
        },
        naga::EarlyDepthTest::Allow { conservative } => ffi::NagaEarlyDepthTest {
            tag: ffi::NagaEarlyDepthTestTag_NagaEarlyDepthTestTag_Allow,
            data: ffi::NagaEarlyDepthTest__bindgen_ty_1 {
                allow: ffi::NagaEarlyDepthTest__bindgen_ty_1__bindgen_ty_1 {
                    conservative: conservative_depth_to_ffi(conservative),
                },
            },
        },
    }
}

pub fn shader_stage_to_ffi(shader_stage: &naga::ir::ShaderStage) -> ffi::NagaShaderStage {
    match shader_stage {
        naga::ShaderStage::Vertex => ffi::NagaShaderStage_NagaShaderStage_Vertex,
        naga::ShaderStage::Task => ffi::NagaShaderStage_NagaShaderStage_Task,
        naga::ShaderStage::Mesh => ffi::NagaShaderStage_NagaShaderStage_Mesh,
        naga::ShaderStage::Fragment => ffi::NagaShaderStage_NagaShaderStage_Fragment,
        naga::ShaderStage::Compute => ffi::NagaShaderStage_NagaShaderStage_Compute,
    }
}

pub fn shader_stage_to_naga(shader_stage: &ffi::NagaShaderStage) -> naga::ShaderStage {
    match *shader_stage {
        ffi::NagaShaderStage_NagaShaderStage_Vertex => naga::ShaderStage::Vertex,
        ffi::NagaShaderStage_NagaShaderStage_Task => naga::ShaderStage::Task,
        ffi::NagaShaderStage_NagaShaderStage_Mesh => naga::ShaderStage::Mesh,
        ffi::NagaShaderStage_NagaShaderStage_Fragment => naga::ShaderStage::Fragment,
        ffi::NagaShaderStage_NagaShaderStage_Compute => naga::ShaderStage::Compute,
        _ => panic!("Unknown ShaderStage"),
    }
}

pub fn entry_point_to_ffi(entry_point: &naga::ir::EntryPoint) -> ffi::NagaEntryPoint {
    ffi::NagaEntryPoint {
        name: unsafe { string_to_ffi(&entry_point.name) },
        stage: shader_stage_to_ffi(&entry_point.stage),
        early_depth_test: ffi::NagaEntryPoint__bindgen_ty_1 {
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

pub fn module_to_ffi(module: naga::ir::Module, flags: ffi::NagaModuleFillFlags) -> ffi::NagaModule {
    let use_types = (flags & ffi::NagaModuleFill_NagaModuleFill_Types) != 0;
    let use_constants = (flags & ffi::NagaModuleFill_NagaModuleFill_Constants) != 0;
    let use_overrides = (flags & ffi::NagaModuleFill_NagaModuleFill_Overrides) != 0;
    let use_global_variables = (flags & ffi::NagaModuleFill_NagaModuleFill_GlobalVariables) != 0;
    let use_entry_points = (flags & ffi::NagaModuleFill_NagaModuleFill_EntryPoints) != 0;

    let module = Box::new(module);
    let module = Box::leak(module);
    let module_ptr = module as *mut naga::ir::Module;

    unsafe {
        ffi::NagaModule {
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
