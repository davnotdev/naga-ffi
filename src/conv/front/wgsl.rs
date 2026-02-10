use super::*;

pub fn wgsl_front_options_to_ffi(options: &naga::front::wgsl::Options) -> ffi::WGSLFrontOptions {
    ffi::WGSLFrontOptions {
        parse_doc_comments: bool_to_ffi(options.parse_doc_comments),
    }
}

pub fn wgsl_front_parse_error_to_ffi(
    error: &naga::front::wgsl::ParseError,
) -> ffi::WGSLFrontParseError {
    unsafe {
        ffi::WGSLFrontParseError {
            message: string_to_ffi(error.message()),
            labels: slice_to_ffi(&error.labels().collect::<Vec<_>>(), |(span, s)| {
                ffi::WGSLFrontParseErrorLabel {
                    span: span_to_ffi(span),
                    message: string_to_ffi(s),
                }
            }),
            labels_len: error.labels().len(),
        }
    }
}
