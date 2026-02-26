use super::*;

pub fn wgsl_back_writer_flags_to_naga(
    flags: ffi::NagaWGSLBackWriterFlagsFlags,
) -> naga::back::wgsl::WriterFlags {
    let mut result = naga::back::wgsl::WriterFlags::empty();

    if flags & ffi::NagaWGSLBackWriterFlags_NagaWGSLBackWriterFlags_EXPLICIT_TYPES != 0 {
        result |= naga::back::wgsl::WriterFlags::EXPLICIT_TYPES;
    }

    sa::const_assert_eq!(
        naga::back::wgsl::WriterFlags::all().bits(),
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES.bits()
    );

    result
}

pub fn wgsl_back_error_to_ffi(error: &naga::back::wgsl::Error) -> ffi::NagaWGSLBackError {
    match error {
        naga::back::wgsl::Error::FmtError(error) => ffi::NagaWGSLBackError {
            tag: ffi::NagaWGSLBackErrorTag_NagaWGSLBackErrorTag_FmtError,
            data: ffi::NagaWGSLBackError__bindgen_ty_1 {
                fmt_error: unsafe { string_to_ffi(&error.to_string()) },
            },
        },
        naga::back::wgsl::Error::Custom(custom) => ffi::NagaWGSLBackError {
            tag: ffi::NagaWGSLBackErrorTag_NagaWGSLBackErrorTag_Custom,
            data: ffi::NagaWGSLBackError__bindgen_ty_1 {
                custom: unsafe { string_to_ffi(custom) },
            },
        },
        naga::back::wgsl::Error::Unimplemented(error) => ffi::NagaWGSLBackError {
            tag: ffi::NagaWGSLBackErrorTag_NagaWGSLBackErrorTag_FmtError,
            data: ffi::NagaWGSLBackError__bindgen_ty_1 {
                unimplemented: unsafe { string_to_ffi(error) },
            },
        },
        naga::back::wgsl::Error::UnsupportedRelationalFunction(relational_function) => {
            ffi::NagaWGSLBackError {
                tag: ffi::NagaWGSLBackErrorTag_NagaWGSLBackErrorTag_FmtError,
                data: ffi::NagaWGSLBackError__bindgen_ty_1 {
                    unsupported_relational_function: relational_function_to_ffi(
                        *relational_function,
                    ),
                },
            }
        }
        naga::back::wgsl::Error::Unsupported { kind, value } => ffi::NagaWGSLBackError {
            tag: ffi::NagaWGSLBackErrorTag_NagaWGSLBackErrorTag_FmtError,
            data: ffi::NagaWGSLBackError__bindgen_ty_1 {
                unsupported: ffi::NagaWGSLBackError__bindgen_ty_1__bindgen_ty_1 {
                    kind: unsafe { string_to_ffi(kind) },
                    value: unsafe { string_to_ffi(value) },
                },
            },
        },
    }
}
