// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateAppInstance`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_app_instance`](crate::client::Client::create_app_instance).
///
/// See [`crate::client::fluent_builders::CreateAppInstance`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateAppInstance {
    _private: (),
}
impl CreateAppInstance {
    /// Creates a new builder-style object to manufacture [`CreateAppInstanceInput`](crate::input::CreateAppInstanceInput)
    pub fn builder() -> crate::input::create_app_instance_input::Builder {
        crate::input::create_app_instance_input::Builder::default()
    }
    /// Creates a new `CreateAppInstance` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAppInstance {
    type Output = std::result::Result<
        crate::output::CreateAppInstanceOutput,
        crate::error::CreateAppInstanceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_app_instance_error(response)
        } else {
            crate::operation_deser::parse_create_app_instance_response(response)
        }
    }
}

/// Operation shape for `CreateAppInstanceAdmin`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_app_instance_admin`](crate::client::Client::create_app_instance_admin).
///
/// See [`crate::client::fluent_builders::CreateAppInstanceAdmin`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateAppInstanceAdmin {
    _private: (),
}
impl CreateAppInstanceAdmin {
    /// Creates a new builder-style object to manufacture [`CreateAppInstanceAdminInput`](crate::input::CreateAppInstanceAdminInput)
    pub fn builder() -> crate::input::create_app_instance_admin_input::Builder {
        crate::input::create_app_instance_admin_input::Builder::default()
    }
    /// Creates a new `CreateAppInstanceAdmin` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAppInstanceAdmin {
    type Output = std::result::Result<
        crate::output::CreateAppInstanceAdminOutput,
        crate::error::CreateAppInstanceAdminError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_app_instance_admin_error(response)
        } else {
            crate::operation_deser::parse_create_app_instance_admin_response(response)
        }
    }
}

/// Operation shape for `CreateAppInstanceUser`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_app_instance_user`](crate::client::Client::create_app_instance_user).
///
/// See [`crate::client::fluent_builders::CreateAppInstanceUser`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateAppInstanceUser {
    _private: (),
}
impl CreateAppInstanceUser {
    /// Creates a new builder-style object to manufacture [`CreateAppInstanceUserInput`](crate::input::CreateAppInstanceUserInput)
    pub fn builder() -> crate::input::create_app_instance_user_input::Builder {
        crate::input::create_app_instance_user_input::Builder::default()
    }
    /// Creates a new `CreateAppInstanceUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAppInstanceUser {
    type Output = std::result::Result<
        crate::output::CreateAppInstanceUserOutput,
        crate::error::CreateAppInstanceUserError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_app_instance_user_error(response)
        } else {
            crate::operation_deser::parse_create_app_instance_user_response(response)
        }
    }
}

/// Operation shape for `DeleteAppInstance`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_app_instance`](crate::client::Client::delete_app_instance).
///
/// See [`crate::client::fluent_builders::DeleteAppInstance`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAppInstance {
    _private: (),
}
impl DeleteAppInstance {
    /// Creates a new builder-style object to manufacture [`DeleteAppInstanceInput`](crate::input::DeleteAppInstanceInput)
    pub fn builder() -> crate::input::delete_app_instance_input::Builder {
        crate::input::delete_app_instance_input::Builder::default()
    }
    /// Creates a new `DeleteAppInstance` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAppInstance {
    type Output = std::result::Result<
        crate::output::DeleteAppInstanceOutput,
        crate::error::DeleteAppInstanceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_app_instance_error(response)
        } else {
            crate::operation_deser::parse_delete_app_instance_response(response)
        }
    }
}

/// Operation shape for `DeleteAppInstanceAdmin`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_app_instance_admin`](crate::client::Client::delete_app_instance_admin).
///
/// See [`crate::client::fluent_builders::DeleteAppInstanceAdmin`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAppInstanceAdmin {
    _private: (),
}
impl DeleteAppInstanceAdmin {
    /// Creates a new builder-style object to manufacture [`DeleteAppInstanceAdminInput`](crate::input::DeleteAppInstanceAdminInput)
    pub fn builder() -> crate::input::delete_app_instance_admin_input::Builder {
        crate::input::delete_app_instance_admin_input::Builder::default()
    }
    /// Creates a new `DeleteAppInstanceAdmin` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAppInstanceAdmin {
    type Output = std::result::Result<
        crate::output::DeleteAppInstanceAdminOutput,
        crate::error::DeleteAppInstanceAdminError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_app_instance_admin_error(response)
        } else {
            crate::operation_deser::parse_delete_app_instance_admin_response(response)
        }
    }
}

/// Operation shape for `DeleteAppInstanceUser`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_app_instance_user`](crate::client::Client::delete_app_instance_user).
///
/// See [`crate::client::fluent_builders::DeleteAppInstanceUser`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAppInstanceUser {
    _private: (),
}
impl DeleteAppInstanceUser {
    /// Creates a new builder-style object to manufacture [`DeleteAppInstanceUserInput`](crate::input::DeleteAppInstanceUserInput)
    pub fn builder() -> crate::input::delete_app_instance_user_input::Builder {
        crate::input::delete_app_instance_user_input::Builder::default()
    }
    /// Creates a new `DeleteAppInstanceUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAppInstanceUser {
    type Output = std::result::Result<
        crate::output::DeleteAppInstanceUserOutput,
        crate::error::DeleteAppInstanceUserError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_app_instance_user_error(response)
        } else {
            crate::operation_deser::parse_delete_app_instance_user_response(response)
        }
    }
}

/// Operation shape for `DeregisterAppInstanceUserEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`deregister_app_instance_user_endpoint`](crate::client::Client::deregister_app_instance_user_endpoint).
///
/// See [`crate::client::fluent_builders::DeregisterAppInstanceUserEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeregisterAppInstanceUserEndpoint {
    _private: (),
}
impl DeregisterAppInstanceUserEndpoint {
    /// Creates a new builder-style object to manufacture [`DeregisterAppInstanceUserEndpointInput`](crate::input::DeregisterAppInstanceUserEndpointInput)
    pub fn builder() -> crate::input::deregister_app_instance_user_endpoint_input::Builder {
        crate::input::deregister_app_instance_user_endpoint_input::Builder::default()
    }
    /// Creates a new `DeregisterAppInstanceUserEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeregisterAppInstanceUserEndpoint {
    type Output = std::result::Result<
        crate::output::DeregisterAppInstanceUserEndpointOutput,
        crate::error::DeregisterAppInstanceUserEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_deregister_app_instance_user_endpoint_error(response)
        } else {
            crate::operation_deser::parse_deregister_app_instance_user_endpoint_response(response)
        }
    }
}

/// Operation shape for `DescribeAppInstance`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_app_instance`](crate::client::Client::describe_app_instance).
///
/// See [`crate::client::fluent_builders::DescribeAppInstance`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAppInstance {
    _private: (),
}
impl DescribeAppInstance {
    /// Creates a new builder-style object to manufacture [`DescribeAppInstanceInput`](crate::input::DescribeAppInstanceInput)
    pub fn builder() -> crate::input::describe_app_instance_input::Builder {
        crate::input::describe_app_instance_input::Builder::default()
    }
    /// Creates a new `DescribeAppInstance` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAppInstance {
    type Output = std::result::Result<
        crate::output::DescribeAppInstanceOutput,
        crate::error::DescribeAppInstanceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_app_instance_error(response)
        } else {
            crate::operation_deser::parse_describe_app_instance_response(response)
        }
    }
}

/// Operation shape for `DescribeAppInstanceAdmin`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_app_instance_admin`](crate::client::Client::describe_app_instance_admin).
///
/// See [`crate::client::fluent_builders::DescribeAppInstanceAdmin`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAppInstanceAdmin {
    _private: (),
}
impl DescribeAppInstanceAdmin {
    /// Creates a new builder-style object to manufacture [`DescribeAppInstanceAdminInput`](crate::input::DescribeAppInstanceAdminInput)
    pub fn builder() -> crate::input::describe_app_instance_admin_input::Builder {
        crate::input::describe_app_instance_admin_input::Builder::default()
    }
    /// Creates a new `DescribeAppInstanceAdmin` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAppInstanceAdmin {
    type Output = std::result::Result<
        crate::output::DescribeAppInstanceAdminOutput,
        crate::error::DescribeAppInstanceAdminError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_app_instance_admin_error(response)
        } else {
            crate::operation_deser::parse_describe_app_instance_admin_response(response)
        }
    }
}

/// Operation shape for `DescribeAppInstanceUser`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_app_instance_user`](crate::client::Client::describe_app_instance_user).
///
/// See [`crate::client::fluent_builders::DescribeAppInstanceUser`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAppInstanceUser {
    _private: (),
}
impl DescribeAppInstanceUser {
    /// Creates a new builder-style object to manufacture [`DescribeAppInstanceUserInput`](crate::input::DescribeAppInstanceUserInput)
    pub fn builder() -> crate::input::describe_app_instance_user_input::Builder {
        crate::input::describe_app_instance_user_input::Builder::default()
    }
    /// Creates a new `DescribeAppInstanceUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAppInstanceUser {
    type Output = std::result::Result<
        crate::output::DescribeAppInstanceUserOutput,
        crate::error::DescribeAppInstanceUserError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_app_instance_user_error(response)
        } else {
            crate::operation_deser::parse_describe_app_instance_user_response(response)
        }
    }
}

/// Operation shape for `DescribeAppInstanceUserEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_app_instance_user_endpoint`](crate::client::Client::describe_app_instance_user_endpoint).
///
/// See [`crate::client::fluent_builders::DescribeAppInstanceUserEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAppInstanceUserEndpoint {
    _private: (),
}
impl DescribeAppInstanceUserEndpoint {
    /// Creates a new builder-style object to manufacture [`DescribeAppInstanceUserEndpointInput`](crate::input::DescribeAppInstanceUserEndpointInput)
    pub fn builder() -> crate::input::describe_app_instance_user_endpoint_input::Builder {
        crate::input::describe_app_instance_user_endpoint_input::Builder::default()
    }
    /// Creates a new `DescribeAppInstanceUserEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAppInstanceUserEndpoint {
    type Output = std::result::Result<
        crate::output::DescribeAppInstanceUserEndpointOutput,
        crate::error::DescribeAppInstanceUserEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_app_instance_user_endpoint_error(response)
        } else {
            crate::operation_deser::parse_describe_app_instance_user_endpoint_response(response)
        }
    }
}

/// Operation shape for `GetAppInstanceRetentionSettings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_app_instance_retention_settings`](crate::client::Client::get_app_instance_retention_settings).
///
/// See [`crate::client::fluent_builders::GetAppInstanceRetentionSettings`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetAppInstanceRetentionSettings {
    _private: (),
}
impl GetAppInstanceRetentionSettings {
    /// Creates a new builder-style object to manufacture [`GetAppInstanceRetentionSettingsInput`](crate::input::GetAppInstanceRetentionSettingsInput)
    pub fn builder() -> crate::input::get_app_instance_retention_settings_input::Builder {
        crate::input::get_app_instance_retention_settings_input::Builder::default()
    }
    /// Creates a new `GetAppInstanceRetentionSettings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAppInstanceRetentionSettings {
    type Output = std::result::Result<
        crate::output::GetAppInstanceRetentionSettingsOutput,
        crate::error::GetAppInstanceRetentionSettingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_app_instance_retention_settings_error(response)
        } else {
            crate::operation_deser::parse_get_app_instance_retention_settings_response(response)
        }
    }
}

/// Operation shape for `ListAppInstanceAdmins`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_app_instance_admins`](crate::client::Client::list_app_instance_admins).
///
/// See [`crate::client::fluent_builders::ListAppInstanceAdmins`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAppInstanceAdmins {
    _private: (),
}
impl ListAppInstanceAdmins {
    /// Creates a new builder-style object to manufacture [`ListAppInstanceAdminsInput`](crate::input::ListAppInstanceAdminsInput)
    pub fn builder() -> crate::input::list_app_instance_admins_input::Builder {
        crate::input::list_app_instance_admins_input::Builder::default()
    }
    /// Creates a new `ListAppInstanceAdmins` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAppInstanceAdmins {
    type Output = std::result::Result<
        crate::output::ListAppInstanceAdminsOutput,
        crate::error::ListAppInstanceAdminsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_app_instance_admins_error(response)
        } else {
            crate::operation_deser::parse_list_app_instance_admins_response(response)
        }
    }
}

/// Operation shape for `ListAppInstances`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_app_instances`](crate::client::Client::list_app_instances).
///
/// See [`crate::client::fluent_builders::ListAppInstances`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAppInstances {
    _private: (),
}
impl ListAppInstances {
    /// Creates a new builder-style object to manufacture [`ListAppInstancesInput`](crate::input::ListAppInstancesInput)
    pub fn builder() -> crate::input::list_app_instances_input::Builder {
        crate::input::list_app_instances_input::Builder::default()
    }
    /// Creates a new `ListAppInstances` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAppInstances {
    type Output = std::result::Result<
        crate::output::ListAppInstancesOutput,
        crate::error::ListAppInstancesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_app_instances_error(response)
        } else {
            crate::operation_deser::parse_list_app_instances_response(response)
        }
    }
}

/// Operation shape for `ListAppInstanceUserEndpoints`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_app_instance_user_endpoints`](crate::client::Client::list_app_instance_user_endpoints).
///
/// See [`crate::client::fluent_builders::ListAppInstanceUserEndpoints`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAppInstanceUserEndpoints {
    _private: (),
}
impl ListAppInstanceUserEndpoints {
    /// Creates a new builder-style object to manufacture [`ListAppInstanceUserEndpointsInput`](crate::input::ListAppInstanceUserEndpointsInput)
    pub fn builder() -> crate::input::list_app_instance_user_endpoints_input::Builder {
        crate::input::list_app_instance_user_endpoints_input::Builder::default()
    }
    /// Creates a new `ListAppInstanceUserEndpoints` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAppInstanceUserEndpoints {
    type Output = std::result::Result<
        crate::output::ListAppInstanceUserEndpointsOutput,
        crate::error::ListAppInstanceUserEndpointsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_app_instance_user_endpoints_error(response)
        } else {
            crate::operation_deser::parse_list_app_instance_user_endpoints_response(response)
        }
    }
}

/// Operation shape for `ListAppInstanceUsers`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_app_instance_users`](crate::client::Client::list_app_instance_users).
///
/// See [`crate::client::fluent_builders::ListAppInstanceUsers`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAppInstanceUsers {
    _private: (),
}
impl ListAppInstanceUsers {
    /// Creates a new builder-style object to manufacture [`ListAppInstanceUsersInput`](crate::input::ListAppInstanceUsersInput)
    pub fn builder() -> crate::input::list_app_instance_users_input::Builder {
        crate::input::list_app_instance_users_input::Builder::default()
    }
    /// Creates a new `ListAppInstanceUsers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAppInstanceUsers {
    type Output = std::result::Result<
        crate::output::ListAppInstanceUsersOutput,
        crate::error::ListAppInstanceUsersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_app_instance_users_error(response)
        } else {
            crate::operation_deser::parse_list_app_instance_users_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `PutAppInstanceRetentionSettings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_app_instance_retention_settings`](crate::client::Client::put_app_instance_retention_settings).
///
/// See [`crate::client::fluent_builders::PutAppInstanceRetentionSettings`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutAppInstanceRetentionSettings {
    _private: (),
}
impl PutAppInstanceRetentionSettings {
    /// Creates a new builder-style object to manufacture [`PutAppInstanceRetentionSettingsInput`](crate::input::PutAppInstanceRetentionSettingsInput)
    pub fn builder() -> crate::input::put_app_instance_retention_settings_input::Builder {
        crate::input::put_app_instance_retention_settings_input::Builder::default()
    }
    /// Creates a new `PutAppInstanceRetentionSettings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutAppInstanceRetentionSettings {
    type Output = std::result::Result<
        crate::output::PutAppInstanceRetentionSettingsOutput,
        crate::error::PutAppInstanceRetentionSettingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_app_instance_retention_settings_error(response)
        } else {
            crate::operation_deser::parse_put_app_instance_retention_settings_response(response)
        }
    }
}

/// Operation shape for `RegisterAppInstanceUserEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_app_instance_user_endpoint`](crate::client::Client::register_app_instance_user_endpoint).
///
/// See [`crate::client::fluent_builders::RegisterAppInstanceUserEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RegisterAppInstanceUserEndpoint {
    _private: (),
}
impl RegisterAppInstanceUserEndpoint {
    /// Creates a new builder-style object to manufacture [`RegisterAppInstanceUserEndpointInput`](crate::input::RegisterAppInstanceUserEndpointInput)
    pub fn builder() -> crate::input::register_app_instance_user_endpoint_input::Builder {
        crate::input::register_app_instance_user_endpoint_input::Builder::default()
    }
    /// Creates a new `RegisterAppInstanceUserEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterAppInstanceUserEndpoint {
    type Output = std::result::Result<
        crate::output::RegisterAppInstanceUserEndpointOutput,
        crate::error::RegisterAppInstanceUserEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_register_app_instance_user_endpoint_error(response)
        } else {
            crate::operation_deser::parse_register_app_instance_user_endpoint_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateAppInstance`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_app_instance`](crate::client::Client::update_app_instance).
///
/// See [`crate::client::fluent_builders::UpdateAppInstance`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAppInstance {
    _private: (),
}
impl UpdateAppInstance {
    /// Creates a new builder-style object to manufacture [`UpdateAppInstanceInput`](crate::input::UpdateAppInstanceInput)
    pub fn builder() -> crate::input::update_app_instance_input::Builder {
        crate::input::update_app_instance_input::Builder::default()
    }
    /// Creates a new `UpdateAppInstance` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAppInstance {
    type Output = std::result::Result<
        crate::output::UpdateAppInstanceOutput,
        crate::error::UpdateAppInstanceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_app_instance_error(response)
        } else {
            crate::operation_deser::parse_update_app_instance_response(response)
        }
    }
}

/// Operation shape for `UpdateAppInstanceUser`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_app_instance_user`](crate::client::Client::update_app_instance_user).
///
/// See [`crate::client::fluent_builders::UpdateAppInstanceUser`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAppInstanceUser {
    _private: (),
}
impl UpdateAppInstanceUser {
    /// Creates a new builder-style object to manufacture [`UpdateAppInstanceUserInput`](crate::input::UpdateAppInstanceUserInput)
    pub fn builder() -> crate::input::update_app_instance_user_input::Builder {
        crate::input::update_app_instance_user_input::Builder::default()
    }
    /// Creates a new `UpdateAppInstanceUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAppInstanceUser {
    type Output = std::result::Result<
        crate::output::UpdateAppInstanceUserOutput,
        crate::error::UpdateAppInstanceUserError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_app_instance_user_error(response)
        } else {
            crate::operation_deser::parse_update_app_instance_user_response(response)
        }
    }
}

/// Operation shape for `UpdateAppInstanceUserEndpoint`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_app_instance_user_endpoint`](crate::client::Client::update_app_instance_user_endpoint).
///
/// See [`crate::client::fluent_builders::UpdateAppInstanceUserEndpoint`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAppInstanceUserEndpoint {
    _private: (),
}
impl UpdateAppInstanceUserEndpoint {
    /// Creates a new builder-style object to manufacture [`UpdateAppInstanceUserEndpointInput`](crate::input::UpdateAppInstanceUserEndpointInput)
    pub fn builder() -> crate::input::update_app_instance_user_endpoint_input::Builder {
        crate::input::update_app_instance_user_endpoint_input::Builder::default()
    }
    /// Creates a new `UpdateAppInstanceUserEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAppInstanceUserEndpoint {
    type Output = std::result::Result<
        crate::output::UpdateAppInstanceUserEndpointOutput,
        crate::error::UpdateAppInstanceUserEndpointError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_app_instance_user_endpoint_error(response)
        } else {
            crate::operation_deser::parse_update_app_instance_user_endpoint_response(response)
        }
    }
}
