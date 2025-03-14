// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateCluster`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_cluster`](crate::client::Client::create_cluster).
///
/// See [`crate::client::fluent_builders::CreateCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateCluster {
    _private: (),
}
impl CreateCluster {
    /// Creates a new builder-style object to manufacture [`CreateClusterInput`](crate::input::CreateClusterInput)
    pub fn builder() -> crate::input::create_cluster_input::Builder {
        crate::input::create_cluster_input::Builder::default()
    }
    /// Creates a new `CreateCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCluster {
    type Output =
        std::result::Result<crate::output::CreateClusterOutput, crate::error::CreateClusterError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_cluster_error(response)
        } else {
            crate::operation_deser::parse_create_cluster_response(response)
        }
    }
}

/// Operation shape for `CreateParameterGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_parameter_group`](crate::client::Client::create_parameter_group).
///
/// See [`crate::client::fluent_builders::CreateParameterGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateParameterGroup {
    _private: (),
}
impl CreateParameterGroup {
    /// Creates a new builder-style object to manufacture [`CreateParameterGroupInput`](crate::input::CreateParameterGroupInput)
    pub fn builder() -> crate::input::create_parameter_group_input::Builder {
        crate::input::create_parameter_group_input::Builder::default()
    }
    /// Creates a new `CreateParameterGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateParameterGroup {
    type Output = std::result::Result<
        crate::output::CreateParameterGroupOutput,
        crate::error::CreateParameterGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_parameter_group_error(response)
        } else {
            crate::operation_deser::parse_create_parameter_group_response(response)
        }
    }
}

/// Operation shape for `CreateSubnetGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_subnet_group`](crate::client::Client::create_subnet_group).
///
/// See [`crate::client::fluent_builders::CreateSubnetGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateSubnetGroup {
    _private: (),
}
impl CreateSubnetGroup {
    /// Creates a new builder-style object to manufacture [`CreateSubnetGroupInput`](crate::input::CreateSubnetGroupInput)
    pub fn builder() -> crate::input::create_subnet_group_input::Builder {
        crate::input::create_subnet_group_input::Builder::default()
    }
    /// Creates a new `CreateSubnetGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSubnetGroup {
    type Output = std::result::Result<
        crate::output::CreateSubnetGroupOutput,
        crate::error::CreateSubnetGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_subnet_group_error(response)
        } else {
            crate::operation_deser::parse_create_subnet_group_response(response)
        }
    }
}

/// Operation shape for `DecreaseReplicationFactor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`decrease_replication_factor`](crate::client::Client::decrease_replication_factor).
///
/// See [`crate::client::fluent_builders::DecreaseReplicationFactor`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DecreaseReplicationFactor {
    _private: (),
}
impl DecreaseReplicationFactor {
    /// Creates a new builder-style object to manufacture [`DecreaseReplicationFactorInput`](crate::input::DecreaseReplicationFactorInput)
    pub fn builder() -> crate::input::decrease_replication_factor_input::Builder {
        crate::input::decrease_replication_factor_input::Builder::default()
    }
    /// Creates a new `DecreaseReplicationFactor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DecreaseReplicationFactor {
    type Output = std::result::Result<
        crate::output::DecreaseReplicationFactorOutput,
        crate::error::DecreaseReplicationFactorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_decrease_replication_factor_error(response)
        } else {
            crate::operation_deser::parse_decrease_replication_factor_response(response)
        }
    }
}

/// Operation shape for `DeleteCluster`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_cluster`](crate::client::Client::delete_cluster).
///
/// See [`crate::client::fluent_builders::DeleteCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteCluster {
    _private: (),
}
impl DeleteCluster {
    /// Creates a new builder-style object to manufacture [`DeleteClusterInput`](crate::input::DeleteClusterInput)
    pub fn builder() -> crate::input::delete_cluster_input::Builder {
        crate::input::delete_cluster_input::Builder::default()
    }
    /// Creates a new `DeleteCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteCluster {
    type Output =
        std::result::Result<crate::output::DeleteClusterOutput, crate::error::DeleteClusterError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_cluster_error(response)
        } else {
            crate::operation_deser::parse_delete_cluster_response(response)
        }
    }
}

/// Operation shape for `DeleteParameterGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_parameter_group`](crate::client::Client::delete_parameter_group).
///
/// See [`crate::client::fluent_builders::DeleteParameterGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteParameterGroup {
    _private: (),
}
impl DeleteParameterGroup {
    /// Creates a new builder-style object to manufacture [`DeleteParameterGroupInput`](crate::input::DeleteParameterGroupInput)
    pub fn builder() -> crate::input::delete_parameter_group_input::Builder {
        crate::input::delete_parameter_group_input::Builder::default()
    }
    /// Creates a new `DeleteParameterGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteParameterGroup {
    type Output = std::result::Result<
        crate::output::DeleteParameterGroupOutput,
        crate::error::DeleteParameterGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_parameter_group_error(response)
        } else {
            crate::operation_deser::parse_delete_parameter_group_response(response)
        }
    }
}

/// Operation shape for `DeleteSubnetGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_subnet_group`](crate::client::Client::delete_subnet_group).
///
/// See [`crate::client::fluent_builders::DeleteSubnetGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSubnetGroup {
    _private: (),
}
impl DeleteSubnetGroup {
    /// Creates a new builder-style object to manufacture [`DeleteSubnetGroupInput`](crate::input::DeleteSubnetGroupInput)
    pub fn builder() -> crate::input::delete_subnet_group_input::Builder {
        crate::input::delete_subnet_group_input::Builder::default()
    }
    /// Creates a new `DeleteSubnetGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSubnetGroup {
    type Output = std::result::Result<
        crate::output::DeleteSubnetGroupOutput,
        crate::error::DeleteSubnetGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_subnet_group_error(response)
        } else {
            crate::operation_deser::parse_delete_subnet_group_response(response)
        }
    }
}

/// Operation shape for `DescribeClusters`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_clusters`](crate::client::Client::describe_clusters).
///
/// See [`crate::client::fluent_builders::DescribeClusters`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeClusters {
    _private: (),
}
impl DescribeClusters {
    /// Creates a new builder-style object to manufacture [`DescribeClustersInput`](crate::input::DescribeClustersInput)
    pub fn builder() -> crate::input::describe_clusters_input::Builder {
        crate::input::describe_clusters_input::Builder::default()
    }
    /// Creates a new `DescribeClusters` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeClusters {
    type Output = std::result::Result<
        crate::output::DescribeClustersOutput,
        crate::error::DescribeClustersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_clusters_error(response)
        } else {
            crate::operation_deser::parse_describe_clusters_response(response)
        }
    }
}

/// Operation shape for `DescribeDefaultParameters`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_default_parameters`](crate::client::Client::describe_default_parameters).
///
/// See [`crate::client::fluent_builders::DescribeDefaultParameters`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDefaultParameters {
    _private: (),
}
impl DescribeDefaultParameters {
    /// Creates a new builder-style object to manufacture [`DescribeDefaultParametersInput`](crate::input::DescribeDefaultParametersInput)
    pub fn builder() -> crate::input::describe_default_parameters_input::Builder {
        crate::input::describe_default_parameters_input::Builder::default()
    }
    /// Creates a new `DescribeDefaultParameters` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDefaultParameters {
    type Output = std::result::Result<
        crate::output::DescribeDefaultParametersOutput,
        crate::error::DescribeDefaultParametersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_default_parameters_error(response)
        } else {
            crate::operation_deser::parse_describe_default_parameters_response(response)
        }
    }
}

/// Operation shape for `DescribeEvents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_events`](crate::client::Client::describe_events).
///
/// See [`crate::client::fluent_builders::DescribeEvents`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEvents {
    _private: (),
}
impl DescribeEvents {
    /// Creates a new builder-style object to manufacture [`DescribeEventsInput`](crate::input::DescribeEventsInput)
    pub fn builder() -> crate::input::describe_events_input::Builder {
        crate::input::describe_events_input::Builder::default()
    }
    /// Creates a new `DescribeEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEvents {
    type Output =
        std::result::Result<crate::output::DescribeEventsOutput, crate::error::DescribeEventsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_events_error(response)
        } else {
            crate::operation_deser::parse_describe_events_response(response)
        }
    }
}

/// Operation shape for `DescribeParameterGroups`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_parameter_groups`](crate::client::Client::describe_parameter_groups).
///
/// See [`crate::client::fluent_builders::DescribeParameterGroups`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeParameterGroups {
    _private: (),
}
impl DescribeParameterGroups {
    /// Creates a new builder-style object to manufacture [`DescribeParameterGroupsInput`](crate::input::DescribeParameterGroupsInput)
    pub fn builder() -> crate::input::describe_parameter_groups_input::Builder {
        crate::input::describe_parameter_groups_input::Builder::default()
    }
    /// Creates a new `DescribeParameterGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeParameterGroups {
    type Output = std::result::Result<
        crate::output::DescribeParameterGroupsOutput,
        crate::error::DescribeParameterGroupsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_parameter_groups_error(response)
        } else {
            crate::operation_deser::parse_describe_parameter_groups_response(response)
        }
    }
}

/// Operation shape for `DescribeParameters`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_parameters`](crate::client::Client::describe_parameters).
///
/// See [`crate::client::fluent_builders::DescribeParameters`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeParameters {
    _private: (),
}
impl DescribeParameters {
    /// Creates a new builder-style object to manufacture [`DescribeParametersInput`](crate::input::DescribeParametersInput)
    pub fn builder() -> crate::input::describe_parameters_input::Builder {
        crate::input::describe_parameters_input::Builder::default()
    }
    /// Creates a new `DescribeParameters` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeParameters {
    type Output = std::result::Result<
        crate::output::DescribeParametersOutput,
        crate::error::DescribeParametersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_parameters_error(response)
        } else {
            crate::operation_deser::parse_describe_parameters_response(response)
        }
    }
}

/// Operation shape for `DescribeSubnetGroups`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_subnet_groups`](crate::client::Client::describe_subnet_groups).
///
/// See [`crate::client::fluent_builders::DescribeSubnetGroups`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSubnetGroups {
    _private: (),
}
impl DescribeSubnetGroups {
    /// Creates a new builder-style object to manufacture [`DescribeSubnetGroupsInput`](crate::input::DescribeSubnetGroupsInput)
    pub fn builder() -> crate::input::describe_subnet_groups_input::Builder {
        crate::input::describe_subnet_groups_input::Builder::default()
    }
    /// Creates a new `DescribeSubnetGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSubnetGroups {
    type Output = std::result::Result<
        crate::output::DescribeSubnetGroupsOutput,
        crate::error::DescribeSubnetGroupsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_subnet_groups_error(response)
        } else {
            crate::operation_deser::parse_describe_subnet_groups_response(response)
        }
    }
}

/// Operation shape for `IncreaseReplicationFactor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`increase_replication_factor`](crate::client::Client::increase_replication_factor).
///
/// See [`crate::client::fluent_builders::IncreaseReplicationFactor`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct IncreaseReplicationFactor {
    _private: (),
}
impl IncreaseReplicationFactor {
    /// Creates a new builder-style object to manufacture [`IncreaseReplicationFactorInput`](crate::input::IncreaseReplicationFactorInput)
    pub fn builder() -> crate::input::increase_replication_factor_input::Builder {
        crate::input::increase_replication_factor_input::Builder::default()
    }
    /// Creates a new `IncreaseReplicationFactor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for IncreaseReplicationFactor {
    type Output = std::result::Result<
        crate::output::IncreaseReplicationFactorOutput,
        crate::error::IncreaseReplicationFactorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_increase_replication_factor_error(response)
        } else {
            crate::operation_deser::parse_increase_replication_factor_response(response)
        }
    }
}

/// Operation shape for `ListTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags`](crate::client::Client::list_tags).
///
/// See [`crate::client::fluent_builders::ListTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTags {
    _private: (),
}
impl ListTags {
    /// Creates a new builder-style object to manufacture [`ListTagsInput`](crate::input::ListTagsInput)
    pub fn builder() -> crate::input::list_tags_input::Builder {
        crate::input::list_tags_input::Builder::default()
    }
    /// Creates a new `ListTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTags {
    type Output = std::result::Result<crate::output::ListTagsOutput, crate::error::ListTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_error(response)
        } else {
            crate::operation_deser::parse_list_tags_response(response)
        }
    }
}

/// Operation shape for `RebootNode`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`reboot_node`](crate::client::Client::reboot_node).
///
/// See [`crate::client::fluent_builders::RebootNode`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RebootNode {
    _private: (),
}
impl RebootNode {
    /// Creates a new builder-style object to manufacture [`RebootNodeInput`](crate::input::RebootNodeInput)
    pub fn builder() -> crate::input::reboot_node_input::Builder {
        crate::input::reboot_node_input::Builder::default()
    }
    /// Creates a new `RebootNode` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RebootNode {
    type Output =
        std::result::Result<crate::output::RebootNodeOutput, crate::error::RebootNodeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_reboot_node_error(response)
        } else {
            crate::operation_deser::parse_reboot_node_response(response)
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
        if !response.status().is_success() && response.status().as_u16() != 200 {
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
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateCluster`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_cluster`](crate::client::Client::update_cluster).
///
/// See [`crate::client::fluent_builders::UpdateCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateCluster {
    _private: (),
}
impl UpdateCluster {
    /// Creates a new builder-style object to manufacture [`UpdateClusterInput`](crate::input::UpdateClusterInput)
    pub fn builder() -> crate::input::update_cluster_input::Builder {
        crate::input::update_cluster_input::Builder::default()
    }
    /// Creates a new `UpdateCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateCluster {
    type Output =
        std::result::Result<crate::output::UpdateClusterOutput, crate::error::UpdateClusterError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_cluster_error(response)
        } else {
            crate::operation_deser::parse_update_cluster_response(response)
        }
    }
}

/// Operation shape for `UpdateParameterGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_parameter_group`](crate::client::Client::update_parameter_group).
///
/// See [`crate::client::fluent_builders::UpdateParameterGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateParameterGroup {
    _private: (),
}
impl UpdateParameterGroup {
    /// Creates a new builder-style object to manufacture [`UpdateParameterGroupInput`](crate::input::UpdateParameterGroupInput)
    pub fn builder() -> crate::input::update_parameter_group_input::Builder {
        crate::input::update_parameter_group_input::Builder::default()
    }
    /// Creates a new `UpdateParameterGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateParameterGroup {
    type Output = std::result::Result<
        crate::output::UpdateParameterGroupOutput,
        crate::error::UpdateParameterGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_parameter_group_error(response)
        } else {
            crate::operation_deser::parse_update_parameter_group_response(response)
        }
    }
}

/// Operation shape for `UpdateSubnetGroup`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_subnet_group`](crate::client::Client::update_subnet_group).
///
/// See [`crate::client::fluent_builders::UpdateSubnetGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSubnetGroup {
    _private: (),
}
impl UpdateSubnetGroup {
    /// Creates a new builder-style object to manufacture [`UpdateSubnetGroupInput`](crate::input::UpdateSubnetGroupInput)
    pub fn builder() -> crate::input::update_subnet_group_input::Builder {
        crate::input::update_subnet_group_input::Builder::default()
    }
    /// Creates a new `UpdateSubnetGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSubnetGroup {
    type Output = std::result::Result<
        crate::output::UpdateSubnetGroupOutput,
        crate::error::UpdateSubnetGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_subnet_group_error(response)
        } else {
            crate::operation_deser::parse_update_subnet_group_response(response)
        }
    }
}
