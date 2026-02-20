use super::*;

pub fn spv_front_options_to_naga(options: &ffi::SPVFrontOptions) -> naga::front::spv::Options {
    naga::front::spv::Options {
        adjust_coordinate_space: bool_to_naga(options.adjust_coordinate_space),
        strict_capabilities: bool_to_naga(options.strict_capabilities),
        block_ctx_dump_prefix: if options.block_ctx_dump_prefix.is_null() {
            None
        } else {
            Some(string_to_naga(options.block_ctx_dump_prefix))
        },
    }
}

pub fn spv_front_module_state_to_ffi(
    state: &naga::front::spv::ModuleState,
) -> ffi::SPVFrontModuleState {
    match state {
        naga::front::spv::ModuleState::Empty => ffi::SPVFrontModuleState_SPVFrontModuleState_Empty,
        naga::front::spv::ModuleState::Capability => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_Capability
        }
        naga::front::spv::ModuleState::Extension => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_Extension
        }
        naga::front::spv::ModuleState::ExtInstImport => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_ExtInstImport
        }
        naga::front::spv::ModuleState::MemoryModel => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_MemoryModel
        }
        naga::front::spv::ModuleState::EntryPoint => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_EntryPoint
        }
        naga::front::spv::ModuleState::ExecutionMode => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_ExecutionMode
        }
        naga::front::spv::ModuleState::Source => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_Source
        }
        naga::front::spv::ModuleState::Name => ffi::SPVFrontModuleState_SPVFrontModuleState_Name,
        naga::front::spv::ModuleState::ModuleProcessed => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_ModuleProcessed
        }
        naga::front::spv::ModuleState::Annotation => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_Annotation
        }
        naga::front::spv::ModuleState::Type => ffi::SPVFrontModuleState_SPVFrontModuleState_Type,
        naga::front::spv::ModuleState::Function => {
            ffi::SPVFrontModuleState_SPVFrontModuleState_Function
        }
    }
}

pub fn spv_front_error_to_ffi(error: &naga::front::spv::Error) -> ffi::SPVFrontError {
    let default_data = ffi::SPVFrontError__bindgen_ty_1::default();

    match error {
        naga::front::spv::Error::InvalidHeader => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidHeader,
            data: default_data,
        },
        naga::front::spv::Error::InvalidWordCount => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidWordCount,
            data: default_data,
        },
        naga::front::spv::Error::UnknownInstruction(instruction) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnknownInstruction,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unknown_instruction: *instruction,
            },
        },
        naga::front::spv::Error::UnknownCapability(word) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnknownCapability,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unknown_capability: *word,
            },
        },
        naga::front::spv::Error::UnsupportedInstruction(module_state, op) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedInstruction,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_instruction: ffi::SPVFrontError__bindgen_ty_1__bindgen_ty_1 {
                    module_state: spv_front_module_state_to_ffi(module_state),
                    op: *op as u32 as u16,
                },
            },
        },
        naga::front::spv::Error::UnsupportedCapability(capability) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedCapability,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_capability: *capability as u32,
            },
        },
        naga::front::spv::Error::UnsupportedExtension(extension) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedExtension,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_extension: unsafe { string_to_ffi(extension) },
            },
        },
        naga::front::spv::Error::UnsupportedExtSet(ext_set) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedExtSet,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_ext_set: unsafe { string_to_ffi(ext_set) },
            },
        },
        naga::front::spv::Error::UnsupportedExtInstSet(ext_inst_set) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedExtInstSet,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_ext_inst_set: *ext_inst_set,
            },
        },
        naga::front::spv::Error::UnsupportedExtInst(ext_inst) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedExtInst,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_ext_inst: *ext_inst,
            },
        },
        naga::front::spv::Error::UnsupportedType(handle) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedType,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_type: handle.index(),
            },
        },
        naga::front::spv::Error::UnsupportedExecutionModel(module) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedExecutionModel,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_execution_model: *module,
            },
        },
        naga::front::spv::Error::UnsupportedExecutionMode(mode) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedExecutionMode,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_execution_mode: *mode,
            },
        },
        naga::front::spv::Error::UnsupportedStorageClass(class) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedStorageClass,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_storage_class: *class,
            },
        },
        naga::front::spv::Error::UnsupportedImageDim(dim) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedImageDim,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_image_dim: *dim,
            },
        },
        naga::front::spv::Error::UnsupportedImageFormat(format) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedImageFormat,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_image_format: *format,
            },
        },
        naga::front::spv::Error::UnsupportedBuiltIn(built_in) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedBuiltIn,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_built_in: *built_in,
            },
        },
        naga::front::spv::Error::UnsupportedControlFlow(control_flow) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedControlFlow,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_control_flow: *control_flow,
            },
        },
        naga::front::spv::Error::UnsupportedBinaryOperator(op) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedBinaryOperator,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_binary_operator: *op,
            },
        },
        naga::front::spv::Error::UnsupportedRuntimeArrayStorageClass => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedRuntimeArrayStorageClass,
            data: default_data,
        },
        naga::front::spv::Error::UnsupportedMatrixStride {
            stride,
            columns,
            rows,
            width,
        } => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedMatrixStride,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unsupported_matrix_stride: ffi::SPVFrontError__bindgen_ty_1__bindgen_ty_2 {
                    stride: *stride,
                    columns: *columns,
                    rows: *rows,
                    width: *width,
                },
            },
        },
        naga::front::spv::Error::UnknownBinaryOperator(op) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnknownBinaryOperator,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unknown_binary_operator: *op as u32 as u16,
            },
        },
        naga::front::spv::Error::UnknownRelationalFunction(op) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnknownRelationalFunction,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                unknown_relational_function: *op as u32 as u16,
            },
        },
        naga::front::spv::Error::UnsupportedGroupOperation(_) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_UnsupportedGroupOperation,
            data: default_data,
        },
        naga::front::spv::Error::InvalidParameter(op) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidParameter,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_parameter: *op as u32 as u16,
            },
        },
        naga::front::spv::Error::InvalidOperandCount(op, count) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidOperandCount,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_operand_count: ffi::SPVFrontError__bindgen_ty_1__bindgen_ty_3 {
                    op: *op as u32 as u16,
                    count: *count,
                },
            },
        },
        naga::front::spv::Error::InvalidOperand => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidOperand,
            data: default_data,
        },
        naga::front::spv::Error::InvalidId(invalid_id) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidId,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_id: *invalid_id,
            },
        },
        naga::front::spv::Error::InvalidDecoration(decoration) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidDecoration,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_decoration: *decoration,
            },
        },
        naga::front::spv::Error::InvalidTypeWidth(width) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidTypeWidth,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_type_width: *width,
            },
        },
        naga::front::spv::Error::InvalidSign(sign) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidSign,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_sign: *sign,
            },
        },
        naga::front::spv::Error::InvalidInnerType(inner_type) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidInnerType,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_inner_type: *inner_type,
            },
        },
        naga::front::spv::Error::InvalidVectorSize(size) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidVectorSize,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_vector_size: *size,
            },
        },
        naga::front::spv::Error::InvalidAccessType(type_) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidAccessType,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_access_type: *type_,
            },
        },
        naga::front::spv::Error::InvalidAccess(_expression) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidAccess,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_access: EMPTY_MUT,
            },
        },
        naga::front::spv::Error::InvalidAccessIndex(index) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidAccessIndex,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_access_index: *index,
            },
        },
        naga::front::spv::Error::InvalidIndexType(type_) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidIndexType,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_index_type: *type_,
            },
        },
        naga::front::spv::Error::InvalidBinding(binding) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidBinding,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_binding: *binding,
            },
        },
        naga::front::spv::Error::InvalidGlobalVar(_expression) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidGlobalVar,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_global_var: EMPTY_MUT,
            },
        },
        naga::front::spv::Error::InvalidImageExpression(_expression) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidImageExpression,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_image_expression: EMPTY_MUT,
            },
        },
        naga::front::spv::Error::InvalidImageWriteType => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidImageWriteType,
            data: default_data,
        },
        naga::front::spv::Error::InvalidImageBaseType(handle) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidImageBaseType,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_image_base_type: handle.index(),
            },
        },
        naga::front::spv::Error::InvalidImage(handle) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidImage,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_image: handle.index(),
            },
        },
        naga::front::spv::Error::InvalidAsType(handle) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidAsType,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_as_type: handle.index(),
            },
        },
        naga::front::spv::Error::InvalidVectorType(handle) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidVectorType,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_vector_type: handle.index(),
            },
        },
        naga::front::spv::Error::InconsistentComparisonSampling(handle) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InconsistentComparisonSampling,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                inconsistent_comparison_sampling: handle.index(),
            },
        },
        naga::front::spv::Error::WrongFunctionResultType(type_) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_WrongFunctionResultType,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                wrong_function_result_type: *type_,
            },
        },
        naga::front::spv::Error::WrongFunctionArgumentType(type_) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_WrongFunctionArgumentType,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                wrong_function_argument_type: *type_,
            },
        },
        naga::front::spv::Error::MissingDecoration(decoration) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_MissingDecoration,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                missing_decoration: *decoration as u32,
            },
        },
        naga::front::spv::Error::BadString => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_BadString,
            data: default_data,
        },
        naga::front::spv::Error::IncompleteData => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_IncompleteData,
            data: default_data,
        },
        naga::front::spv::Error::InvalidTerminator => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidTerminator,
            data: default_data,
        },
        naga::front::spv::Error::InvalidEdgeClassification => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidEdgeClassification,
            data: default_data,
        },
        naga::front::spv::Error::ControlFlowGraphCycle(id) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_ControlFlowGraphCycle,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                control_flow_graph_cycle: *id,
            },
        },
        naga::front::spv::Error::FunctionCallCycle(word) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_FunctionCallCycle,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                function_call_cycle: *word,
            },
        },
        naga::front::spv::Error::InvalidArraySize(word) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidArraySize,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_array_size: *word,
            },
        },
        naga::front::spv::Error::InvalidBarrierScope(word) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidBarrierScope,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_barrier_scope: *word,
            },
        },
        naga::front::spv::Error::InvalidBarrierMemorySemantics(word) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_InvalidBarrierMemorySemantics,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                invalid_barrier_memory_semantics: *word,
            },
        },
        naga::front::spv::Error::NonBindingArrayOfImageOrSamplers => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_NonBindingArrayOfImageOrSamplers,
            data: default_data,
        },
        naga::front::spv::Error::SpecIdTooHigh(spec_id) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_SpecIdTooHigh,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                spec_id_too_high: *spec_id,
            },
        },
        naga::front::spv::Error::AtomicUpgradeError(error) => ffi::SPVFrontError {
            tag: ffi::SPVFrontErrorTag_SPVFrontErrorTag_AtomicUpgradeError,
            data: ffi::SPVFrontError__bindgen_ty_1 {
                atomic_upgrade_error: atmoic_upgrade_front_error_to_ffi(error),
            },
        },
    }
}
