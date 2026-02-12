use super::*;

pub fn glsl_front_defines_to_naga(
    defines: &ffi::GLSLFrontDefines,
) -> naga::FastHashMap<String, String> {
    unsafe {
        std::slice::from_raw_parts(defines.entries, defines.entries_len)
            .iter()
            .map(|entry| (string_to_naga(entry.key), string_to_naga(entry.value)))
            .collect()
    }
}

pub fn glsl_front_options_to_naga(options: &ffi::GLSLFrontOptions) -> naga::front::glsl::Options {
    naga::front::glsl::Options {
        stage: shader_stage_to_naga(&options.stage),
        defines: glsl_front_defines_to_naga(&options.defines),
    }
}

fn glsl_front_precision_to_ffi(
    precision: &naga::front::glsl::Precision,
) -> ffi::GLSLFrontPrecision {
    match precision {
        naga::front::glsl::Precision::Low => ffi::GLSLFrontPrecision_Low,
        naga::front::glsl::Precision::Medium => ffi::GLSLFrontPrecision_Medium,
        naga::front::glsl::Precision::High => ffi::GLSLFrontPrecision_High,
    }
}

pub fn glsl_front_token_value_to_ffi(
    token_value: &naga::front::glsl::TokenValue,
) -> ffi::GLSLFrontTokenValue {
    let default_data = ffi::GLSLFrontTokenValue__bindgen_ty_1::default();

    match token_value {
        naga::front::glsl::TokenValue::Identifier(identifier) => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Identifier,
            data: ffi::GLSLFrontTokenValue__bindgen_ty_1 {
                identifier: unsafe { string_to_ffi(identifier) },
            },
        },
        naga::front::glsl::TokenValue::FloatConstant(float) => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_FloatConstant,
            data: ffi::GLSLFrontTokenValue__bindgen_ty_1 {
                float_constant: ffi::GLSLFrontFloat {
                    value: float.value,
                    width: float.width,
                },
            },
        },
        naga::front::glsl::TokenValue::IntConstant(integer) => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_IntConstant,
            data: ffi::GLSLFrontTokenValue__bindgen_ty_1 {
                int_constant: ffi::GLSLFrontInteger {
                    value: integer.value,
                    signed_: bool_to_ffi(integer.signed),
                    width: integer.width,
                },
            },
        },
        naga::front::glsl::TokenValue::BoolConstant(_) => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_BoolConstant,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Layout => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Layout,
            data: default_data,
        },
        naga::front::glsl::TokenValue::In => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_In,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Out => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Out,
            data: default_data,
        },
        naga::front::glsl::TokenValue::InOut => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_InOut,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Uniform => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Uniform,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Buffer => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Buffer,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Const => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Const,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Shared => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Shared,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Restrict => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Restrict,
            data: default_data,
        },
        naga::front::glsl::TokenValue::MemoryQualifier(storage_access) => {
            ffi::GLSLFrontTokenValue {
                tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_MemoryQualifier,
                data: ffi::GLSLFrontTokenValue__bindgen_ty_1 {
                    memory_qualifier: storage_access_to_ffi(storage_access),
                },
            }
        }
        naga::front::glsl::TokenValue::Invariant => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Invariant,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Interpolation(interpolation) => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Interpolation,
            data: ffi::GLSLFrontTokenValue__bindgen_ty_1 {
                interpolation: interpolation_to_ffi(interpolation),
            },
        },
        naga::front::glsl::TokenValue::Sampling(sampling) => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Sampling,
            data: ffi::GLSLFrontTokenValue__bindgen_ty_1 {
                sampling: sampling_to_ffi(sampling),
            },
        },
        naga::front::glsl::TokenValue::Precision => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Precision,
            data: default_data,
        },
        naga::front::glsl::TokenValue::PrecisionQualifier(precision) => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_PrecisionQualifier,
            data: ffi::GLSLFrontTokenValue__bindgen_ty_1 {
                precision_qualifier: glsl_front_precision_to_ffi(precision),
            },
        },
        naga::front::glsl::TokenValue::Continue => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Continue,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Break => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Break,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Return => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Return,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Discard => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Discard,
            data: default_data,
        },
        naga::front::glsl::TokenValue::If => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_If,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Else => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Else,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Switch => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Switch,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Case => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Case,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Default => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Default,
            data: default_data,
        },
        naga::front::glsl::TokenValue::While => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_While,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Do => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Do,
            data: default_data,
        },
        naga::front::glsl::TokenValue::For => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_For,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Void => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Void,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Struct => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Struct,
            data: default_data,
        },
        naga::front::glsl::TokenValue::TypeName(type_name) => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_TypeName,
            data: ffi::GLSLFrontTokenValue__bindgen_ty_1 {
                type_name: type_to_ffi(type_name),
            },
        },
        naga::front::glsl::TokenValue::Assign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Assign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::AddAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_AddAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::SubAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_SubAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::MulAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_MulAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::DivAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_DivAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::ModAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_ModAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftShiftAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LeftShiftAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightShiftAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_RightShiftAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::AndAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_AndAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::XorAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_XorAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::OrAssign => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_OrAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Increment => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Increment,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Decrement => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Decrement,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LogicalOr => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LogicalOr,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LogicalAnd => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LogicalAnd,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LogicalXor => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LogicalXor,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LessEqual => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LessEqual,
            data: default_data,
        },
        naga::front::glsl::TokenValue::GreaterEqual => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_GreaterEqual,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Equal => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Equal,
            data: default_data,
        },
        naga::front::glsl::TokenValue::NotEqual => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_NotEqual,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftShift => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LeftShift,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightShift => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_RightShift,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftBrace => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LeftBrace,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightBrace => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_RightBrace,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftParen => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LeftParen,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightParen => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_RightParen,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftBracket => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LeftBracket,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightBracket => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_RightBracket,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftAngle => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_LeftAngle,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightAngle => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_RightAngle,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Comma => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Comma,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Semicolon => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Semicolon,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Colon => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Colon,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Dot => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Dot,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Bang => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Bang,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Dash => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Dash,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Tilde => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Tilde,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Plus => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Plus,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Star => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Star,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Slash => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Slash,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Percent => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Percent,
            data: default_data,
        },
        naga::front::glsl::TokenValue::VerticalBar => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_VerticalBar,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Caret => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Caret,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Ampersand => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Ampersand,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Question => ffi::GLSLFrontTokenValue {
            tag: ffi::GLSLFrontTokenValueTag_GLSLFrontTokenValueTag_Question,
            data: default_data,
        },
    }
}

pub fn glsl_front_expected_token_to_ffi(
    expected_token: &naga::front::glsl::ExpectedToken,
) -> ffi::GLSLFrontExpectedToken {
    let default_data = ffi::GLSLFrontExpectedToken__bindgen_ty_1::default();

    match expected_token {
        naga::front::glsl::ExpectedToken::Token(token_value) => ffi::GLSLFrontExpectedToken {
            tag: ffi::GLSLFrontExpectedTokenTag_GLSLFrontExpectedTokenTag_Token,
            data: ffi::GLSLFrontExpectedToken__bindgen_ty_1 {
                token: glsl_front_token_value_to_ffi(token_value),
            },
        },
        naga::front::glsl::ExpectedToken::TypeName => ffi::GLSLFrontExpectedToken {
            tag: ffi::GLSLFrontExpectedTokenTag_GLSLFrontExpectedTokenTag_TypeName,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::Identifier => ffi::GLSLFrontExpectedToken {
            tag: ffi::GLSLFrontExpectedTokenTag_GLSLFrontExpectedTokenTag_Identifier,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::IntLiteral => ffi::GLSLFrontExpectedToken {
            tag: ffi::GLSLFrontExpectedTokenTag_GLSLFrontExpectedTokenTag_IntLiteral,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::FloatLiteral => ffi::GLSLFrontExpectedToken {
            tag: ffi::GLSLFrontExpectedTokenTag_GLSLFrontExpectedTokenTag_FloatLiteral,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::BoolLiteral => ffi::GLSLFrontExpectedToken {
            tag: ffi::GLSLFrontExpectedTokenTag_GLSLFrontExpectedTokenTag_BoolLiteral,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::Eof => ffi::GLSLFrontExpectedToken {
            tag: ffi::GLSLFrontExpectedTokenTag_GLSLFrontExpectedTokenTag_Eof,
            data: default_data,
        },
    }
}

pub fn glsl_front_error_kind_to_ffi(
    error_kind: &naga::front::glsl::ErrorKind,
) -> ffi::GLSLFrontErrorKind {
    let default_data = ffi::GLSLFrontErrorKind__bindgen_ty_1::default();

    match error_kind {
        naga::front::glsl::ErrorKind::EndOfFile => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_EndOfFile,
            data: default_data,
        },
        naga::front::glsl::ErrorKind::InvalidProfile(error) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_InvalidProfile,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                invalid_profile: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::InvalidVersion(version) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_InvalidVersion,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                invalid_version: *version
            },
        },
        naga::front::glsl::ErrorKind::InvalidToken(token_value, expected_tokens) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_InvalidToken,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                invalid_token: ffi::GLSLFrontErrorKind__bindgen_ty_1__bindgen_ty_1 {
                    token: glsl_front_token_value_to_ffi(token_value),
                    expected: unsafe { slice_to_ffi(expected_tokens.as_slice(), glsl_front_expected_token_to_ffi) },
                    expected_len: expected_tokens.len(),
                },
            },
        },
        naga::front::glsl::ErrorKind::NotImplemented(error) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_NotImplemented,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnknownVariable(error) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_UnknownVariable,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnknownType(error) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_UnknownType,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnknownField(error) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_UnknownField,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnknownLayoutQualifier(error) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_UnknownLayoutQualifier,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnsupportedMatrixWithTwoRowsInStd140 { columns } => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_UnsupportedMatrixWithTwoRowsInStd140,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                unsupported_matrix_with_two_rows_in_std140: ffi::GLSLFrontErrorKind__bindgen_ty_1__bindgen_ty_2 {
                    columns: *columns,
                },
            },
        },
        naga::front::glsl::ErrorKind::UnsupportedF16MatrixInStd140 { columns, rows } => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_UnsupportedF16MatrixInStd140,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                unsupported_f16_matrix_in_std140: ffi::GLSLFrontErrorKind__bindgen_ty_1__bindgen_ty_3 {
                    columns: *columns,
                    rows: *rows,
                },
            },
        },
        naga::front::glsl::ErrorKind::VariableAlreadyDeclared(error) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_VariableAlreadyDeclared,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::SemanticError(cow) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_SemanticError,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                semantic_error: unsafe { string_to_ffi(cow.as_ref()) },
            },
        },
        naga::front::glsl::ErrorKind::PreprocessorError(_) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_PreprocessorError,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                preprocessor_error: EMPTY_MUT
            },
        },
        naga::front::glsl::ErrorKind::InternalError(error) => ffi::GLSLFrontErrorKind {
            tag: ffi::GLSLFrontErrorKindTag_GLSLFrontErrorKindTag_InternalError,
            data: ffi::GLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
    }
}

pub fn glsl_front_parse_errors_to_ffi(
    errors: &naga::front::glsl::ParseErrors,
) -> ffi::GLSLFrontParseErrors {
    ffi::GLSLFrontParseErrors {
        errors: unsafe {
            slice_to_ffi(&errors.errors, |error| ffi::GLSLFrontParseError {
                kind: glsl_front_error_kind_to_ffi(&error.kind),
                span: span_to_ffi(&error.meta),
            })
        },
        errors_len: errors.errors.len(),
    }
}
