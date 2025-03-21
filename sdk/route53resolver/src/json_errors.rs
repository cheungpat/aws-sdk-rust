// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_smithy_json::deserialize::token::skip_value;
use aws_smithy_json::deserialize::{json_token_iter, Error as DeserializeError, Token};
use aws_smithy_types::Error as SmithyError;
use bytes::Bytes;
use http::header::ToStrError;
use http::{HeaderMap, HeaderValue};
use std::borrow::Cow;

// currently only used by AwsJson
#[allow(unused)]
pub fn is_error<B>(response: &http::Response<B>) -> bool {
    !response.status().is_success()
}

fn sanitize_error_code(error_code: &str) -> &str {
    // Trim a trailing URL from the error code, beginning with a `:`
    let error_code = match error_code.find(':') {
        Some(idx) => &error_code[..idx],
        None => error_code,
    };

    // Trim a prefixing namespace from the error code, beginning with a `#`
    match error_code.find('#') {
        Some(idx) => &error_code[idx + 1..],
        None => error_code,
    }
}

struct ErrorBody<'a> {
    code: Option<Cow<'a, str>>,
    message: Option<Cow<'a, str>>,
}

fn parse_error_body(bytes: &[u8]) -> Result<ErrorBody, DeserializeError> {
    let mut tokens = json_token_iter(bytes).peekable();
    let (mut typ, mut code, mut message) = (None, None, None);
    if let Some(Token::StartObject { .. }) = tokens.next().transpose()? {
        loop {
            match tokens.next().transpose()? {
                Some(Token::EndObject { .. }) => break,
                Some(Token::ObjectKey { key, .. }) => {
                    if let Some(Ok(Token::ValueString { value, .. })) = tokens.peek() {
                        match key.as_escaped_str() {
                            "code" => code = Some(value.to_unescaped()?),
                            "__type" => typ = Some(value.to_unescaped()?),
                            "message" | "Message" | "errorMessage" => {
                                message = Some(value.to_unescaped()?)
                            }
                            _ => {}
                        }
                    }
                    skip_value(&mut tokens)?;
                }
                _ => {
                    return Err(DeserializeError::custom(
                        "expected object key or end object",
                    ))
                }
            }
        }
        if tokens.next().is_some() {
            return Err(DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ));
        }
    }
    Ok(ErrorBody {
        code: code.or(typ),
        message,
    })
}

fn error_type_from_header(headers: &HeaderMap<HeaderValue>) -> Result<Option<&str>, ToStrError> {
    headers
        .get("X-Amzn-Errortype")
        .map(|v| v.to_str())
        .transpose()
}

fn request_id(headers: &HeaderMap<HeaderValue>) -> Option<&str> {
    headers
        .get("X-Amzn-Requestid")
        .and_then(|v| v.to_str().ok())
}

pub fn parse_generic_error(
    payload: &Bytes,
    headers: &HeaderMap<HeaderValue>,
) -> Result<SmithyError, DeserializeError> {
    let ErrorBody { code, message } = parse_error_body(payload.as_ref())?;

    let mut err_builder = SmithyError::builder();
    if let Some(code) = error_type_from_header(headers)
        .map_err(|_| DeserializeError::custom("X-Amzn-Errortype header was not valid UTF-8"))?
        .or_else(|| code.as_deref())
        .map(sanitize_error_code)
    {
        err_builder.code(code);
    }
    if let Some(message) = message {
        err_builder.message(message);
    }
    if let Some(request_id) = request_id(headers) {
        err_builder.request_id(request_id);
    }
    Ok(err_builder.build())
}

#[cfg(test)]
mod test {
    use crate::json_errors::{parse_error_body, parse_generic_error, sanitize_error_code};
    use aws_smithy_types::Error;
    use bytes::Bytes;
    use std::borrow::Cow;

    #[test]
    fn generic_error() {
        let response = http::Response::builder()
            .header("X-Amzn-Requestid", "1234")
            .body(Bytes::from_static(
                br#"{ "__type": "FooError", "message": "Go to foo" }"#,
            ))
            .unwrap();
        assert_eq!(
            parse_generic_error(response.body(), response.headers()).unwrap(),
            Error::builder()
                .code("FooError")
                .message("Go to foo")
                .request_id("1234")
                .build()
        )
    }

    #[test]
    fn error_type() {
        assert_eq!(
            Some(Cow::Borrowed("FooError")),
            parse_error_body(br#"{ "__type": "FooError" }"#)
                .unwrap()
                .code
        );
    }

    #[test]
    fn code_takes_priority() {
        assert_eq!(
            Some(Cow::Borrowed("BarError")),
            parse_error_body(br#"{ "code": "BarError", "__type": "FooError" }"#)
                .unwrap()
                .code
        );
    }

    #[test]
    fn ignore_unrecognized_fields() {
        assert_eq!(
            Some(Cow::Borrowed("FooError")),
            parse_error_body(br#"{ "__type": "FooError", "asdf": 5, "fdsa": {}, "foo": "1" }"#)
                .unwrap()
                .code
        );
    }

    #[test]
    fn sanitize_namespace_and_url() {
        assert_eq!(
            sanitize_error_code("aws.protocoltests.restjson#FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"),
            "FooError");
    }

    #[test]
    fn sanitize_noop() {
        assert_eq!(sanitize_error_code("FooError"), "FooError");
    }

    #[test]
    fn sanitize_url() {
        assert_eq!(
            sanitize_error_code(
                "FooError:http://internal.amazon.com/coral/com.amazon.coral.validate/"
            ),
            "FooError"
        );
    }

    #[test]
    fn sanitize_namespace() {
        assert_eq!(
            sanitize_error_code("aws.protocoltests.restjson#FooError"),
            "FooError"
        );
    }

    // services like lambda use an alternate `Message` instead of `message`
    #[test]
    fn alternative_error_message_names() {
        let response = http::Response::builder()
            .header("x-amzn-errortype", "ResourceNotFoundException")
            .body(Bytes::from_static(
                br#"{
                    "Type": "User",
                    "Message": "Functions from 'us-west-2' are not reachable from us-east-1"
                }"#,
            ))
            .unwrap();
        assert_eq!(
            parse_generic_error(response.body(), response.headers()).unwrap(),
            Error::builder()
                .code("ResourceNotFoundException")
                .message("Functions from 'us-west-2' are not reachable from us-east-1")
                .build()
        );
    }
}
