// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StopHumanLoopOutput {}
impl std::fmt::Debug for StopHumanLoopOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StopHumanLoopOutput");
        formatter.finish()
    }
}
/// See [`StopHumanLoopOutput`](crate::output::StopHumanLoopOutput)
pub mod stop_human_loop_output {
    /// A builder for [`StopHumanLoopOutput`](crate::output::StopHumanLoopOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`StopHumanLoopOutput`](crate::output::StopHumanLoopOutput)
        pub fn build(self) -> crate::output::StopHumanLoopOutput {
            crate::output::StopHumanLoopOutput {}
        }
    }
}
impl StopHumanLoopOutput {
    /// Creates a new builder-style object to manufacture [`StopHumanLoopOutput`](crate::output::StopHumanLoopOutput)
    pub fn builder() -> crate::output::stop_human_loop_output::Builder {
        crate::output::stop_human_loop_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartHumanLoopOutput {
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    pub human_loop_arn: std::option::Option<std::string::String>,
}
impl StartHumanLoopOutput {
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    pub fn human_loop_arn(&self) -> std::option::Option<&str> {
        self.human_loop_arn.as_deref()
    }
}
impl std::fmt::Debug for StartHumanLoopOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartHumanLoopOutput");
        formatter.field("human_loop_arn", &self.human_loop_arn);
        formatter.finish()
    }
}
/// See [`StartHumanLoopOutput`](crate::output::StartHumanLoopOutput)
pub mod start_human_loop_output {
    /// A builder for [`StartHumanLoopOutput`](crate::output::StartHumanLoopOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) human_loop_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
        pub fn human_loop_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.human_loop_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
        pub fn set_human_loop_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.human_loop_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`StartHumanLoopOutput`](crate::output::StartHumanLoopOutput)
        pub fn build(self) -> crate::output::StartHumanLoopOutput {
            crate::output::StartHumanLoopOutput {
                human_loop_arn: self.human_loop_arn,
            }
        }
    }
}
impl StartHumanLoopOutput {
    /// Creates a new builder-style object to manufacture [`StartHumanLoopOutput`](crate::output::StartHumanLoopOutput)
    pub fn builder() -> crate::output::start_human_loop_output::Builder {
        crate::output::start_human_loop_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListHumanLoopsOutput {
    /// <p>An array of objects that contain information about the human loops.</p>
    pub human_loop_summaries: std::option::Option<std::vec::Vec<crate::model::HumanLoopSummary>>,
    /// <p>A token to display the next page of results.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListHumanLoopsOutput {
    /// <p>An array of objects that contain information about the human loops.</p>
    pub fn human_loop_summaries(&self) -> std::option::Option<&[crate::model::HumanLoopSummary]> {
        self.human_loop_summaries.as_deref()
    }
    /// <p>A token to display the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListHumanLoopsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListHumanLoopsOutput");
        formatter.field("human_loop_summaries", &self.human_loop_summaries);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListHumanLoopsOutput`](crate::output::ListHumanLoopsOutput)
pub mod list_human_loops_output {
    /// A builder for [`ListHumanLoopsOutput`](crate::output::ListHumanLoopsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) human_loop_summaries:
            std::option::Option<std::vec::Vec<crate::model::HumanLoopSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `human_loop_summaries`.
        ///
        /// To override the contents of this collection use [`set_human_loop_summaries`](Self::set_human_loop_summaries).
        ///
        /// <p>An array of objects that contain information about the human loops.</p>
        pub fn human_loop_summaries(mut self, input: crate::model::HumanLoopSummary) -> Self {
            let mut v = self.human_loop_summaries.unwrap_or_default();
            v.push(input);
            self.human_loop_summaries = Some(v);
            self
        }
        /// <p>An array of objects that contain information about the human loops.</p>
        pub fn set_human_loop_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::HumanLoopSummary>>,
        ) -> Self {
            self.human_loop_summaries = input;
            self
        }
        /// <p>A token to display the next page of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token to display the next page of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListHumanLoopsOutput`](crate::output::ListHumanLoopsOutput)
        pub fn build(self) -> crate::output::ListHumanLoopsOutput {
            crate::output::ListHumanLoopsOutput {
                human_loop_summaries: self.human_loop_summaries,
                next_token: self.next_token,
            }
        }
    }
}
impl ListHumanLoopsOutput {
    /// Creates a new builder-style object to manufacture [`ListHumanLoopsOutput`](crate::output::ListHumanLoopsOutput)
    pub fn builder() -> crate::output::list_human_loops_output::Builder {
        crate::output::list_human_loops_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeHumanLoopOutput {
    /// <p>The creation time when Amazon Augmented AI created the human loop.</p>
    pub creation_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The reason why a human loop failed. The failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
    pub failure_reason: std::option::Option<std::string::String>,
    /// <p>A failure code that identifies the type of failure.</p>
    /// <p>Possible values: <code>ValidationError</code>, <code>Expired</code>, <code>InternalError</code> </p>
    pub failure_code: std::option::Option<std::string::String>,
    /// <p>The status of the human loop. </p>
    pub human_loop_status: std::option::Option<crate::model::HumanLoopStatus>,
    /// <p>The name of the human loop. The name must be lowercase, unique within the Region in your account, and can have up to 63 characters. Valid characters: a-z, 0-9, and - (hyphen).</p>
    pub human_loop_name: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    pub human_loop_arn: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    pub flow_definition_arn: std::option::Option<std::string::String>,
    /// <p>An object that contains information about the output of the human loop.</p>
    pub human_loop_output: std::option::Option<crate::model::HumanLoopOutput>,
}
impl DescribeHumanLoopOutput {
    /// <p>The creation time when Amazon Augmented AI created the human loop.</p>
    pub fn creation_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The reason why a human loop failed. The failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
    pub fn failure_reason(&self) -> std::option::Option<&str> {
        self.failure_reason.as_deref()
    }
    /// <p>A failure code that identifies the type of failure.</p>
    /// <p>Possible values: <code>ValidationError</code>, <code>Expired</code>, <code>InternalError</code> </p>
    pub fn failure_code(&self) -> std::option::Option<&str> {
        self.failure_code.as_deref()
    }
    /// <p>The status of the human loop. </p>
    pub fn human_loop_status(&self) -> std::option::Option<&crate::model::HumanLoopStatus> {
        self.human_loop_status.as_ref()
    }
    /// <p>The name of the human loop. The name must be lowercase, unique within the Region in your account, and can have up to 63 characters. Valid characters: a-z, 0-9, and - (hyphen).</p>
    pub fn human_loop_name(&self) -> std::option::Option<&str> {
        self.human_loop_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
    pub fn human_loop_arn(&self) -> std::option::Option<&str> {
        self.human_loop_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    pub fn flow_definition_arn(&self) -> std::option::Option<&str> {
        self.flow_definition_arn.as_deref()
    }
    /// <p>An object that contains information about the output of the human loop.</p>
    pub fn human_loop_output(&self) -> std::option::Option<&crate::model::HumanLoopOutput> {
        self.human_loop_output.as_ref()
    }
}
impl std::fmt::Debug for DescribeHumanLoopOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeHumanLoopOutput");
        formatter.field("creation_time", &self.creation_time);
        formatter.field("failure_reason", &self.failure_reason);
        formatter.field("failure_code", &self.failure_code);
        formatter.field("human_loop_status", &self.human_loop_status);
        formatter.field("human_loop_name", &self.human_loop_name);
        formatter.field("human_loop_arn", &self.human_loop_arn);
        formatter.field("flow_definition_arn", &self.flow_definition_arn);
        formatter.field("human_loop_output", &self.human_loop_output);
        formatter.finish()
    }
}
/// See [`DescribeHumanLoopOutput`](crate::output::DescribeHumanLoopOutput)
pub mod describe_human_loop_output {
    /// A builder for [`DescribeHumanLoopOutput`](crate::output::DescribeHumanLoopOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) creation_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) failure_reason: std::option::Option<std::string::String>,
        pub(crate) failure_code: std::option::Option<std::string::String>,
        pub(crate) human_loop_status: std::option::Option<crate::model::HumanLoopStatus>,
        pub(crate) human_loop_name: std::option::Option<std::string::String>,
        pub(crate) human_loop_arn: std::option::Option<std::string::String>,
        pub(crate) flow_definition_arn: std::option::Option<std::string::String>,
        pub(crate) human_loop_output: std::option::Option<crate::model::HumanLoopOutput>,
    }
    impl Builder {
        /// <p>The creation time when Amazon Augmented AI created the human loop.</p>
        pub fn creation_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.creation_time = Some(input);
            self
        }
        /// <p>The creation time when Amazon Augmented AI created the human loop.</p>
        pub fn set_creation_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.creation_time = input;
            self
        }
        /// <p>The reason why a human loop failed. The failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
        pub fn failure_reason(mut self, input: impl Into<std::string::String>) -> Self {
            self.failure_reason = Some(input.into());
            self
        }
        /// <p>The reason why a human loop failed. The failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
        pub fn set_failure_reason(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.failure_reason = input;
            self
        }
        /// <p>A failure code that identifies the type of failure.</p>
        /// <p>Possible values: <code>ValidationError</code>, <code>Expired</code>, <code>InternalError</code> </p>
        pub fn failure_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.failure_code = Some(input.into());
            self
        }
        /// <p>A failure code that identifies the type of failure.</p>
        /// <p>Possible values: <code>ValidationError</code>, <code>Expired</code>, <code>InternalError</code> </p>
        pub fn set_failure_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.failure_code = input;
            self
        }
        /// <p>The status of the human loop. </p>
        pub fn human_loop_status(mut self, input: crate::model::HumanLoopStatus) -> Self {
            self.human_loop_status = Some(input);
            self
        }
        /// <p>The status of the human loop. </p>
        pub fn set_human_loop_status(
            mut self,
            input: std::option::Option<crate::model::HumanLoopStatus>,
        ) -> Self {
            self.human_loop_status = input;
            self
        }
        /// <p>The name of the human loop. The name must be lowercase, unique within the Region in your account, and can have up to 63 characters. Valid characters: a-z, 0-9, and - (hyphen).</p>
        pub fn human_loop_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.human_loop_name = Some(input.into());
            self
        }
        /// <p>The name of the human loop. The name must be lowercase, unique within the Region in your account, and can have up to 63 characters. Valid characters: a-z, 0-9, and - (hyphen).</p>
        pub fn set_human_loop_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.human_loop_name = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
        pub fn human_loop_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.human_loop_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the human loop.</p>
        pub fn set_human_loop_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.human_loop_arn = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
        pub fn flow_definition_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.flow_definition_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the flow definition.</p>
        pub fn set_flow_definition_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.flow_definition_arn = input;
            self
        }
        /// <p>An object that contains information about the output of the human loop.</p>
        pub fn human_loop_output(mut self, input: crate::model::HumanLoopOutput) -> Self {
            self.human_loop_output = Some(input);
            self
        }
        /// <p>An object that contains information about the output of the human loop.</p>
        pub fn set_human_loop_output(
            mut self,
            input: std::option::Option<crate::model::HumanLoopOutput>,
        ) -> Self {
            self.human_loop_output = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeHumanLoopOutput`](crate::output::DescribeHumanLoopOutput)
        pub fn build(self) -> crate::output::DescribeHumanLoopOutput {
            crate::output::DescribeHumanLoopOutput {
                creation_time: self.creation_time,
                failure_reason: self.failure_reason,
                failure_code: self.failure_code,
                human_loop_status: self.human_loop_status,
                human_loop_name: self.human_loop_name,
                human_loop_arn: self.human_loop_arn,
                flow_definition_arn: self.flow_definition_arn,
                human_loop_output: self.human_loop_output,
            }
        }
    }
}
impl DescribeHumanLoopOutput {
    /// Creates a new builder-style object to manufacture [`DescribeHumanLoopOutput`](crate::output::DescribeHumanLoopOutput)
    pub fn builder() -> crate::output::describe_human_loop_output::Builder {
        crate::output::describe_human_loop_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteHumanLoopOutput {}
impl std::fmt::Debug for DeleteHumanLoopOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteHumanLoopOutput");
        formatter.finish()
    }
}
/// See [`DeleteHumanLoopOutput`](crate::output::DeleteHumanLoopOutput)
pub mod delete_human_loop_output {
    /// A builder for [`DeleteHumanLoopOutput`](crate::output::DeleteHumanLoopOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteHumanLoopOutput`](crate::output::DeleteHumanLoopOutput)
        pub fn build(self) -> crate::output::DeleteHumanLoopOutput {
            crate::output::DeleteHumanLoopOutput {}
        }
    }
}
impl DeleteHumanLoopOutput {
    /// Creates a new builder-style object to manufacture [`DeleteHumanLoopOutput`](crate::output::DeleteHumanLoopOutput)
    pub fn builder() -> crate::output::delete_human_loop_output::Builder {
        crate::output::delete_human_loop_output::Builder::default()
    }
}
