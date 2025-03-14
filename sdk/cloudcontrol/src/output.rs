// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateResourceOutput {
    /// <p>Represents the current status of the resource update request.</p>
    /// <p>Use the <code>RequestToken</code> of the <code>ProgressEvent</code> with <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> to return the current status of a resource operation request.</p>
    pub progress_event: std::option::Option<crate::model::ProgressEvent>,
}
impl UpdateResourceOutput {
    /// <p>Represents the current status of the resource update request.</p>
    /// <p>Use the <code>RequestToken</code> of the <code>ProgressEvent</code> with <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> to return the current status of a resource operation request.</p>
    pub fn progress_event(&self) -> std::option::Option<&crate::model::ProgressEvent> {
        self.progress_event.as_ref()
    }
}
impl std::fmt::Debug for UpdateResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateResourceOutput");
        formatter.field("progress_event", &self.progress_event);
        formatter.finish()
    }
}
/// See [`UpdateResourceOutput`](crate::output::UpdateResourceOutput)
pub mod update_resource_output {
    /// A builder for [`UpdateResourceOutput`](crate::output::UpdateResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) progress_event: std::option::Option<crate::model::ProgressEvent>,
    }
    impl Builder {
        /// <p>Represents the current status of the resource update request.</p>
        /// <p>Use the <code>RequestToken</code> of the <code>ProgressEvent</code> with <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> to return the current status of a resource operation request.</p>
        pub fn progress_event(mut self, input: crate::model::ProgressEvent) -> Self {
            self.progress_event = Some(input);
            self
        }
        /// <p>Represents the current status of the resource update request.</p>
        /// <p>Use the <code>RequestToken</code> of the <code>ProgressEvent</code> with <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> to return the current status of a resource operation request.</p>
        pub fn set_progress_event(
            mut self,
            input: std::option::Option<crate::model::ProgressEvent>,
        ) -> Self {
            self.progress_event = input;
            self
        }
        /// Consumes the builder and constructs a [`UpdateResourceOutput`](crate::output::UpdateResourceOutput)
        pub fn build(self) -> crate::output::UpdateResourceOutput {
            crate::output::UpdateResourceOutput {
                progress_event: self.progress_event,
            }
        }
    }
}
impl UpdateResourceOutput {
    /// Creates a new builder-style object to manufacture [`UpdateResourceOutput`](crate::output::UpdateResourceOutput)
    pub fn builder() -> crate::output::update_resource_output::Builder {
        crate::output::update_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListResourcesOutput {
    /// <p>The name of the resource type.</p>
    pub type_name: std::option::Option<std::string::String>,
    /// <p>Information about the specified resources, including primary identifier and resource model.</p>
    pub resource_descriptions:
        std::option::Option<std::vec::Vec<crate::model::ResourceDescription>>,
    /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListResources</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to null.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListResourcesOutput {
    /// <p>The name of the resource type.</p>
    pub fn type_name(&self) -> std::option::Option<&str> {
        self.type_name.as_deref()
    }
    /// <p>Information about the specified resources, including primary identifier and resource model.</p>
    pub fn resource_descriptions(
        &self,
    ) -> std::option::Option<&[crate::model::ResourceDescription]> {
        self.resource_descriptions.as_deref()
    }
    /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListResources</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to null.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListResourcesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListResourcesOutput");
        formatter.field("type_name", &self.type_name);
        formatter.field("resource_descriptions", &self.resource_descriptions);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListResourcesOutput`](crate::output::ListResourcesOutput)
pub mod list_resources_output {
    /// A builder for [`ListResourcesOutput`](crate::output::ListResourcesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) type_name: std::option::Option<std::string::String>,
        pub(crate) resource_descriptions:
            std::option::Option<std::vec::Vec<crate::model::ResourceDescription>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the resource type.</p>
        pub fn type_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.type_name = Some(input.into());
            self
        }
        /// <p>The name of the resource type.</p>
        pub fn set_type_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.type_name = input;
            self
        }
        /// Appends an item to `resource_descriptions`.
        ///
        /// To override the contents of this collection use [`set_resource_descriptions`](Self::set_resource_descriptions).
        ///
        /// <p>Information about the specified resources, including primary identifier and resource model.</p>
        pub fn resource_descriptions(mut self, input: crate::model::ResourceDescription) -> Self {
            let mut v = self.resource_descriptions.unwrap_or_default();
            v.push(input);
            self.resource_descriptions = Some(v);
            self
        }
        /// <p>Information about the specified resources, including primary identifier and resource model.</p>
        pub fn set_resource_descriptions(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ResourceDescription>>,
        ) -> Self {
            self.resource_descriptions = input;
            self
        }
        /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListResources</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to null.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListResources</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to null.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListResourcesOutput`](crate::output::ListResourcesOutput)
        pub fn build(self) -> crate::output::ListResourcesOutput {
            crate::output::ListResourcesOutput {
                type_name: self.type_name,
                resource_descriptions: self.resource_descriptions,
                next_token: self.next_token,
            }
        }
    }
}
impl ListResourcesOutput {
    /// Creates a new builder-style object to manufacture [`ListResourcesOutput`](crate::output::ListResourcesOutput)
    pub fn builder() -> crate::output::list_resources_output::Builder {
        crate::output::list_resources_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListResourceRequestsOutput {
    /// <p>The requests that match the specified filter criteria.</p>
    pub resource_request_status_summaries:
        std::option::Option<std::vec::Vec<crate::model::ProgressEvent>>,
    /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListResources</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to null.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListResourceRequestsOutput {
    /// <p>The requests that match the specified filter criteria.</p>
    pub fn resource_request_status_summaries(
        &self,
    ) -> std::option::Option<&[crate::model::ProgressEvent]> {
        self.resource_request_status_summaries.as_deref()
    }
    /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListResources</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to null.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListResourceRequestsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListResourceRequestsOutput");
        formatter.field(
            "resource_request_status_summaries",
            &self.resource_request_status_summaries,
        );
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListResourceRequestsOutput`](crate::output::ListResourceRequestsOutput)
pub mod list_resource_requests_output {
    /// A builder for [`ListResourceRequestsOutput`](crate::output::ListResourceRequestsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) resource_request_status_summaries:
            std::option::Option<std::vec::Vec<crate::model::ProgressEvent>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `resource_request_status_summaries`.
        ///
        /// To override the contents of this collection use [`set_resource_request_status_summaries`](Self::set_resource_request_status_summaries).
        ///
        /// <p>The requests that match the specified filter criteria.</p>
        pub fn resource_request_status_summaries(
            mut self,
            input: crate::model::ProgressEvent,
        ) -> Self {
            let mut v = self.resource_request_status_summaries.unwrap_or_default();
            v.push(input);
            self.resource_request_status_summaries = Some(v);
            self
        }
        /// <p>The requests that match the specified filter criteria.</p>
        pub fn set_resource_request_status_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ProgressEvent>>,
        ) -> Self {
            self.resource_request_status_summaries = input;
            self
        }
        /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListResources</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to null.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If the request doesn't return all of the remaining results, <code>NextToken</code> is set to a token. To retrieve the next set of results, call <code>ListResources</code> again and assign that token to the request object's <code>NextToken</code> parameter. If the request returns all results, <code>NextToken</code> is set to null.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListResourceRequestsOutput`](crate::output::ListResourceRequestsOutput)
        pub fn build(self) -> crate::output::ListResourceRequestsOutput {
            crate::output::ListResourceRequestsOutput {
                resource_request_status_summaries: self.resource_request_status_summaries,
                next_token: self.next_token,
            }
        }
    }
}
impl ListResourceRequestsOutput {
    /// Creates a new builder-style object to manufacture [`ListResourceRequestsOutput`](crate::output::ListResourceRequestsOutput)
    pub fn builder() -> crate::output::list_resource_requests_output::Builder {
        crate::output::list_resource_requests_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetResourceRequestStatusOutput {
    /// <p>Represents the current status of the resource operation request.</p>
    pub progress_event: std::option::Option<crate::model::ProgressEvent>,
}
impl GetResourceRequestStatusOutput {
    /// <p>Represents the current status of the resource operation request.</p>
    pub fn progress_event(&self) -> std::option::Option<&crate::model::ProgressEvent> {
        self.progress_event.as_ref()
    }
}
impl std::fmt::Debug for GetResourceRequestStatusOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetResourceRequestStatusOutput");
        formatter.field("progress_event", &self.progress_event);
        formatter.finish()
    }
}
/// See [`GetResourceRequestStatusOutput`](crate::output::GetResourceRequestStatusOutput)
pub mod get_resource_request_status_output {
    /// A builder for [`GetResourceRequestStatusOutput`](crate::output::GetResourceRequestStatusOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) progress_event: std::option::Option<crate::model::ProgressEvent>,
    }
    impl Builder {
        /// <p>Represents the current status of the resource operation request.</p>
        pub fn progress_event(mut self, input: crate::model::ProgressEvent) -> Self {
            self.progress_event = Some(input);
            self
        }
        /// <p>Represents the current status of the resource operation request.</p>
        pub fn set_progress_event(
            mut self,
            input: std::option::Option<crate::model::ProgressEvent>,
        ) -> Self {
            self.progress_event = input;
            self
        }
        /// Consumes the builder and constructs a [`GetResourceRequestStatusOutput`](crate::output::GetResourceRequestStatusOutput)
        pub fn build(self) -> crate::output::GetResourceRequestStatusOutput {
            crate::output::GetResourceRequestStatusOutput {
                progress_event: self.progress_event,
            }
        }
    }
}
impl GetResourceRequestStatusOutput {
    /// Creates a new builder-style object to manufacture [`GetResourceRequestStatusOutput`](crate::output::GetResourceRequestStatusOutput)
    pub fn builder() -> crate::output::get_resource_request_status_output::Builder {
        crate::output::get_resource_request_status_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetResourceOutput {
    /// <p>The name of the resource type.</p>
    pub type_name: std::option::Option<std::string::String>,
    /// <p>Represents information about a provisioned resource.</p>
    pub resource_description: std::option::Option<crate::model::ResourceDescription>,
}
impl GetResourceOutput {
    /// <p>The name of the resource type.</p>
    pub fn type_name(&self) -> std::option::Option<&str> {
        self.type_name.as_deref()
    }
    /// <p>Represents information about a provisioned resource.</p>
    pub fn resource_description(&self) -> std::option::Option<&crate::model::ResourceDescription> {
        self.resource_description.as_ref()
    }
}
impl std::fmt::Debug for GetResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetResourceOutput");
        formatter.field("type_name", &self.type_name);
        formatter.field("resource_description", &self.resource_description);
        formatter.finish()
    }
}
/// See [`GetResourceOutput`](crate::output::GetResourceOutput)
pub mod get_resource_output {
    /// A builder for [`GetResourceOutput`](crate::output::GetResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) type_name: std::option::Option<std::string::String>,
        pub(crate) resource_description: std::option::Option<crate::model::ResourceDescription>,
    }
    impl Builder {
        /// <p>The name of the resource type.</p>
        pub fn type_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.type_name = Some(input.into());
            self
        }
        /// <p>The name of the resource type.</p>
        pub fn set_type_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.type_name = input;
            self
        }
        /// <p>Represents information about a provisioned resource.</p>
        pub fn resource_description(mut self, input: crate::model::ResourceDescription) -> Self {
            self.resource_description = Some(input);
            self
        }
        /// <p>Represents information about a provisioned resource.</p>
        pub fn set_resource_description(
            mut self,
            input: std::option::Option<crate::model::ResourceDescription>,
        ) -> Self {
            self.resource_description = input;
            self
        }
        /// Consumes the builder and constructs a [`GetResourceOutput`](crate::output::GetResourceOutput)
        pub fn build(self) -> crate::output::GetResourceOutput {
            crate::output::GetResourceOutput {
                type_name: self.type_name,
                resource_description: self.resource_description,
            }
        }
    }
}
impl GetResourceOutput {
    /// Creates a new builder-style object to manufacture [`GetResourceOutput`](crate::output::GetResourceOutput)
    pub fn builder() -> crate::output::get_resource_output::Builder {
        crate::output::get_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteResourceOutput {
    /// <p>Represents the current status of the resource deletion request.</p>
    /// <p>After you have initiated a resource deletion request, you can monitor the progress of your request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the <code>ProgressEvent</code> returned by <code>DeleteResource</code>.</p>
    pub progress_event: std::option::Option<crate::model::ProgressEvent>,
}
impl DeleteResourceOutput {
    /// <p>Represents the current status of the resource deletion request.</p>
    /// <p>After you have initiated a resource deletion request, you can monitor the progress of your request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the <code>ProgressEvent</code> returned by <code>DeleteResource</code>.</p>
    pub fn progress_event(&self) -> std::option::Option<&crate::model::ProgressEvent> {
        self.progress_event.as_ref()
    }
}
impl std::fmt::Debug for DeleteResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteResourceOutput");
        formatter.field("progress_event", &self.progress_event);
        formatter.finish()
    }
}
/// See [`DeleteResourceOutput`](crate::output::DeleteResourceOutput)
pub mod delete_resource_output {
    /// A builder for [`DeleteResourceOutput`](crate::output::DeleteResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) progress_event: std::option::Option<crate::model::ProgressEvent>,
    }
    impl Builder {
        /// <p>Represents the current status of the resource deletion request.</p>
        /// <p>After you have initiated a resource deletion request, you can monitor the progress of your request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the <code>ProgressEvent</code> returned by <code>DeleteResource</code>.</p>
        pub fn progress_event(mut self, input: crate::model::ProgressEvent) -> Self {
            self.progress_event = Some(input);
            self
        }
        /// <p>Represents the current status of the resource deletion request.</p>
        /// <p>After you have initiated a resource deletion request, you can monitor the progress of your request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the <code>ProgressEvent</code> returned by <code>DeleteResource</code>.</p>
        pub fn set_progress_event(
            mut self,
            input: std::option::Option<crate::model::ProgressEvent>,
        ) -> Self {
            self.progress_event = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteResourceOutput`](crate::output::DeleteResourceOutput)
        pub fn build(self) -> crate::output::DeleteResourceOutput {
            crate::output::DeleteResourceOutput {
                progress_event: self.progress_event,
            }
        }
    }
}
impl DeleteResourceOutput {
    /// Creates a new builder-style object to manufacture [`DeleteResourceOutput`](crate::output::DeleteResourceOutput)
    pub fn builder() -> crate::output::delete_resource_output::Builder {
        crate::output::delete_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateResourceOutput {
    /// <p>Represents the current status of the resource creation request.</p>
    /// <p>After you have initiated a resource creation request, you can monitor the progress of your request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the <code>ProgressEvent</code> returned by <code>CreateResource</code>.</p>
    pub progress_event: std::option::Option<crate::model::ProgressEvent>,
}
impl CreateResourceOutput {
    /// <p>Represents the current status of the resource creation request.</p>
    /// <p>After you have initiated a resource creation request, you can monitor the progress of your request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the <code>ProgressEvent</code> returned by <code>CreateResource</code>.</p>
    pub fn progress_event(&self) -> std::option::Option<&crate::model::ProgressEvent> {
        self.progress_event.as_ref()
    }
}
impl std::fmt::Debug for CreateResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateResourceOutput");
        formatter.field("progress_event", &self.progress_event);
        formatter.finish()
    }
}
/// See [`CreateResourceOutput`](crate::output::CreateResourceOutput)
pub mod create_resource_output {
    /// A builder for [`CreateResourceOutput`](crate::output::CreateResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) progress_event: std::option::Option<crate::model::ProgressEvent>,
    }
    impl Builder {
        /// <p>Represents the current status of the resource creation request.</p>
        /// <p>After you have initiated a resource creation request, you can monitor the progress of your request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the <code>ProgressEvent</code> returned by <code>CreateResource</code>.</p>
        pub fn progress_event(mut self, input: crate::model::ProgressEvent) -> Self {
            self.progress_event = Some(input);
            self
        }
        /// <p>Represents the current status of the resource creation request.</p>
        /// <p>After you have initiated a resource creation request, you can monitor the progress of your request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the <code>ProgressEvent</code> returned by <code>CreateResource</code>.</p>
        pub fn set_progress_event(
            mut self,
            input: std::option::Option<crate::model::ProgressEvent>,
        ) -> Self {
            self.progress_event = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateResourceOutput`](crate::output::CreateResourceOutput)
        pub fn build(self) -> crate::output::CreateResourceOutput {
            crate::output::CreateResourceOutput {
                progress_event: self.progress_event,
            }
        }
    }
}
impl CreateResourceOutput {
    /// Creates a new builder-style object to manufacture [`CreateResourceOutput`](crate::output::CreateResourceOutput)
    pub fn builder() -> crate::output::create_resource_output::Builder {
        crate::output::create_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CancelResourceRequestOutput {
    /// <p>Represents the current status of a resource operation request. For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html">Managing resource operation requests</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub progress_event: std::option::Option<crate::model::ProgressEvent>,
}
impl CancelResourceRequestOutput {
    /// <p>Represents the current status of a resource operation request. For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html">Managing resource operation requests</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn progress_event(&self) -> std::option::Option<&crate::model::ProgressEvent> {
        self.progress_event.as_ref()
    }
}
impl std::fmt::Debug for CancelResourceRequestOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CancelResourceRequestOutput");
        formatter.field("progress_event", &self.progress_event);
        formatter.finish()
    }
}
/// See [`CancelResourceRequestOutput`](crate::output::CancelResourceRequestOutput)
pub mod cancel_resource_request_output {
    /// A builder for [`CancelResourceRequestOutput`](crate::output::CancelResourceRequestOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) progress_event: std::option::Option<crate::model::ProgressEvent>,
    }
    impl Builder {
        /// <p>Represents the current status of a resource operation request. For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html">Managing resource operation requests</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
        pub fn progress_event(mut self, input: crate::model::ProgressEvent) -> Self {
            self.progress_event = Some(input);
            self
        }
        /// <p>Represents the current status of a resource operation request. For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-manage-requests.html">Managing resource operation requests</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
        pub fn set_progress_event(
            mut self,
            input: std::option::Option<crate::model::ProgressEvent>,
        ) -> Self {
            self.progress_event = input;
            self
        }
        /// Consumes the builder and constructs a [`CancelResourceRequestOutput`](crate::output::CancelResourceRequestOutput)
        pub fn build(self) -> crate::output::CancelResourceRequestOutput {
            crate::output::CancelResourceRequestOutput {
                progress_event: self.progress_event,
            }
        }
    }
}
impl CancelResourceRequestOutput {
    /// Creates a new builder-style object to manufacture [`CancelResourceRequestOutput`](crate::output::CancelResourceRequestOutput)
    pub fn builder() -> crate::output::cancel_resource_request_output::Builder {
        crate::output::cancel_resource_request_output::Builder::default()
    }
}
