// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for AWS Cost and Usage Report Service
///
/// Client for invoking operations on AWS Cost and Usage Report Service. Each operation on AWS Cost and Usage Report Service is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_costandusagereport::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
/// let shared_config = aws_config::load_from_env().await;
/// let config = aws_sdk_costandusagereport::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_costandusagereport::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client {
    handle: std::sync::Arc<Handle>,
}

impl std::clone::Clone for Client {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl
    From<
        aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    > for Client
{
    fn from(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    ) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl Client {
    /// Creates a client with the given service configuration.
    pub fn with_config(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
        conf: crate::Config,
    ) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    /// Constructs a fluent builder for the [`DeleteReportDefinition`](crate::client::fluent_builders::DeleteReportDefinition) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`report_name(impl Into<String>)`](crate::client::fluent_builders::DeleteReportDefinition::report_name) / [`set_report_name(Option<String>)`](crate::client::fluent_builders::DeleteReportDefinition::set_report_name): <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
    /// - On success, responds with [`DeleteReportDefinitionOutput`](crate::output::DeleteReportDefinitionOutput) with field(s):
    ///   - [`response_message(Option<String>)`](crate::output::DeleteReportDefinitionOutput::response_message): <p>Whether the deletion was successful or not.</p>
    /// - On failure, responds with [`SdkError<DeleteReportDefinitionError>`](crate::error::DeleteReportDefinitionError)
    pub fn delete_report_definition(&self) -> fluent_builders::DeleteReportDefinition {
        fluent_builders::DeleteReportDefinition::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`DescribeReportDefinitions`](crate::client::fluent_builders::DescribeReportDefinitions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeReportDefinitions::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeReportDefinitions::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeReportDefinitions::set_max_results): <p>The maximum number of results that AWS returns for the operation.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeReportDefinitions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeReportDefinitions::set_next_token): <p>A generic string.</p>
    /// - On success, responds with [`DescribeReportDefinitionsOutput`](crate::output::DescribeReportDefinitionsOutput) with field(s):
    ///   - [`report_definitions(Option<Vec<ReportDefinition>>)`](crate::output::DescribeReportDefinitionsOutput::report_definitions): <p>A list of AWS Cost and Usage reports owned by the account.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeReportDefinitionsOutput::next_token): <p>A generic string.</p>
    /// - On failure, responds with [`SdkError<DescribeReportDefinitionsError>`](crate::error::DescribeReportDefinitionsError)
    pub fn describe_report_definitions(&self) -> fluent_builders::DescribeReportDefinitions {
        fluent_builders::DescribeReportDefinitions::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ModifyReportDefinition`](crate::client::fluent_builders::ModifyReportDefinition) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`report_name(impl Into<String>)`](crate::client::fluent_builders::ModifyReportDefinition::report_name) / [`set_report_name(Option<String>)`](crate::client::fluent_builders::ModifyReportDefinition::set_report_name): <p>The name of the report that you want to create. The name must be unique, is case sensitive, and can't include spaces. </p>
    ///   - [`report_definition(ReportDefinition)`](crate::client::fluent_builders::ModifyReportDefinition::report_definition) / [`set_report_definition(Option<ReportDefinition>)`](crate::client::fluent_builders::ModifyReportDefinition::set_report_definition): <p>The definition of AWS Cost and Usage Report. You can specify the report name, time unit, report format, compression format, S3 bucket, additional artifacts, and schema elements in the definition. </p>
    /// - On success, responds with [`ModifyReportDefinitionOutput`](crate::output::ModifyReportDefinitionOutput)

    /// - On failure, responds with [`SdkError<ModifyReportDefinitionError>`](crate::error::ModifyReportDefinitionError)
    pub fn modify_report_definition(&self) -> fluent_builders::ModifyReportDefinition {
        fluent_builders::ModifyReportDefinition::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`PutReportDefinition`](crate::client::fluent_builders::PutReportDefinition) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`report_definition(ReportDefinition)`](crate::client::fluent_builders::PutReportDefinition::report_definition) / [`set_report_definition(Option<ReportDefinition>)`](crate::client::fluent_builders::PutReportDefinition::set_report_definition): <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed metadata and data file information. </p>
    /// - On success, responds with [`PutReportDefinitionOutput`](crate::output::PutReportDefinitionOutput)

    /// - On failure, responds with [`SdkError<PutReportDefinitionError>`](crate::error::PutReportDefinitionError)
    pub fn put_report_definition(&self) -> fluent_builders::PutReportDefinition {
        fluent_builders::PutReportDefinition::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `DeleteReportDefinition`.
    ///
    /// <p>Deletes the specified report.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DeleteReportDefinition {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::delete_report_definition_input::Builder,
    }
    impl DeleteReportDefinition {
        /// Creates a new `DeleteReportDefinition`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteReportDefinitionOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
        pub fn report_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_name(input.into());
            self
        }
        /// <p>The name of the report that you want to delete. The name must be unique, is case sensitive, and can't include spaces.</p>
        pub fn set_report_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DescribeReportDefinitions`.
    ///
    /// <p>Lists the AWS Cost and Usage reports available to this account.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeReportDefinitions {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::describe_report_definitions_input::Builder,
    }
    impl DescribeReportDefinitions {
        /// Creates a new `DescribeReportDefinitions`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeReportDefinitionsOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeReportDefinitionsError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::DescribeReportDefinitionsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::DescribeReportDefinitionsPaginator {
            crate::paginator::DescribeReportDefinitionsPaginator::new(self.handle, self.inner)
        }
        /// <p>The maximum number of results that AWS returns for the operation.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of results that AWS returns for the operation.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>A generic string.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>A generic string.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ModifyReportDefinition`.
    ///
    /// <p>Allows you to programatically update your report preferences.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ModifyReportDefinition {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::modify_report_definition_input::Builder,
    }
    impl ModifyReportDefinition {
        /// Creates a new `ModifyReportDefinition`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ModifyReportDefinitionOutput,
            aws_smithy_http::result::SdkError<crate::error::ModifyReportDefinitionError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The name of the report that you want to create. The name must be unique, is case sensitive, and can't include spaces. </p>
        pub fn report_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_name(input.into());
            self
        }
        /// <p>The name of the report that you want to create. The name must be unique, is case sensitive, and can't include spaces. </p>
        pub fn set_report_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_name(input);
            self
        }
        /// <p>The definition of AWS Cost and Usage Report. You can specify the report name, time unit, report format, compression format, S3 bucket, additional artifacts, and schema elements in the definition. </p>
        pub fn report_definition(mut self, input: crate::model::ReportDefinition) -> Self {
            self.inner = self.inner.report_definition(input);
            self
        }
        /// <p>The definition of AWS Cost and Usage Report. You can specify the report name, time unit, report format, compression format, S3 bucket, additional artifacts, and schema elements in the definition. </p>
        pub fn set_report_definition(
            mut self,
            input: std::option::Option<crate::model::ReportDefinition>,
        ) -> Self {
            self.inner = self.inner.set_report_definition(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutReportDefinition`.
    ///
    /// <p>Creates a new report using the description that you provide.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutReportDefinition {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::put_report_definition_input::Builder,
    }
    impl PutReportDefinition {
        /// Creates a new `PutReportDefinition`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::PutReportDefinitionOutput,
            aws_smithy_http::result::SdkError<crate::error::PutReportDefinitionError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed metadata and data file information. </p>
        pub fn report_definition(mut self, input: crate::model::ReportDefinition) -> Self {
            self.inner = self.inner.report_definition(input);
            self
        }
        /// <p>Represents the output of the PutReportDefinition operation. The content consists of the detailed metadata and data file information. </p>
        pub fn set_report_definition(
            mut self,
            input: std::option::Option<crate::model::ReportDefinition>,
        ) -> Self {
            self.inner = self.inner.set_report_definition(input);
            self
        }
    }
}

impl Client {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn<C, E>(conf: crate::Config, conn: C) -> Self
    where
        C: aws_smithy_client::bounds::SmithyConnector<Error = E> + Send + 'static,
        E: Into<aws_smithy_http::result::ConnectorError>,
    {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(aws_smithy_client::erase::DynConnector::new(conn))
            .middleware(aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ));
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https().middleware(
            aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ),
        );
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
