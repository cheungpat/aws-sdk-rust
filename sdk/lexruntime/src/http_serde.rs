// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_post_content(
    input: &crate::input::PostContentInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.session_attributes {
        let formatted_2 = aws_smithy_types::base64::encode(&inner_1);
        if !formatted_2.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_2;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "session_attributes",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-lex-session-attributes", header_value);
        }
    }
    if let Some(inner_3) = &input.request_attributes {
        let formatted_4 = aws_smithy_types::base64::encode(&inner_3);
        if !formatted_4.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_4;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "request_attributes",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-lex-request-attributes", header_value);
        }
    }
    if let Some(inner_5) = &input.content_type {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_6;
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
    if let Some(inner_7) = &input.accept {
        let formatted_8 = AsRef::<str>::as_ref(inner_7);
        if !formatted_8.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_8;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "accept",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Accept", header_value);
        }
    }
    if let Some(inner_9) = &input.active_contexts {
        let formatted_10 = aws_smithy_types::base64::encode(&inner_9);
        if !formatted_10.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_10;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "active_contexts",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &"*** Sensitive Data Redacted ***", err
                        ),
                    }
                })?;
            builder = builder.header("x-amz-lex-active-contexts", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_put_session(
    input: &crate::input::PutSessionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_11) = &input.accept {
        let formatted_12 = AsRef::<str>::as_ref(inner_11);
        if !formatted_12.is_empty() {
            use std::convert::TryFrom;
            let header_value = formatted_12;
            let header_value =
                http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                    aws_smithy_http::operation::BuildError::InvalidField {
                        field: "accept",
                        details: format!(
                            "`{}` cannot be used as a header value: {}",
                            &header_value, err
                        ),
                    }
                })?;
            builder = builder.header("Accept", header_value);
        }
    }
    Ok(builder)
}

pub fn deser_header_delete_session_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_get_session_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_active_contexts(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-active-contexts").iter();
    let var_13: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_13: std::result::Result<Vec<_>, _> = var_13
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_13 = var_13?;
    if var_13.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_13.len()),
        ))
    } else {
        let mut var_13 = var_13;
        Ok(var_13.pop())
    }
}

pub fn deser_header_post_content_post_content_output_alternative_intents(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-alternative-intents").iter();
    let var_14: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_14: std::result::Result<Vec<_>, _> = var_14
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_14 = var_14?;
    if var_14.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_14.len()),
        ))
    } else {
        let mut var_14 = var_14;
        Ok(var_14.pop())
    }
}

pub fn deser_payload_post_content_post_content_output_audio_stream(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::PostContentError> {
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_header_post_content_post_content_output_bot_version(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-bot-version").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_dialog_state(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::DialogState>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-dialog-state").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_encoded_input_transcript(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map
        .get_all("x-amz-lex-encoded-input-transcript")
        .iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_encoded_message(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-encoded-message").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_input_transcript(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-input-transcript").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_intent_name(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-intent-name").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_message(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-message").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_message_format(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::MessageFormatType>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-message-format").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_nlu_intent_confidence(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-nlu-intent-confidence").iter();
    let var_15: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_15: std::result::Result<Vec<_>, _> = var_15
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_15 = var_15?;
    if var_15.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_15.len()),
        ))
    } else {
        let mut var_15 = var_15;
        Ok(var_15.pop())
    }
}

pub fn deser_header_post_content_post_content_output_sentiment_response(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-sentiment").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_session_attributes(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-session-attributes").iter();
    let var_16: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_16: std::result::Result<Vec<_>, _> = var_16
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_16 = var_16?;
    if var_16.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_16.len()),
        ))
    } else {
        let mut var_16 = var_16;
        Ok(var_16.pop())
    }
}

pub fn deser_header_post_content_post_content_output_session_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-session-id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_slot_to_elicit(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-slot-to-elicit").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_content_post_content_output_slots(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-slots").iter();
    let var_17: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_17: std::result::Result<Vec<_>, _> = var_17
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_17 = var_17?;
    if var_17.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_17.len()),
        ))
    } else {
        let mut var_17 = var_17;
        Ok(var_17.pop())
    }
}

pub fn deser_header_post_content_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_post_text_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_active_contexts(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-active-contexts").iter();
    let var_18: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_18: std::result::Result<Vec<_>, _> = var_18
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_18 = var_18?;
    if var_18.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_18.len()),
        ))
    } else {
        let mut var_18 = var_18;
        Ok(var_18.pop())
    }
}

pub fn deser_payload_put_session_put_session_output_audio_stream(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<aws_smithy_http::byte_stream::ByteStream, crate::error::PutSessionError> {
    // replace the body with an empty body
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::byte_stream::ByteStream::new(body))
}

pub fn deser_header_put_session_put_session_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_dialog_state(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::DialogState>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-dialog-state").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_encoded_message(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-encoded-message").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_intent_name(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-intent-name").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_message(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-message").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_message_format(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<crate::model::MessageFormatType>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-message-format").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_session_attributes(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-session-attributes").iter();
    let var_19: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_19: std::result::Result<Vec<_>, _> = var_19
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_19 = var_19?;
    if var_19.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_19.len()),
        ))
    } else {
        let mut var_19 = var_19;
        Ok(var_19.pop())
    }
}

pub fn deser_header_put_session_put_session_output_session_id(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-session-id").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_slot_to_elicit(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-slot-to-elicit").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_header_put_session_put_session_output_slots(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("x-amz-lex-slots").iter();
    let var_20: Vec<std::string::String> = aws_smithy_http::header::read_many_from_str(headers)?;
    let var_20: std::result::Result<Vec<_>, _> = var_20
        .iter()
        .map(|s| {
            aws_smithy_types::base64::decode(s)
                .map_err(|_| {
                    aws_smithy_http::header::ParseError::new_with_message("failed to decode base64")
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|_| {
                        aws_smithy_http::header::ParseError::new_with_message(
                            "base64 encoded data was not valid utf-8",
                        )
                    })
                })
        })
        .collect();
    let var_20 = var_20?;
    if var_20.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_20.len()),
        ))
    } else {
        let mut var_20 = var_20;
        Ok(var_20.pop())
    }
}

pub fn deser_header_put_session_limit_exceeded_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}
