// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_get_object(
    input: &crate::input::GetObjectInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.range {
        let formatted_2 = AsRef::<str>::as_ref(inner_1);
        if !formatted_2.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "range",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Range", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_put_object(
    input: &crate::input::PutObjectInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_3) = &input.content_type {
        let formatted_4 = AsRef::<str>::as_ref(inner_3);
        if !formatted_4.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "content_type",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Content-Type", header_value);
        }
    }
    if let Some(inner_5) = &input.cache_control {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_6;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "cache_control",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Cache-Control", header_value);
        }
    }
    if let Some(inner_7) = &input.storage_class {
        let formatted_8 = AsRef::<str>::as_ref(inner_7);
        if !formatted_8.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_8;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "storage_class",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-storage-class", header_value);
        }
    }
    if let Some(inner_9) = &input.upload_availability {
        let formatted_10 = AsRef::<str>::as_ref(inner_9);
        if !formatted_10.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_10;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "upload_availability",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-upload-availability", header_value);
        }
    }
    Ok(builder)
}

pub fn deser_header_describe_object_describe_object_output_cache_control(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Cache-Control").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_describe_object_describe_object_output_content_length(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Length").iter();
    let var_11 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_11.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_11.len()),
        ))
    } else {
        let mut var_11 = var_11;
        Ok(var_11.pop())
    }
}

pub fn deser_header_describe_object_describe_object_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_describe_object_describe_object_output_e_tag(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("ETag").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_describe_object_describe_object_output_last_modified(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<aws_smithy_types::DateTime>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Last-Modified").iter();
    let var_12: Vec<aws_smithy_types::DateTime> = aws_smithy_http::header::many_dates(
        headers,
        aws_smithy_types::date_time::Format::HttpDate,
    )?;
    if var_12.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_12.len()),
        ))
    } else {
        let mut var_12 = var_12;
        Ok(var_12.pop())
    }
}

pub fn deser_payload_get_object_get_object_output_body(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::GetObjectError> {
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_header_get_object_get_object_output_cache_control(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Cache-Control").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_object_get_object_output_content_length(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Length").iter();
    let var_13 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_13.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_13.len()),
        ))
    } else {
        let mut var_13 = var_13;
        Ok(var_13.pop())
    }
}

pub fn deser_header_get_object_get_object_output_content_range(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Range").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_object_get_object_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_object_get_object_output_e_tag(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("ETag").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_object_get_object_output_last_modified(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<aws_smithy_types::DateTime>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Last-Modified").iter();
    let var_14: Vec<aws_smithy_types::DateTime> = aws_smithy_http::header::many_dates(
        headers,
        aws_smithy_types::date_time::Format::HttpDate,
    )?;
    if var_14.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_14.len()),
        ))
    } else {
        let mut var_14 = var_14;
        Ok(var_14.pop())
    }
}
