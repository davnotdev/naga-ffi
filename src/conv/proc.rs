use super::*;

pub fn constant_evaluator_error_to_ffi(
    error: &naga::proc::ConstantEvaluatorError,
) -> ffi::NagaConstantEvaluatorError {
    let default_data = ffi::NagaConstantEvaluatorError__bindgen_ty_1::default();

    match error {
    naga::proc::ConstantEvaluatorError::FunctionArg => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_FunctionArg,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::GlobalVariable => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_GlobalVariable,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::LocalVariable => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_LocalVariable,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidArrayLengthArg => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidArrayLengthArg,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ArrayLengthDynamic => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_ArrayLengthDynamic,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ArrayLengthOverridden => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_ArrayLengthOverridden,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Call => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_Call,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::WorkGroupUniformLoadResult => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_WorkGroupUniformLoadResult,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Atomic => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_Atomic,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Derivative => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_Derivative,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Load => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_Load,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ImageExpression => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_ImageExpression,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::RayQueryExpression => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_RayQueryExpression,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SubgroupExpression => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SubgroupExpression,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidAccessBase => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidAccessBase,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidAccessIndex => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidAccessIndex,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidAccessIndexTy => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidAccessIndexTy,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ArrayLength => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_ArrayLength,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidCastArg { from, to } => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidCastArg,
        data: ffi::NagaConstantEvaluatorError__bindgen_ty_1 {
            invalid_cast_arg: ffi::NagaConstantEvaluatorError__bindgen_ty_1__bindgen_ty_1 {
                from: unsafe { string_to_ffi(from) } ,
                to: unsafe { string_to_ffi(to) },
            },
        },
    },
    naga::proc::ConstantEvaluatorError::InvalidUnaryOpArg => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidUnaryOpArg,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidBinaryOpArgs => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidBinaryOpArgs,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidMathArg => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidMathArg,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidMathArgCount(math_function, expected, actual) => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidMathArgCount,
        data: ffi::NagaConstantEvaluatorError__bindgen_ty_1 {
            invalid_math_arg_count: ffi::NagaConstantEvaluatorError__bindgen_ty_1__bindgen_ty_2 {
                function: math_function_to_ffi(math_function),
                expected: *expected,
                actual: *actual,
            },
        },
    },
    naga::proc::ConstantEvaluatorError::InvalidRelationalArg(_relational_function) => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidRelationalArg,
        data: ffi::NagaConstantEvaluatorError__bindgen_ty_1 {
            invalid_relational_arg: EMPTY_MUT
        },
    },
    naga::proc::ConstantEvaluatorError::InvalidClamp => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidClamp,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidVectorComposeLength { expected, actual } => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidVectorComposeLength,
        data: ffi::NagaConstantEvaluatorError__bindgen_ty_1 {
            invalid_vector_compose_length: ffi::NagaConstantEvaluatorError__bindgen_ty_1__bindgen_ty_3 {
                expected: *expected,
                actual: *actual,
            },
        },
    },
    naga::proc::ConstantEvaluatorError::InvalidVectorComposeComponent => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_InvalidVectorComposeComponent,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SplatScalarOnly => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SplatScalarOnly,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SwizzleVectorOnly => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SwizzleVectorOnly,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SwizzleOutOfBounds => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SwizzleOutOfBounds,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::TypeNotConstructible => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_TypeNotConstructible,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SubexpressionsAreNotConstant => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SubexpressionsAreNotConstant,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::NotImplemented(_) => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_NotImplemented,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Overflow(_) => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_Overflow,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::AutomaticConversionLossy { value, to_type } => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_AutomaticConversionLossy,
        data: ffi::NagaConstantEvaluatorError__bindgen_ty_1 {
            automatic_conversion_lossy: ffi::NagaConstantEvaluatorError__bindgen_ty_1__bindgen_ty_4 {
                value: unsafe { string_to_ffi(value) } ,
                to_type: unsafe { string_to_ffi(to_type) },
            },
        },
    },
    naga::proc::ConstantEvaluatorError::DivisionByZero => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_DivisionByZero,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::RemainderByZero => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_RemainderByZero,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ShiftedMoreThan32Bits => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_ShiftedMoreThan32Bits,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Literal(_literal_error) => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_Literal,
        data: ffi::NagaConstantEvaluatorError__bindgen_ty_1 {
            literal: EMPTY_MUT
        },
    },
    naga::proc::ConstantEvaluatorError::Override => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_Override,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::RuntimeExpr => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_RuntimeExpr,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::OverrideExpr => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_OverrideExpr,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SelectScalarConditionNotABool => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SelectScalarConditionNotABool,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SelectVecRejectAcceptSizeMismatch { reject, accept } => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SelectVecRejectAcceptSizeMismatch,
        data: ffi::NagaConstantEvaluatorError__bindgen_ty_1 {
            select_vec_reject_accept_size_mismatch: ffi::NagaConstantEvaluatorError__bindgen_ty_1__bindgen_ty_5 {
                reject: vector_size_to_ffi(reject),
                accept: vector_size_to_ffi(accept),
            },
        },
    },
    naga::proc::ConstantEvaluatorError::SelectConditionNotAVecBool => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SelectConditionNotAVecBool,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SelectConditionVecSizeMismatch => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SelectConditionVecSizeMismatch,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SelectAcceptRejectTypeMismatch => ffi::NagaConstantEvaluatorError {
        tag: ffi::NagaConstantEvaluatorErrorTag_NagaConstantEvaluatorErrorTag_SelectAcceptRejectTypeMismatch,
        data: default_data,
    },
}
}

pub fn bound_check_policy_to_ffi(
    policy: &naga::proc::BoundsCheckPolicy,
) -> ffi::NagaBoundsCheckPolicy {
    match policy {
        naga::proc::BoundsCheckPolicy::Restrict => {
            ffi::NagaBoundsCheckPolicy_NagaBoundsCheckPolicy_Restrict
        }
        naga::proc::BoundsCheckPolicy::ReadZeroSkipWrite => {
            ffi::NagaBoundsCheckPolicy_NagaBoundsCheckPolicy_ReadZeroSkipWrite
        }
        naga::proc::BoundsCheckPolicy::Unchecked => {
            ffi::NagaBoundsCheckPolicy_NagaBoundsCheckPolicy_Unchecked
        }
    }
}

pub fn bound_check_policies_to_ffi(
    policies: &naga::proc::BoundsCheckPolicies,
) -> ffi::NagaBoundsCheckPolicies {
    ffi::NagaBoundsCheckPolicies {
        index: bound_check_policy_to_ffi(&policies.index),
        buffer: bound_check_policy_to_ffi(&policies.buffer),
        image_load: bound_check_policy_to_ffi(&policies.image_load),
        binding_array: bound_check_policy_to_ffi(&policies.binding_array),
    }
}

pub fn bound_check_policy_to_naga(
    policy: ffi::NagaBoundsCheckPolicy,
) -> naga::proc::BoundsCheckPolicy {
    match policy {
        ffi::NagaBoundsCheckPolicy_NagaBoundsCheckPolicy_Restrict => {
            naga::proc::BoundsCheckPolicy::Restrict
        }
        ffi::NagaBoundsCheckPolicy_NagaBoundsCheckPolicy_ReadZeroSkipWrite => {
            naga::proc::BoundsCheckPolicy::ReadZeroSkipWrite
        }
        ffi::NagaBoundsCheckPolicy_NagaBoundsCheckPolicy_Unchecked => {
            naga::proc::BoundsCheckPolicy::Unchecked
        }
        _ => panic!("Unknown BoundsCheckPolicy"),
    }
}

pub fn bound_check_policies_to_naga(
    policies: &ffi::NagaBoundsCheckPolicies,
) -> naga::proc::BoundsCheckPolicies {
    naga::proc::BoundsCheckPolicies {
        index: bound_check_policy_to_naga(policies.index),
        buffer: bound_check_policy_to_naga(policies.buffer),
        image_load: bound_check_policy_to_naga(policies.image_load),
        binding_array: bound_check_policy_to_naga(policies.binding_array),
    }
}

pub fn resolve_array_size_error_to_ffi(
    error: &naga::proc::ResolveArraySizeError,
) -> ffi::NagaResolveArraySizeError {
    match error {
        naga::proc::ResolveArraySizeError::ExpectedPositiveArrayLength => {
            ffi::NagaResolveArraySizeError_NagaResolveArraySizeError_ExpectedPositiveArrayLength
        }
        naga::proc::ResolveArraySizeError::NonConstArrayLength => {
            ffi::NagaResolveArraySizeError_NagaResolveArraySizeError_NonConstArrayLength
        }
    }
}
