use super::*;

pub fn pipeline_constant_error_to_ffi(
    error: &naga::back::pipeline_constants::PipelineConstantError,
) -> ffi::PipelineConstantError {
    match error {
        naga::back::pipeline_constants::PipelineConstantError::MissingValue(missing_value) => {
            ffi::PipelineConstantError {
                tag: ffi::PipelineConstantErrorTag_PipelineConstantErrorTag_MissingValue,
                data: ffi::PipelineConstantError__bindgen_ty_1 {
                    missing_value: unsafe { string_to_ffi(missing_value) },
                },
            }
        }
        naga::back::pipeline_constants::PipelineConstantError::SrcNeedsToBeFinite => {
            ffi::PipelineConstantError {
                tag: ffi::PipelineConstantErrorTag_PipelineConstantErrorTag_SrcNeedsToBeFinite,
                data: ffi::PipelineConstantError__bindgen_ty_1::default(),
            }
        }
        naga::back::pipeline_constants::PipelineConstantError::DstRangeTooSmall => {
            ffi::PipelineConstantError {
                tag: ffi::PipelineConstantErrorTag_PipelineConstantErrorTag_DstRangeTooSmall,
                data: ffi::PipelineConstantError__bindgen_ty_1::default(),
            }
        }
        naga::back::pipeline_constants::PipelineConstantError::ConstantEvaluatorError(
            constant_evaluator_error,
        ) => ffi::PipelineConstantError {
            tag: ffi::PipelineConstantErrorTag_PipelineConstantErrorTag_ConstantEvaluatorError,
            data: ffi::PipelineConstantError__bindgen_ty_1 {
                constant_evaluator_error: constant_evaluator_error_to_ffi(constant_evaluator_error),
            },
        },
        naga::back::pipeline_constants::PipelineConstantError::ValidationError(
            _validation_error,
        ) => ffi::PipelineConstantError {
            tag: ffi::PipelineConstantErrorTag_PipelineConstantErrorTag_ValidationError,
            data: ffi::PipelineConstantError__bindgen_ty_1 {
                validation_error: EMPTY_MUT,
            },
        },
        naga::back::pipeline_constants::PipelineConstantError::NegativeWorkgroupSize => {
            ffi::PipelineConstantError {
                tag: ffi::PipelineConstantErrorTag_PipelineConstantErrorTag_NegativeWorkgroupSize,
                data: ffi::PipelineConstantError__bindgen_ty_1::default(),
            }
        }
        naga::back::pipeline_constants::PipelineConstantError::NegativeMeshOutputMax => {
            ffi::PipelineConstantError {
                tag: ffi::PipelineConstantErrorTag_PipelineConstantErrorTag_NegativeMeshOutputMax,
                data: ffi::PipelineConstantError__bindgen_ty_1::default(),
            }
        }
    }
}
