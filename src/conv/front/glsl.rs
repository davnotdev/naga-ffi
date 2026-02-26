use super::*;

pub fn glsl_front_defines_to_naga(
    defines: &ffi::NagaGLSLFrontDefines,
) -> naga::FastHashMap<String, String> {
    unsafe {
        std::slice::from_raw_parts(defines.entries, defines.entries_len)
            .iter()
            .map(|entry| (string_to_naga(entry.key), string_to_naga(entry.value)))
            .collect()
    }
}

pub fn glsl_front_options_to_naga(
    options: &ffi::NagaGLSLFrontOptions,
) -> naga::front::glsl::Options {
    naga::front::glsl::Options {
        stage: shader_stage_to_naga(&options.stage),
        defines: glsl_front_defines_to_naga(&options.defines),
    }
}

fn glsl_front_precision_to_ffi(
    precision: &naga::front::glsl::Precision,
) -> ffi::NagaGLSLFrontPrecision {
    match precision {
        naga::front::glsl::Precision::Low => ffi::NagaGLSLFrontPrecision_NagaGLSLFrontPrecision_Low,
        naga::front::glsl::Precision::Medium => {
            ffi::NagaGLSLFrontPrecision_NagaGLSLFrontPrecision_Medium
        }
        naga::front::glsl::Precision::High => {
            ffi::NagaGLSLFrontPrecision_NagaGLSLFrontPrecision_High
        }
    }
}

pub fn glsl_front_token_value_to_ffi(
    token_value: &naga::front::glsl::TokenValue,
) -> ffi::NagaGLSLFrontTokenValue {
    let default_data = ffi::NagaGLSLFrontTokenValue__bindgen_ty_1::default();

    match token_value {
        naga::front::glsl::TokenValue::Identifier(identifier) => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Identifier,
            data: ffi::NagaGLSLFrontTokenValue__bindgen_ty_1 {
                identifier: unsafe { string_to_ffi(identifier) },
            },
        },
        naga::front::glsl::TokenValue::FloatConstant(float) => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_FloatConstant,
            data: ffi::NagaGLSLFrontTokenValue__bindgen_ty_1 {
                float_constant: ffi::NagaGLSLFrontFloat {
                    value: float.value,
                    width: float.width,
                },
            },
        },
        naga::front::glsl::TokenValue::IntConstant(integer) => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_IntConstant,
            data: ffi::NagaGLSLFrontTokenValue__bindgen_ty_1 {
                int_constant: ffi::NagaGLSLFrontInteger {
                    value: integer.value,
                    signed_: bool_to_ffi(integer.signed),
                    width: integer.width,
                },
            },
        },
        naga::front::glsl::TokenValue::BoolConstant(_) => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_BoolConstant,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Layout => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Layout,
            data: default_data,
        },
        naga::front::glsl::TokenValue::In => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_In,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Out => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Out,
            data: default_data,
        },
        naga::front::glsl::TokenValue::InOut => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_InOut,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Uniform => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Uniform,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Buffer => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Buffer,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Const => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Const,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Shared => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Shared,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Restrict => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Restrict,
            data: default_data,
        },
        naga::front::glsl::TokenValue::MemoryQualifier(storage_access) => {
            ffi::NagaGLSLFrontTokenValue {
                tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_MemoryQualifier,
                data: ffi::NagaGLSLFrontTokenValue__bindgen_ty_1 {
                    memory_qualifier: storage_access_to_ffi(storage_access),
                },
            }
        }
        naga::front::glsl::TokenValue::Invariant => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Invariant,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Interpolation(interpolation) => {
            ffi::NagaGLSLFrontTokenValue {
                tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Interpolation,
                data: ffi::NagaGLSLFrontTokenValue__bindgen_ty_1 {
                    interpolation: interpolation_to_ffi(interpolation),
                },
            }
        }
        naga::front::glsl::TokenValue::Sampling(sampling) => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Sampling,
            data: ffi::NagaGLSLFrontTokenValue__bindgen_ty_1 {
                sampling: sampling_to_ffi(sampling),
            },
        },
        naga::front::glsl::TokenValue::Precision => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Precision,
            data: default_data,
        },
        naga::front::glsl::TokenValue::PrecisionQualifier(precision) => {
            ffi::NagaGLSLFrontTokenValue {
                tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_PrecisionQualifier,
                data: ffi::NagaGLSLFrontTokenValue__bindgen_ty_1 {
                    precision_qualifier: glsl_front_precision_to_ffi(precision),
                },
            }
        }
        naga::front::glsl::TokenValue::Continue => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Continue,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Break => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Break,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Return => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Return,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Discard => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Discard,
            data: default_data,
        },
        naga::front::glsl::TokenValue::If => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_If,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Else => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Else,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Switch => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Switch,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Case => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Case,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Default => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Default,
            data: default_data,
        },
        naga::front::glsl::TokenValue::While => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_While,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Do => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Do,
            data: default_data,
        },
        naga::front::glsl::TokenValue::For => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_For,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Void => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Void,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Struct => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Struct,
            data: default_data,
        },
        naga::front::glsl::TokenValue::TypeName(type_name) => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_TypeName,
            data: ffi::NagaGLSLFrontTokenValue__bindgen_ty_1 {
                type_name: type_to_ffi(type_name),
            },
        },
        naga::front::glsl::TokenValue::Assign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Assign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::AddAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_AddAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::SubAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_SubAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::MulAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_MulAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::DivAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_DivAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::ModAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_ModAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftShiftAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LeftShiftAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightShiftAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_RightShiftAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::AndAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_AndAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::XorAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_XorAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::OrAssign => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_OrAssign,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Increment => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Increment,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Decrement => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Decrement,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LogicalOr => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LogicalOr,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LogicalAnd => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LogicalAnd,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LogicalXor => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LogicalXor,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LessEqual => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LessEqual,
            data: default_data,
        },
        naga::front::glsl::TokenValue::GreaterEqual => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_GreaterEqual,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Equal => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Equal,
            data: default_data,
        },
        naga::front::glsl::TokenValue::NotEqual => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_NotEqual,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftShift => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LeftShift,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightShift => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_RightShift,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftBrace => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LeftBrace,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightBrace => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_RightBrace,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftParen => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LeftParen,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightParen => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_RightParen,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftBracket => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LeftBracket,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightBracket => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_RightBracket,
            data: default_data,
        },
        naga::front::glsl::TokenValue::LeftAngle => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_LeftAngle,
            data: default_data,
        },
        naga::front::glsl::TokenValue::RightAngle => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_RightAngle,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Comma => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Comma,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Semicolon => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Semicolon,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Colon => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Colon,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Dot => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Dot,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Bang => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Bang,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Dash => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Dash,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Tilde => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Tilde,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Plus => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Plus,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Star => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Star,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Slash => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Slash,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Percent => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Percent,
            data: default_data,
        },
        naga::front::glsl::TokenValue::VerticalBar => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_VerticalBar,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Caret => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Caret,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Ampersand => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Ampersand,
            data: default_data,
        },
        naga::front::glsl::TokenValue::Question => ffi::NagaGLSLFrontTokenValue {
            tag: ffi::NagaGLSLFrontTokenValueTag_NagaGLSLFrontTokenValueTag_Question,
            data: default_data,
        },
    }
}

pub fn glsl_front_expected_token_to_ffi(
    expected_token: &naga::front::glsl::ExpectedToken,
) -> ffi::NagaGLSLFrontExpectedToken {
    let default_data = ffi::NagaGLSLFrontExpectedToken__bindgen_ty_1::default();

    match expected_token {
        naga::front::glsl::ExpectedToken::Token(token_value) => ffi::NagaGLSLFrontExpectedToken {
            tag: ffi::NagaGLSLFrontExpectedTokenTag_NagaGLSLFrontExpectedTokenTag_Token,
            data: ffi::NagaGLSLFrontExpectedToken__bindgen_ty_1 {
                token: glsl_front_token_value_to_ffi(token_value),
            },
        },
        naga::front::glsl::ExpectedToken::TypeName => ffi::NagaGLSLFrontExpectedToken {
            tag: ffi::NagaGLSLFrontExpectedTokenTag_NagaGLSLFrontExpectedTokenTag_TypeName,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::Identifier => ffi::NagaGLSLFrontExpectedToken {
            tag: ffi::NagaGLSLFrontExpectedTokenTag_NagaGLSLFrontExpectedTokenTag_Identifier,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::IntLiteral => ffi::NagaGLSLFrontExpectedToken {
            tag: ffi::NagaGLSLFrontExpectedTokenTag_NagaGLSLFrontExpectedTokenTag_IntLiteral,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::FloatLiteral => ffi::NagaGLSLFrontExpectedToken {
            tag: ffi::NagaGLSLFrontExpectedTokenTag_NagaGLSLFrontExpectedTokenTag_FloatLiteral,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::BoolLiteral => ffi::NagaGLSLFrontExpectedToken {
            tag: ffi::NagaGLSLFrontExpectedTokenTag_NagaGLSLFrontExpectedTokenTag_BoolLiteral,
            data: default_data,
        },
        naga::front::glsl::ExpectedToken::Eof => ffi::NagaGLSLFrontExpectedToken {
            tag: ffi::NagaGLSLFrontExpectedTokenTag_NagaGLSLFrontExpectedTokenTag_Eof,
            data: default_data,
        },
    }
}

pub fn glsl_front_error_kind_to_ffi(
    error_kind: &naga::front::glsl::ErrorKind,
) -> ffi::NagaGLSLFrontErrorKind {
    let default_data = ffi::NagaGLSLFrontErrorKind__bindgen_ty_1::default();

    match error_kind {
        naga::front::glsl::ErrorKind::EndOfFile => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_EndOfFile,
            data: default_data,
        },
        naga::front::glsl::ErrorKind::InvalidProfile(error) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_InvalidProfile,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                invalid_profile: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::InvalidVersion(version) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_InvalidVersion,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                invalid_version: *version
            },
        },
        naga::front::glsl::ErrorKind::InvalidToken(token_value, expected_tokens) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_InvalidToken,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                invalid_token: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1__bindgen_ty_1 {
                    token: glsl_front_token_value_to_ffi(token_value),
                    expected: unsafe { slice_to_ffi(expected_tokens.as_slice(), glsl_front_expected_token_to_ffi) },
                    expected_len: expected_tokens.len(),
                },
            },
        },
        naga::front::glsl::ErrorKind::NotImplemented(error) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_NotImplemented,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnknownVariable(error) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_UnknownVariable,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnknownType(error) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_UnknownType,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnknownField(error) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_UnknownField,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnknownLayoutQualifier(error) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_UnknownLayoutQualifier,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::UnsupportedMatrixWithTwoRowsInStd140 { columns } => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_UnsupportedMatrixWithTwoRowsInStd140,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                unsupported_matrix_with_two_rows_in_std140: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1__bindgen_ty_2 {
                    columns: *columns,
                },
            },
        },
        naga::front::glsl::ErrorKind::UnsupportedF16MatrixInStd140 { columns, rows } => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_UnsupportedF16MatrixInStd140,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                unsupported_f16_matrix_in_std140: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1__bindgen_ty_3 {
                    columns: *columns,
                    rows: *rows,
                },
            },
        },
        naga::front::glsl::ErrorKind::VariableAlreadyDeclared(error) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_VariableAlreadyDeclared,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
        naga::front::glsl::ErrorKind::SemanticError(cow) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_SemanticError,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                semantic_error: unsafe { string_to_ffi(cow.as_ref()) },
            },
        },
        naga::front::glsl::ErrorKind::PreprocessorError(_) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_PreprocessorError,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                preprocessor_error: EMPTY_MUT
            },
        },
        naga::front::glsl::ErrorKind::InternalError(error) => ffi::NagaGLSLFrontErrorKind {
            tag: ffi::NagaGLSLFrontErrorKindTag_NagaGLSLFrontErrorKindTag_InternalError,
            data: ffi::NagaGLSLFrontErrorKind__bindgen_ty_1 {
                internal_error: unsafe { string_to_ffi(error) }
            },
        },
    }
}

pub fn glsl_front_parse_errors_to_ffi(
    errors: &naga::front::glsl::ParseErrors,
) -> ffi::NagaGLSLFrontParseErrors {
    ffi::NagaGLSLFrontParseErrors {
        errors: unsafe {
            slice_to_ffi(&errors.errors, |error| ffi::NagaGLSLFrontParseError {
                kind: glsl_front_error_kind_to_ffi(&error.kind),
                span: span_to_ffi(&error.meta),
            })
        },
        errors_len: errors.errors.len(),
    }
}
