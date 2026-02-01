use super::*;

pub fn wgsl_back_writer_flags_to_ffi(
    flags: naga::back::wgsl::WriterFlags,
) -> ffi::WGSLBackWriterFlags {
    let mut result: ffi::WGSLBackWriterFlags = 0;

    if flags.contains(naga::back::wgsl::WriterFlags::EXPLICIT_TYPES) {
        result |= ffi::WGSLBackWriterFlags_WGSLBackWriterFlags_EXPLICIT_TYPES;
    }

    sa::const_assert_eq!(
        naga::back::wgsl::WriterFlags::all().bits(),
        naga::back::wgsl::WriterFlags::EXPLICIT_TYPES.bits()
    );

    result
}

pub fn wgsl_back_error_to_ffi(error: &naga::back::wgsl::Error) -> ffi::WGSLBackError {
    match error {
        naga::back::wgsl::Error::FmtError(error) => ffi::WGSLBackError {
            tag: ffi::WGSLBackErrorTag_WGSLBackErrorTag_FmtError,
            data: ffi::WGSLBackError__bindgen_ty_1 {
                fmt_error: unsafe { string_to_ffi(&error.to_string()) },
            },
        },
        naga::back::wgsl::Error::Custom(custom) => ffi::WGSLBackError {
            tag: ffi::WGSLBackErrorTag_WGSLBackErrorTag_Custom,
            data: ffi::WGSLBackError__bindgen_ty_1 {
                custom: unsafe { string_to_ffi(custom) },
            },
        },
        naga::back::wgsl::Error::Unimplemented(error) => ffi::WGSLBackError {
            tag: ffi::WGSLBackErrorTag_WGSLBackErrorTag_FmtError,
            data: ffi::WGSLBackError__bindgen_ty_1 {
                unimplemented: unsafe { string_to_ffi(error) },
            },
        },
        naga::back::wgsl::Error::UnsupportedRelationalFunction(relational_function) => {
            ffi::WGSLBackError {
                tag: ffi::WGSLBackErrorTag_WGSLBackErrorTag_FmtError,
                data: ffi::WGSLBackError__bindgen_ty_1 {
                    unsupported_relational_function: relational_function_to_ffi(
                        *relational_function,
                    ),
                },
            }
        }
        naga::back::wgsl::Error::Unsupported { kind, value } => ffi::WGSLBackError {
            tag: ffi::WGSLBackErrorTag_WGSLBackErrorTag_FmtError,
            data: ffi::WGSLBackError__bindgen_ty_1 {
                unsupported: ffi::WGSLBackError__bindgen_ty_1__bindgen_ty_1 {
                    kind: unsafe { string_to_ffi(kind) },
                    value: unsafe { string_to_ffi(value) },
                },
            },
        },
    }
}
