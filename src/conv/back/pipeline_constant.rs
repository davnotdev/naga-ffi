use super::*;

pub fn pipeline_constant_error_to_ffi(
    error: &naga::back::pipeline_constants::PipelineConstantError,
) -> ffi::NagaPipelineConstantError {
    match error {
        naga::back::pipeline_constants::PipelineConstantError::MissingValue(missing_value) => {
            ffi::NagaPipelineConstantError {
                tag: ffi::NagaPipelineConstantErrorTag_NagaPipelineConstantErrorTag_MissingValue,
                data: ffi::NagaPipelineConstantError__bindgen_ty_1 {
                    missing_value: unsafe { string_to_ffi(missing_value) },
                },
            }
        }
        naga::back::pipeline_constants::PipelineConstantError::SrcNeedsToBeFinite => {
            ffi::NagaPipelineConstantError {
                tag: ffi::NagaPipelineConstantErrorTag_NagaPipelineConstantErrorTag_SrcNeedsToBeFinite,
                data: ffi::NagaPipelineConstantError__bindgen_ty_1::default(),
            }
        }
        naga::back::pipeline_constants::PipelineConstantError::DstRangeTooSmall => {
            ffi::NagaPipelineConstantError {
                tag: ffi::NagaPipelineConstantErrorTag_NagaPipelineConstantErrorTag_DstRangeTooSmall,
                data: ffi::NagaPipelineConstantError__bindgen_ty_1::default(),
            }
        }
        naga::back::pipeline_constants::PipelineConstantError::ConstantEvaluatorError(
            constant_evaluator_error,
        ) => ffi::NagaPipelineConstantError {
            tag: ffi::NagaPipelineConstantErrorTag_NagaPipelineConstantErrorTag_ConstantEvaluatorError,
            data: ffi::NagaPipelineConstantError__bindgen_ty_1 {
                constant_evaluator_error: constant_evaluator_error_to_ffi(constant_evaluator_error),
            },
        },
        naga::back::pipeline_constants::PipelineConstantError::ValidationError(
            _validation_error,
        ) => ffi::NagaPipelineConstantError {
            tag: ffi::NagaPipelineConstantErrorTag_NagaPipelineConstantErrorTag_ValidationError,
            data: ffi::NagaPipelineConstantError__bindgen_ty_1 {
                validation_error: EMPTY_MUT,
            },
        },
        naga::back::pipeline_constants::PipelineConstantError::NegativeWorkgroupSize => {
            ffi::NagaPipelineConstantError {
                tag: ffi::NagaPipelineConstantErrorTag_NagaPipelineConstantErrorTag_NegativeWorkgroupSize,
                data: ffi::NagaPipelineConstantError__bindgen_ty_1::default(),
            }
        }
        naga::back::pipeline_constants::PipelineConstantError::NegativeMeshOutputMax => {
            ffi::NagaPipelineConstantError {
                tag: ffi::NagaPipelineConstantErrorTag_NagaPipelineConstantErrorTag_NegativeMeshOutputMax,
                data: ffi::NagaPipelineConstantError__bindgen_ty_1::default(),
            }
        }
    }
}
