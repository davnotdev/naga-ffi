use super::*;

pub fn spv_front_options_to_naga(options: &ffi::NagaSPVFrontOptions) -> naga::front::spv::Options {
    naga::front::spv::Options {
        adjust_coordinate_space: bool_to_naga(options.adjust_coordinate_space),
        strict_capabilities: bool_to_naga(options.strict_capabilities),
        block_ctx_dump_prefix: if options.block_ctx_dump_prefix.is_null() {
            None
        } else {
            Some(unsafe { string_to_naga(options.block_ctx_dump_prefix) })
        },
    }
}

pub fn spv_front_module_state_to_ffi(
    state: &naga::front::spv::ModuleState,
) -> ffi::NagaSPVFrontModuleState {
    match state {
        naga::front::spv::ModuleState::Empty => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_Empty
        }
        naga::front::spv::ModuleState::Capability => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_Capability
        }
        naga::front::spv::ModuleState::Extension => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_Extension
        }
        naga::front::spv::ModuleState::ExtInstImport => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_ExtInstImport
        }
        naga::front::spv::ModuleState::MemoryModel => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_MemoryModel
        }
        naga::front::spv::ModuleState::EntryPoint => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_EntryPoint
        }
        naga::front::spv::ModuleState::ExecutionMode => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_ExecutionMode
        }
        naga::front::spv::ModuleState::Source => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_Source
        }
        naga::front::spv::ModuleState::Name => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_Name
        }
        naga::front::spv::ModuleState::ModuleProcessed => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_ModuleProcessed
        }
        naga::front::spv::ModuleState::Annotation => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_Annotation
        }
        naga::front::spv::ModuleState::Type => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_Type
        }
        naga::front::spv::ModuleState::Function => {
            ffi::NagaSPVFrontModuleState_NagaSPVFrontModuleState_Function
        }
    }
}

pub fn spv_front_error_to_ffi(error: &naga::front::spv::Error) -> ffi::NagaSPVFrontError {
    let default_data = ffi::NagaSPVFrontError__bindgen_ty_1::default();

    match error {
        naga::front::spv::Error::InvalidHeader => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidHeader,
            data: default_data,
        },
        naga::front::spv::Error::InvalidWordCount => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidWordCount,
            data: default_data,
        },
        naga::front::spv::Error::UnknownInstruction(instruction) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnknownInstruction,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unknown_instruction: *instruction,
            },
        },
        naga::front::spv::Error::UnknownCapability(word) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnknownCapability,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unknown_capability: *word,
            },
        },
        naga::front::spv::Error::UnsupportedInstruction(module_state, op) => {
            ffi::NagaSPVFrontError {
                tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedInstruction,
                data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                    unsupported_instruction: ffi::NagaSPVFrontError__bindgen_ty_1__bindgen_ty_1 {
                        module_state: spv_front_module_state_to_ffi(module_state),
                        op: *op as u32 as u16,
                    },
                },
            }
        }
        naga::front::spv::Error::UnsupportedCapability(capability) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedCapability,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_capability: *capability as u32,
            },
        },
        naga::front::spv::Error::UnsupportedExtension(extension) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedExtension,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_extension: unsafe { string_to_ffi(extension) },
            },
        },
        naga::front::spv::Error::UnsupportedExtSet(ext_set) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedExtSet,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_ext_set: unsafe { string_to_ffi(ext_set) },
            },
        },
        naga::front::spv::Error::UnsupportedExtInstSet(ext_inst_set) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedExtInstSet,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_ext_inst_set: *ext_inst_set,
            },
        },
        naga::front::spv::Error::UnsupportedExtInst(ext_inst) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedExtInst,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_ext_inst: *ext_inst,
            },
        },
        naga::front::spv::Error::UnsupportedType(handle) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedType,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_type: handle.index(),
            },
        },
        naga::front::spv::Error::UnsupportedExecutionModel(module) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedExecutionModel,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_execution_model: *module,
            },
        },
        naga::front::spv::Error::UnsupportedExecutionMode(mode) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedExecutionMode,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_execution_mode: *mode,
            },
        },
        naga::front::spv::Error::UnsupportedStorageClass(class) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedStorageClass,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_storage_class: *class,
            },
        },
        naga::front::spv::Error::UnsupportedImageDim(dim) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedImageDim,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_image_dim: *dim,
            },
        },
        naga::front::spv::Error::UnsupportedImageFormat(format) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedImageFormat,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_image_format: *format,
            },
        },
        naga::front::spv::Error::UnsupportedBuiltIn(built_in) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedBuiltIn,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_built_in: *built_in,
            },
        },
        naga::front::spv::Error::UnsupportedControlFlow(control_flow) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedControlFlow,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_control_flow: *control_flow,
            },
        },
        naga::front::spv::Error::UnsupportedBinaryOperator(op) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedBinaryOperator,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_binary_operator: *op,
            },
        },
        naga::front::spv::Error::UnsupportedRuntimeArrayStorageClass => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedRuntimeArrayStorageClass,
            data: default_data,
        },
        naga::front::spv::Error::UnsupportedMatrixStride {
            stride,
            columns,
            rows,
            width,
        } => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedMatrixStride,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unsupported_matrix_stride: ffi::NagaSPVFrontError__bindgen_ty_1__bindgen_ty_2 {
                    stride: *stride,
                    columns: *columns,
                    rows: *rows,
                    width: *width,
                },
            },
        },
        naga::front::spv::Error::UnknownBinaryOperator(op) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnknownBinaryOperator,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unknown_binary_operator: *op as u32 as u16,
            },
        },
        naga::front::spv::Error::UnknownRelationalFunction(op) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnknownRelationalFunction,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                unknown_relational_function: *op as u32 as u16,
            },
        },
        naga::front::spv::Error::UnsupportedGroupOperation(_) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_UnsupportedGroupOperation,
            data: default_data,
        },
        naga::front::spv::Error::InvalidParameter(op) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidParameter,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_parameter: *op as u32 as u16,
            },
        },
        naga::front::spv::Error::InvalidOperandCount(op, count) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidOperandCount,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_operand_count: ffi::NagaSPVFrontError__bindgen_ty_1__bindgen_ty_3 {
                    op: *op as u32 as u16,
                    count: *count,
                },
            },
        },
        naga::front::spv::Error::InvalidOperand => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidOperand,
            data: default_data,
        },
        naga::front::spv::Error::InvalidId(invalid_id) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidId,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_id: *invalid_id,
            },
        },
        naga::front::spv::Error::InvalidDecoration(decoration) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidDecoration,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_decoration: *decoration,
            },
        },
        naga::front::spv::Error::InvalidTypeWidth(width) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidTypeWidth,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_type_width: *width,
            },
        },
        naga::front::spv::Error::InvalidSign(sign) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidSign,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_sign: *sign,
            },
        },
        naga::front::spv::Error::InvalidInnerType(inner_type) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidInnerType,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_inner_type: *inner_type,
            },
        },
        naga::front::spv::Error::InvalidVectorSize(size) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidVectorSize,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_vector_size: *size,
            },
        },
        naga::front::spv::Error::InvalidAccessType(type_) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidAccessType,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_access_type: *type_,
            },
        },
        naga::front::spv::Error::InvalidAccess(_expression) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidAccess,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_access: EMPTY_MUT,
            },
        },
        naga::front::spv::Error::InvalidAccessIndex(index) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidAccessIndex,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_access_index: *index,
            },
        },
        naga::front::spv::Error::InvalidIndexType(type_) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidIndexType,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_index_type: *type_,
            },
        },
        naga::front::spv::Error::InvalidBinding(binding) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidBinding,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_binding: *binding,
            },
        },
        naga::front::spv::Error::InvalidGlobalVar(_expression) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidGlobalVar,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_global_var: EMPTY_MUT,
            },
        },
        naga::front::spv::Error::InvalidImageExpression(_expression) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidImageExpression,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_image_expression: EMPTY_MUT,
            },
        },
        naga::front::spv::Error::InvalidImageWriteType => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidImageWriteType,
            data: default_data,
        },
        naga::front::spv::Error::InvalidImageBaseType(handle) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidImageBaseType,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_image_base_type: handle.index(),
            },
        },
        naga::front::spv::Error::InvalidImage(handle) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidImage,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_image: handle.index(),
            },
        },
        naga::front::spv::Error::InvalidAsType(handle) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidAsType,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_as_type: handle.index(),
            },
        },
        naga::front::spv::Error::InvalidVectorType(handle) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidVectorType,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_vector_type: handle.index(),
            },
        },
        naga::front::spv::Error::InconsistentComparisonSampling(handle) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InconsistentComparisonSampling,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                inconsistent_comparison_sampling: handle.index(),
            },
        },
        naga::front::spv::Error::WrongFunctionResultType(type_) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_WrongFunctionResultType,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                wrong_function_result_type: *type_,
            },
        },
        naga::front::spv::Error::WrongFunctionArgumentType(type_) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_WrongFunctionArgumentType,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                wrong_function_argument_type: *type_,
            },
        },
        naga::front::spv::Error::MissingDecoration(decoration) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_MissingDecoration,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                missing_decoration: *decoration as u32,
            },
        },
        naga::front::spv::Error::BadString => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_BadString,
            data: default_data,
        },
        naga::front::spv::Error::IncompleteData => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_IncompleteData,
            data: default_data,
        },
        naga::front::spv::Error::InvalidTerminator => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidTerminator,
            data: default_data,
        },
        naga::front::spv::Error::InvalidEdgeClassification => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidEdgeClassification,
            data: default_data,
        },
        naga::front::spv::Error::ControlFlowGraphCycle(id) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_ControlFlowGraphCycle,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                control_flow_graph_cycle: *id,
            },
        },
        naga::front::spv::Error::FunctionCallCycle(word) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_FunctionCallCycle,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                function_call_cycle: *word,
            },
        },
        naga::front::spv::Error::InvalidArraySize(word) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidArraySize,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_array_size: *word,
            },
        },
        naga::front::spv::Error::InvalidBarrierScope(word) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidBarrierScope,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_barrier_scope: *word,
            },
        },
        naga::front::spv::Error::InvalidBarrierMemorySemantics(word) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_InvalidBarrierMemorySemantics,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                invalid_barrier_memory_semantics: *word,
            },
        },
        naga::front::spv::Error::NonBindingArrayOfImageOrSamplers => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_NonBindingArrayOfImageOrSamplers,
            data: default_data,
        },
        naga::front::spv::Error::SpecIdTooHigh(spec_id) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_SpecIdTooHigh,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                spec_id_too_high: *spec_id,
            },
        },
        naga::front::spv::Error::AtomicUpgradeError(error) => ffi::NagaSPVFrontError {
            tag: ffi::NagaSPVFrontErrorTag_NagaSPVFrontErrorTag_AtomicUpgradeError,
            data: ffi::NagaSPVFrontError__bindgen_ty_1 {
                atomic_upgrade_error: atmoic_upgrade_front_error_to_ffi(error),
            },
        },
    }
}
