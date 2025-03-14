// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Contains data about the state of a job execution.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct JobExecutionState {
    /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
    pub status: std::option::Option<crate::model::JobExecutionStatus>,
    /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
    pub status_details:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
    pub version_number: i64,
}
impl JobExecutionState {
    /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
    pub fn status(&self) -> std::option::Option<&crate::model::JobExecutionStatus> {
        self.status.as_ref()
    }
    /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
    pub fn status_details(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.status_details.as_ref()
    }
    /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
    pub fn version_number(&self) -> i64 {
        self.version_number
    }
}
impl std::fmt::Debug for JobExecutionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("JobExecutionState");
        formatter.field("status", &self.status);
        formatter.field("status_details", &self.status_details);
        formatter.field("version_number", &self.version_number);
        formatter.finish()
    }
}
/// See [`JobExecutionState`](crate::model::JobExecutionState)
pub mod job_execution_state {
    /// A builder for [`JobExecutionState`](crate::model::JobExecutionState)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<crate::model::JobExecutionStatus>,
        pub(crate) status_details: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        pub(crate) version_number: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
        pub fn status(mut self, input: crate::model::JobExecutionStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::JobExecutionStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// Adds a key-value pair to `status_details`.
        ///
        /// To override the contents of this collection use [`set_status_details`](Self::set_status_details).
        ///
        /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
        pub fn status_details(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.status_details.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.status_details = Some(hash_map);
            self
        }
        /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
        pub fn set_status_details(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.status_details = input;
            self
        }
        /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
        pub fn version_number(mut self, input: i64) -> Self {
            self.version_number = Some(input);
            self
        }
        /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
        pub fn set_version_number(mut self, input: std::option::Option<i64>) -> Self {
            self.version_number = input;
            self
        }
        /// Consumes the builder and constructs a [`JobExecutionState`](crate::model::JobExecutionState)
        pub fn build(self) -> crate::model::JobExecutionState {
            crate::model::JobExecutionState {
                status: self.status,
                status_details: self.status_details,
                version_number: self.version_number.unwrap_or_default(),
            }
        }
    }
}
impl JobExecutionState {
    /// Creates a new builder-style object to manufacture [`JobExecutionState`](crate::model::JobExecutionState)
    pub fn builder() -> crate::model::job_execution_state::Builder {
        crate::model::job_execution_state::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum JobExecutionStatus {
    #[allow(missing_docs)] // documentation missing in model
    Canceled,
    #[allow(missing_docs)] // documentation missing in model
    Failed,
    #[allow(missing_docs)] // documentation missing in model
    InProgress,
    #[allow(missing_docs)] // documentation missing in model
    Queued,
    #[allow(missing_docs)] // documentation missing in model
    Rejected,
    #[allow(missing_docs)] // documentation missing in model
    Removed,
    #[allow(missing_docs)] // documentation missing in model
    Succeeded,
    #[allow(missing_docs)] // documentation missing in model
    TimedOut,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for JobExecutionStatus {
    fn from(s: &str) -> Self {
        match s {
            "CANCELED" => JobExecutionStatus::Canceled,
            "FAILED" => JobExecutionStatus::Failed,
            "IN_PROGRESS" => JobExecutionStatus::InProgress,
            "QUEUED" => JobExecutionStatus::Queued,
            "REJECTED" => JobExecutionStatus::Rejected,
            "REMOVED" => JobExecutionStatus::Removed,
            "SUCCEEDED" => JobExecutionStatus::Succeeded,
            "TIMED_OUT" => JobExecutionStatus::TimedOut,
            other => JobExecutionStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for JobExecutionStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(JobExecutionStatus::from(s))
    }
}
impl JobExecutionStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            JobExecutionStatus::Canceled => "CANCELED",
            JobExecutionStatus::Failed => "FAILED",
            JobExecutionStatus::InProgress => "IN_PROGRESS",
            JobExecutionStatus::Queued => "QUEUED",
            JobExecutionStatus::Rejected => "REJECTED",
            JobExecutionStatus::Removed => "REMOVED",
            JobExecutionStatus::Succeeded => "SUCCEEDED",
            JobExecutionStatus::TimedOut => "TIMED_OUT",
            JobExecutionStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "CANCELED",
            "FAILED",
            "IN_PROGRESS",
            "QUEUED",
            "REJECTED",
            "REMOVED",
            "SUCCEEDED",
            "TIMED_OUT",
        ]
    }
}
impl AsRef<str> for JobExecutionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Contains data about a job execution.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct JobExecution {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    pub job_id: std::option::Option<std::string::String>,
    /// <p>The name of the thing that is executing the job.</p>
    pub thing_name: std::option::Option<std::string::String>,
    /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
    pub status: std::option::Option<crate::model::JobExecutionStatus>,
    /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
    pub status_details:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
    pub queued_at: i64,
    /// <p>The time, in milliseconds since the epoch, when the job execution was started.</p>
    pub started_at: std::option::Option<i64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was last updated. </p>
    pub last_updated_at: i64,
    /// <p>The estimated number of seconds that remain before the job execution status will be changed to <code>TIMED_OUT</code>.</p>
    pub approximate_seconds_before_timed_out: std::option::Option<i64>,
    /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
    pub version_number: i64,
    /// <p>A number that identifies a particular job execution on a particular device. It can be used later in commands that return or update job execution information.</p>
    pub execution_number: std::option::Option<i64>,
    /// <p>The content of the job document.</p>
    pub job_document: std::option::Option<std::string::String>,
}
impl JobExecution {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    pub fn job_id(&self) -> std::option::Option<&str> {
        self.job_id.as_deref()
    }
    /// <p>The name of the thing that is executing the job.</p>
    pub fn thing_name(&self) -> std::option::Option<&str> {
        self.thing_name.as_deref()
    }
    /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
    pub fn status(&self) -> std::option::Option<&crate::model::JobExecutionStatus> {
        self.status.as_ref()
    }
    /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
    pub fn status_details(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.status_details.as_ref()
    }
    /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
    pub fn queued_at(&self) -> i64 {
        self.queued_at
    }
    /// <p>The time, in milliseconds since the epoch, when the job execution was started.</p>
    pub fn started_at(&self) -> std::option::Option<i64> {
        self.started_at
    }
    /// <p>The time, in milliseconds since the epoch, when the job execution was last updated. </p>
    pub fn last_updated_at(&self) -> i64 {
        self.last_updated_at
    }
    /// <p>The estimated number of seconds that remain before the job execution status will be changed to <code>TIMED_OUT</code>.</p>
    pub fn approximate_seconds_before_timed_out(&self) -> std::option::Option<i64> {
        self.approximate_seconds_before_timed_out
    }
    /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
    pub fn version_number(&self) -> i64 {
        self.version_number
    }
    /// <p>A number that identifies a particular job execution on a particular device. It can be used later in commands that return or update job execution information.</p>
    pub fn execution_number(&self) -> std::option::Option<i64> {
        self.execution_number
    }
    /// <p>The content of the job document.</p>
    pub fn job_document(&self) -> std::option::Option<&str> {
        self.job_document.as_deref()
    }
}
impl std::fmt::Debug for JobExecution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("JobExecution");
        formatter.field("job_id", &self.job_id);
        formatter.field("thing_name", &self.thing_name);
        formatter.field("status", &self.status);
        formatter.field("status_details", &self.status_details);
        formatter.field("queued_at", &self.queued_at);
        formatter.field("started_at", &self.started_at);
        formatter.field("last_updated_at", &self.last_updated_at);
        formatter.field(
            "approximate_seconds_before_timed_out",
            &self.approximate_seconds_before_timed_out,
        );
        formatter.field("version_number", &self.version_number);
        formatter.field("execution_number", &self.execution_number);
        formatter.field("job_document", &self.job_document);
        formatter.finish()
    }
}
/// See [`JobExecution`](crate::model::JobExecution)
pub mod job_execution {
    /// A builder for [`JobExecution`](crate::model::JobExecution)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) job_id: std::option::Option<std::string::String>,
        pub(crate) thing_name: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::JobExecutionStatus>,
        pub(crate) status_details: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        pub(crate) queued_at: std::option::Option<i64>,
        pub(crate) started_at: std::option::Option<i64>,
        pub(crate) last_updated_at: std::option::Option<i64>,
        pub(crate) approximate_seconds_before_timed_out: std::option::Option<i64>,
        pub(crate) version_number: std::option::Option<i64>,
        pub(crate) execution_number: std::option::Option<i64>,
        pub(crate) job_document: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The unique identifier you assigned to this job when it was created.</p>
        pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.job_id = Some(input.into());
            self
        }
        /// <p>The unique identifier you assigned to this job when it was created.</p>
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.job_id = input;
            self
        }
        /// <p>The name of the thing that is executing the job.</p>
        pub fn thing_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.thing_name = Some(input.into());
            self
        }
        /// <p>The name of the thing that is executing the job.</p>
        pub fn set_thing_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.thing_name = input;
            self
        }
        /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
        pub fn status(mut self, input: crate::model::JobExecutionStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of the job execution. Can be one of: "QUEUED", "IN_PROGRESS", "FAILED", "SUCCESS", "CANCELED", "REJECTED", or "REMOVED".</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::JobExecutionStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// Adds a key-value pair to `status_details`.
        ///
        /// To override the contents of this collection use [`set_status_details`](Self::set_status_details).
        ///
        /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
        pub fn status_details(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.status_details.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.status_details = Some(hash_map);
            self
        }
        /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
        pub fn set_status_details(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.status_details = input;
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
        pub fn queued_at(mut self, input: i64) -> Self {
            self.queued_at = Some(input);
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
        pub fn set_queued_at(mut self, input: std::option::Option<i64>) -> Self {
            self.queued_at = input;
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was started.</p>
        pub fn started_at(mut self, input: i64) -> Self {
            self.started_at = Some(input);
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was started.</p>
        pub fn set_started_at(mut self, input: std::option::Option<i64>) -> Self {
            self.started_at = input;
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was last updated. </p>
        pub fn last_updated_at(mut self, input: i64) -> Self {
            self.last_updated_at = Some(input);
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was last updated. </p>
        pub fn set_last_updated_at(mut self, input: std::option::Option<i64>) -> Self {
            self.last_updated_at = input;
            self
        }
        /// <p>The estimated number of seconds that remain before the job execution status will be changed to <code>TIMED_OUT</code>.</p>
        pub fn approximate_seconds_before_timed_out(mut self, input: i64) -> Self {
            self.approximate_seconds_before_timed_out = Some(input);
            self
        }
        /// <p>The estimated number of seconds that remain before the job execution status will be changed to <code>TIMED_OUT</code>.</p>
        pub fn set_approximate_seconds_before_timed_out(
            mut self,
            input: std::option::Option<i64>,
        ) -> Self {
            self.approximate_seconds_before_timed_out = input;
            self
        }
        /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
        pub fn version_number(mut self, input: i64) -> Self {
            self.version_number = Some(input);
            self
        }
        /// <p>The version of the job execution. Job execution versions are incremented each time they are updated by a device.</p>
        pub fn set_version_number(mut self, input: std::option::Option<i64>) -> Self {
            self.version_number = input;
            self
        }
        /// <p>A number that identifies a particular job execution on a particular device. It can be used later in commands that return or update job execution information.</p>
        pub fn execution_number(mut self, input: i64) -> Self {
            self.execution_number = Some(input);
            self
        }
        /// <p>A number that identifies a particular job execution on a particular device. It can be used later in commands that return or update job execution information.</p>
        pub fn set_execution_number(mut self, input: std::option::Option<i64>) -> Self {
            self.execution_number = input;
            self
        }
        /// <p>The content of the job document.</p>
        pub fn job_document(mut self, input: impl Into<std::string::String>) -> Self {
            self.job_document = Some(input.into());
            self
        }
        /// <p>The content of the job document.</p>
        pub fn set_job_document(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.job_document = input;
            self
        }
        /// Consumes the builder and constructs a [`JobExecution`](crate::model::JobExecution)
        pub fn build(self) -> crate::model::JobExecution {
            crate::model::JobExecution {
                job_id: self.job_id,
                thing_name: self.thing_name,
                status: self.status,
                status_details: self.status_details,
                queued_at: self.queued_at.unwrap_or_default(),
                started_at: self.started_at,
                last_updated_at: self.last_updated_at.unwrap_or_default(),
                approximate_seconds_before_timed_out: self.approximate_seconds_before_timed_out,
                version_number: self.version_number.unwrap_or_default(),
                execution_number: self.execution_number,
                job_document: self.job_document,
            }
        }
    }
}
impl JobExecution {
    /// Creates a new builder-style object to manufacture [`JobExecution`](crate::model::JobExecution)
    pub fn builder() -> crate::model::job_execution::Builder {
        crate::model::job_execution::Builder::default()
    }
}

/// <p>Contains a subset of information about a job execution.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct JobExecutionSummary {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    pub job_id: std::option::Option<std::string::String>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
    pub queued_at: i64,
    /// <p>The time, in milliseconds since the epoch, when the job execution started.</p>
    pub started_at: std::option::Option<i64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was last updated.</p>
    pub last_updated_at: i64,
    /// <p>The version of the job execution. Job execution versions are incremented each time AWS IoT Jobs receives an update from a device.</p>
    pub version_number: i64,
    /// <p>A number that identifies a particular job execution on a particular device.</p>
    pub execution_number: std::option::Option<i64>,
}
impl JobExecutionSummary {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    pub fn job_id(&self) -> std::option::Option<&str> {
        self.job_id.as_deref()
    }
    /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
    pub fn queued_at(&self) -> i64 {
        self.queued_at
    }
    /// <p>The time, in milliseconds since the epoch, when the job execution started.</p>
    pub fn started_at(&self) -> std::option::Option<i64> {
        self.started_at
    }
    /// <p>The time, in milliseconds since the epoch, when the job execution was last updated.</p>
    pub fn last_updated_at(&self) -> i64 {
        self.last_updated_at
    }
    /// <p>The version of the job execution. Job execution versions are incremented each time AWS IoT Jobs receives an update from a device.</p>
    pub fn version_number(&self) -> i64 {
        self.version_number
    }
    /// <p>A number that identifies a particular job execution on a particular device.</p>
    pub fn execution_number(&self) -> std::option::Option<i64> {
        self.execution_number
    }
}
impl std::fmt::Debug for JobExecutionSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("JobExecutionSummary");
        formatter.field("job_id", &self.job_id);
        formatter.field("queued_at", &self.queued_at);
        formatter.field("started_at", &self.started_at);
        formatter.field("last_updated_at", &self.last_updated_at);
        formatter.field("version_number", &self.version_number);
        formatter.field("execution_number", &self.execution_number);
        formatter.finish()
    }
}
/// See [`JobExecutionSummary`](crate::model::JobExecutionSummary)
pub mod job_execution_summary {
    /// A builder for [`JobExecutionSummary`](crate::model::JobExecutionSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) job_id: std::option::Option<std::string::String>,
        pub(crate) queued_at: std::option::Option<i64>,
        pub(crate) started_at: std::option::Option<i64>,
        pub(crate) last_updated_at: std::option::Option<i64>,
        pub(crate) version_number: std::option::Option<i64>,
        pub(crate) execution_number: std::option::Option<i64>,
    }
    impl Builder {
        /// <p>The unique identifier you assigned to this job when it was created.</p>
        pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.job_id = Some(input.into());
            self
        }
        /// <p>The unique identifier you assigned to this job when it was created.</p>
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.job_id = input;
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
        pub fn queued_at(mut self, input: i64) -> Self {
            self.queued_at = Some(input);
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was enqueued.</p>
        pub fn set_queued_at(mut self, input: std::option::Option<i64>) -> Self {
            self.queued_at = input;
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution started.</p>
        pub fn started_at(mut self, input: i64) -> Self {
            self.started_at = Some(input);
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution started.</p>
        pub fn set_started_at(mut self, input: std::option::Option<i64>) -> Self {
            self.started_at = input;
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was last updated.</p>
        pub fn last_updated_at(mut self, input: i64) -> Self {
            self.last_updated_at = Some(input);
            self
        }
        /// <p>The time, in milliseconds since the epoch, when the job execution was last updated.</p>
        pub fn set_last_updated_at(mut self, input: std::option::Option<i64>) -> Self {
            self.last_updated_at = input;
            self
        }
        /// <p>The version of the job execution. Job execution versions are incremented each time AWS IoT Jobs receives an update from a device.</p>
        pub fn version_number(mut self, input: i64) -> Self {
            self.version_number = Some(input);
            self
        }
        /// <p>The version of the job execution. Job execution versions are incremented each time AWS IoT Jobs receives an update from a device.</p>
        pub fn set_version_number(mut self, input: std::option::Option<i64>) -> Self {
            self.version_number = input;
            self
        }
        /// <p>A number that identifies a particular job execution on a particular device.</p>
        pub fn execution_number(mut self, input: i64) -> Self {
            self.execution_number = Some(input);
            self
        }
        /// <p>A number that identifies a particular job execution on a particular device.</p>
        pub fn set_execution_number(mut self, input: std::option::Option<i64>) -> Self {
            self.execution_number = input;
            self
        }
        /// Consumes the builder and constructs a [`JobExecutionSummary`](crate::model::JobExecutionSummary)
        pub fn build(self) -> crate::model::JobExecutionSummary {
            crate::model::JobExecutionSummary {
                job_id: self.job_id,
                queued_at: self.queued_at.unwrap_or_default(),
                started_at: self.started_at,
                last_updated_at: self.last_updated_at.unwrap_or_default(),
                version_number: self.version_number.unwrap_or_default(),
                execution_number: self.execution_number,
            }
        }
    }
}
impl JobExecutionSummary {
    /// Creates a new builder-style object to manufacture [`JobExecutionSummary`](crate::model::JobExecutionSummary)
    pub fn builder() -> crate::model::job_execution_summary::Builder {
        crate::model::job_execution_summary::Builder::default()
    }
}
