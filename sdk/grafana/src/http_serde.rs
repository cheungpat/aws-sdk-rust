// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_associate_license_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_1.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_1.len()),
        ))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub fn deser_header_associate_license_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_2 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_2.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_2.len()),
        ))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub fn deser_header_create_workspace_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_3 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_3.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_3.len()),
        ))
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub fn deser_header_create_workspace_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_4 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_4.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_4.len()),
        ))
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}

pub fn deser_header_delete_workspace_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_5 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_5.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_5.len()),
        ))
    } else {
        let mut var_5 = var_5;
        Ok(var_5.pop())
    }
}

pub fn deser_header_delete_workspace_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_6 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_6.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_6.len()),
        ))
    } else {
        let mut var_6 = var_6;
        Ok(var_6.pop())
    }
}

pub fn deser_header_describe_workspace_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_7 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_7.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_7.len()),
        ))
    } else {
        let mut var_7 = var_7;
        Ok(var_7.pop())
    }
}

pub fn deser_header_describe_workspace_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_8 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_8.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_8.len()),
        ))
    } else {
        let mut var_8 = var_8;
        Ok(var_8.pop())
    }
}

pub fn deser_header_describe_workspace_authentication_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_9 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_9.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_9.len()),
        ))
    } else {
        let mut var_9 = var_9;
        Ok(var_9.pop())
    }
}

pub fn deser_header_describe_workspace_authentication_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_10 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_10.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_10.len()),
        ))
    } else {
        let mut var_10 = var_10;
        Ok(var_10.pop())
    }
}

pub fn deser_header_disassociate_license_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_11 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_11.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_11.len()),
        ))
    } else {
        let mut var_11 = var_11;
        Ok(var_11.pop())
    }
}

pub fn deser_header_disassociate_license_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_12 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_12.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_12.len()),
        ))
    } else {
        let mut var_12 = var_12;
        Ok(var_12.pop())
    }
}

pub fn deser_header_list_permissions_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_13 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_13.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_13.len()),
        ))
    } else {
        let mut var_13 = var_13;
        Ok(var_13.pop())
    }
}

pub fn deser_header_list_permissions_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_14 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_14.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_14.len()),
        ))
    } else {
        let mut var_14 = var_14;
        Ok(var_14.pop())
    }
}

pub fn deser_header_list_tags_for_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_15 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_15.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_15.len()),
        ))
    } else {
        let mut var_15 = var_15;
        Ok(var_15.pop())
    }
}

pub fn deser_header_list_tags_for_resource_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_16 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_16.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_16.len()),
        ))
    } else {
        let mut var_16 = var_16;
        Ok(var_16.pop())
    }
}

pub fn deser_header_list_workspaces_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_17 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_17.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_17.len()),
        ))
    } else {
        let mut var_17 = var_17;
        Ok(var_17.pop())
    }
}

pub fn deser_header_list_workspaces_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_18 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_18.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_18.len()),
        ))
    } else {
        let mut var_18 = var_18;
        Ok(var_18.pop())
    }
}

pub fn deser_header_tag_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_19 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_19.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_19.len()),
        ))
    } else {
        let mut var_19 = var_19;
        Ok(var_19.pop())
    }
}

pub fn deser_header_tag_resource_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_20 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_20.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_20.len()),
        ))
    } else {
        let mut var_20 = var_20;
        Ok(var_20.pop())
    }
}

pub fn deser_header_untag_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_21 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_21.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_21.len()),
        ))
    } else {
        let mut var_21 = var_21;
        Ok(var_21.pop())
    }
}

pub fn deser_header_untag_resource_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_22 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_22.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_22.len()),
        ))
    } else {
        let mut var_22 = var_22;
        Ok(var_22.pop())
    }
}

pub fn deser_header_update_permissions_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_23 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_23.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_23.len()),
        ))
    } else {
        let mut var_23 = var_23;
        Ok(var_23.pop())
    }
}

pub fn deser_header_update_permissions_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_24 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_24.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_24.len()),
        ))
    } else {
        let mut var_24 = var_24;
        Ok(var_24.pop())
    }
}

pub fn deser_header_update_workspace_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_25 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_25.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_25.len()),
        ))
    } else {
        let mut var_25 = var_25;
        Ok(var_25.pop())
    }
}

pub fn deser_header_update_workspace_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_26 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_26.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_26.len()),
        ))
    } else {
        let mut var_26 = var_26;
        Ok(var_26.pop())
    }
}

pub fn deser_header_update_workspace_authentication_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_27 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_27.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_27.len()),
        ))
    } else {
        let mut var_27 = var_27;
        Ok(var_27.pop())
    }
}

pub fn deser_header_update_workspace_authentication_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_28 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_28.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_28.len()),
        ))
    } else {
        let mut var_28 = var_28;
        Ok(var_28.pop())
    }
}
