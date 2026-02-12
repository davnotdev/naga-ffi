use super::*;

pub fn constant_evaluator_error_to_ffi(
    error: &naga::proc::ConstantEvaluatorError,
) -> ffi::ConstantEvaluatorError {
    let default_data = ffi::ConstantEvaluatorError__bindgen_ty_1::default();

    match error {
    naga::proc::ConstantEvaluatorError::FunctionArg => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_FunctionArg,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::GlobalVariable => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_GlobalVariable,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::LocalVariable => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_LocalVariable,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidArrayLengthArg => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidArrayLengthArg,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ArrayLengthDynamic => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_ArrayLengthDynamic,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ArrayLengthOverridden => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_ArrayLengthOverridden,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Call => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_Call,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::WorkGroupUniformLoadResult => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_WorkGroupUniformLoadResult,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Atomic => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_Atomic,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Derivative => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_Derivative,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Load => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_Load,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ImageExpression => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_ImageExpression,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::RayQueryExpression => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_RayQueryExpression,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SubgroupExpression => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SubgroupExpression,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidAccessBase => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidAccessBase,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidAccessIndex => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidAccessIndex,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidAccessIndexTy => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidAccessIndexTy,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ArrayLength => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_ArrayLength,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidCastArg { from, to } => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidCastArg,
        data: ffi::ConstantEvaluatorError__bindgen_ty_1 {
            invalid_cast_arg: ffi::ConstantEvaluatorError__bindgen_ty_1__bindgen_ty_1 {
                from: unsafe { string_to_ffi(from) } ,
                to: unsafe { string_to_ffi(to) },
            },
        },
    },
    naga::proc::ConstantEvaluatorError::InvalidUnaryOpArg => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidUnaryOpArg,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidBinaryOpArgs => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidBinaryOpArgs,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidMathArg => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidMathArg,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidMathArgCount(math_function, expected, actual) => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidMathArgCount,
        data: ffi::ConstantEvaluatorError__bindgen_ty_1 {
            invalid_math_arg_count: ffi::ConstantEvaluatorError__bindgen_ty_1__bindgen_ty_2 {
                function: math_function_to_ffi(math_function),
                expected: *expected,
                actual: *actual,
            },
        },
    },
    naga::proc::ConstantEvaluatorError::InvalidRelationalArg(_relational_function) => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidRelationalArg,
        data: ffi::ConstantEvaluatorError__bindgen_ty_1 {
            invalid_relational_arg: EMPTY_MUT
        },
    },
    naga::proc::ConstantEvaluatorError::InvalidClamp => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidClamp,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::InvalidVectorComposeLength { expected, actual } => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidVectorComposeLength,
        data: ffi::ConstantEvaluatorError__bindgen_ty_1 {
            invalid_vector_compose_length: ffi::ConstantEvaluatorError__bindgen_ty_1__bindgen_ty_3 {
                expected: *expected,
                actual: *actual,
            },
        },
    },
    naga::proc::ConstantEvaluatorError::InvalidVectorComposeComponent => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_InvalidVectorComposeComponent,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SplatScalarOnly => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SplatScalarOnly,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SwizzleVectorOnly => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SwizzleVectorOnly,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SwizzleOutOfBounds => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SwizzleOutOfBounds,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::TypeNotConstructible => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_TypeNotConstructible,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SubexpressionsAreNotConstant => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SubexpressionsAreNotConstant,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::NotImplemented(_) => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_NotImplemented,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Overflow(_) => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_Overflow,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::AutomaticConversionLossy { value, to_type } => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_AutomaticConversionLossy,
        data: ffi::ConstantEvaluatorError__bindgen_ty_1 {
            automatic_conversion_lossy: ffi::ConstantEvaluatorError__bindgen_ty_1__bindgen_ty_4 {
                value: unsafe { string_to_ffi(value) } ,
                to_type: unsafe { string_to_ffi(to_type) },
            },
        },
    },
    naga::proc::ConstantEvaluatorError::DivisionByZero => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_DivisionByZero,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::RemainderByZero => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_RemainderByZero,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::ShiftedMoreThan32Bits => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_ShiftedMoreThan32Bits,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::Literal(_literal_error) => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_Literal,
        data: ffi::ConstantEvaluatorError__bindgen_ty_1 {
            literal: EMPTY_MUT
        },
    },
    naga::proc::ConstantEvaluatorError::Override => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_Override,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::RuntimeExpr => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_RuntimeExpr,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::OverrideExpr => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_OverrideExpr,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SelectScalarConditionNotABool => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SelectScalarConditionNotABool,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SelectVecRejectAcceptSizeMismatch { reject, accept } => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SelectVecRejectAcceptSizeMismatch,
        data: ffi::ConstantEvaluatorError__bindgen_ty_1 {
            select_vec_reject_accept_size_mismatch: ffi::ConstantEvaluatorError__bindgen_ty_1__bindgen_ty_5 {
                reject: vector_size_to_ffi(reject),
                accept: vector_size_to_ffi(accept),
            },
        },
    },
    naga::proc::ConstantEvaluatorError::SelectConditionNotAVecBool => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SelectConditionNotAVecBool,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SelectConditionVecSizeMismatch => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SelectConditionVecSizeMismatch,
        data: default_data,
    },
    naga::proc::ConstantEvaluatorError::SelectAcceptRejectTypeMismatch => ffi::ConstantEvaluatorError {
        tag: ffi::ConstantEvaluatorErrorTag_ConstantEvaluatorErrorTag_SelectAcceptRejectTypeMismatch,
        data: default_data,
    },
}
}

pub fn bound_check_policy_to_ffi(policy: &naga::proc::BoundsCheckPolicy) -> ffi::BoundsCheckPolicy {
    match policy {
        naga::proc::BoundsCheckPolicy::Restrict => {
            ffi::BoundsCheckPolicy_BoundsCheckPolicy_Restrict
        }
        naga::proc::BoundsCheckPolicy::ReadZeroSkipWrite => {
            ffi::BoundsCheckPolicy_BoundsCheckPolicy_ReadZeroSkipWrite
        }
        naga::proc::BoundsCheckPolicy::Unchecked => {
            ffi::BoundsCheckPolicy_BoundsCheckPolicy_Unchecked
        }
    }
}

pub fn bound_check_policies_to_ffi(
    policies: &naga::proc::BoundsCheckPolicies,
) -> ffi::BoundsCheckPolicies {
    ffi::BoundsCheckPolicies {
        index: bound_check_policy_to_ffi(&policies.index),
        buffer: bound_check_policy_to_ffi(&policies.buffer),
        image_load: bound_check_policy_to_ffi(&policies.image_load),
        binding_array: bound_check_policy_to_ffi(&policies.binding_array),
    }
}

pub fn bound_check_policy_to_naga(policy: ffi::BoundsCheckPolicy) -> naga::proc::BoundsCheckPolicy {
    match policy {
        ffi::BoundsCheckPolicy_BoundsCheckPolicy_Restrict => {
            naga::proc::BoundsCheckPolicy::Restrict
        }
        ffi::BoundsCheckPolicy_BoundsCheckPolicy_ReadZeroSkipWrite => {
            naga::proc::BoundsCheckPolicy::ReadZeroSkipWrite
        }
        ffi::BoundsCheckPolicy_BoundsCheckPolicy_Unchecked => {
            naga::proc::BoundsCheckPolicy::Unchecked
        }
        _ => panic!("Unknown BoundsCheckPolicy"),
    }
}

pub fn bound_check_policies_to_naga(
    policies: &ffi::BoundsCheckPolicies,
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
) -> ffi::ResolveArraySizeError {
    match error {
        naga::proc::ResolveArraySizeError::ExpectedPositiveArrayLength => {
            ffi::ResolveArraySizeError_ResolveArraySizeError_ExpectedPositiveArrayLength
        }
        naga::proc::ResolveArraySizeError::NonConstArrayLength => {
            ffi::ResolveArraySizeError_ResolveArraySizeError_NonConstArrayLength
        }
    }
}
