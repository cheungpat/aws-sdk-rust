// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
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
pub enum ValidationExceptionReason {
    #[allow(missing_docs)] // documentation missing in model
    InvalidPageToken,
    #[allow(missing_docs)] // documentation missing in model
    InvalidParameterValue,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ValidationExceptionReason {
    fn from(s: &str) -> Self {
        match s {
            "INVALID_PAGE_TOKEN" => ValidationExceptionReason::InvalidPageToken,
            "INVALID_PARAMETER_VALUE" => ValidationExceptionReason::InvalidParameterValue,
            other => ValidationExceptionReason::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ValidationExceptionReason {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ValidationExceptionReason::from(s))
    }
}
impl ValidationExceptionReason {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ValidationExceptionReason::InvalidPageToken => "INVALID_PAGE_TOKEN",
            ValidationExceptionReason::InvalidParameterValue => "INVALID_PARAMETER_VALUE",
            ValidationExceptionReason::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["INVALID_PAGE_TOKEN", "INVALID_PARAMETER_VALUE"]
    }
}
impl AsRef<str> for ValidationExceptionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
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
pub enum ResourceNotFoundExceptionReason {
    #[allow(missing_docs)] // documentation missing in model
    RuleNotFound,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ResourceNotFoundExceptionReason {
    fn from(s: &str) -> Self {
        match s {
            "RULE_NOT_FOUND" => ResourceNotFoundExceptionReason::RuleNotFound,
            other => ResourceNotFoundExceptionReason::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ResourceNotFoundExceptionReason {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ResourceNotFoundExceptionReason::from(s))
    }
}
impl ResourceNotFoundExceptionReason {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ResourceNotFoundExceptionReason::RuleNotFound => "RULE_NOT_FOUND",
            ResourceNotFoundExceptionReason::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["RULE_NOT_FOUND"]
    }
}
impl AsRef<str> for ResourceNotFoundExceptionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
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
pub enum RuleStatus {
    #[allow(missing_docs)] // documentation missing in model
    Available,
    #[allow(missing_docs)] // documentation missing in model
    Pending,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for RuleStatus {
    fn from(s: &str) -> Self {
        match s {
            "available" => RuleStatus::Available,
            "pending" => RuleStatus::Pending,
            other => RuleStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for RuleStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(RuleStatus::from(s))
    }
}
impl RuleStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            RuleStatus::Available => "available",
            RuleStatus::Pending => "pending",
            RuleStatus::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["available", "pending"]
    }
}
impl AsRef<str> for RuleStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about the resource tags used to identify resources that are retained by the retention rule.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceTag {
    /// <p>The tag key.</p>
    pub resource_tag_key: std::option::Option<std::string::String>,
    /// <p>The tag value.</p>
    pub resource_tag_value: std::option::Option<std::string::String>,
}
impl ResourceTag {
    /// <p>The tag key.</p>
    pub fn resource_tag_key(&self) -> std::option::Option<&str> {
        self.resource_tag_key.as_deref()
    }
    /// <p>The tag value.</p>
    pub fn resource_tag_value(&self) -> std::option::Option<&str> {
        self.resource_tag_value.as_deref()
    }
}
impl std::fmt::Debug for ResourceTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceTag");
        formatter.field("resource_tag_key", &self.resource_tag_key);
        formatter.field("resource_tag_value", &self.resource_tag_value);
        formatter.finish()
    }
}
/// See [`ResourceTag`](crate::model::ResourceTag)
pub mod resource_tag {
    /// A builder for [`ResourceTag`](crate::model::ResourceTag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) resource_tag_key: std::option::Option<std::string::String>,
        pub(crate) resource_tag_value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The tag key.</p>
        pub fn resource_tag_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.resource_tag_key = Some(input.into());
            self
        }
        /// <p>The tag key.</p>
        pub fn set_resource_tag_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.resource_tag_key = input;
            self
        }
        /// <p>The tag value.</p>
        pub fn resource_tag_value(mut self, input: impl Into<std::string::String>) -> Self {
            self.resource_tag_value = Some(input.into());
            self
        }
        /// <p>The tag value.</p>
        pub fn set_resource_tag_value(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.resource_tag_value = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceTag`](crate::model::ResourceTag)
        pub fn build(self) -> crate::model::ResourceTag {
            crate::model::ResourceTag {
                resource_tag_key: self.resource_tag_key,
                resource_tag_value: self.resource_tag_value,
            }
        }
    }
}
impl ResourceTag {
    /// Creates a new builder-style object to manufacture [`ResourceTag`](crate::model::ResourceTag)
    pub fn builder() -> crate::model::resource_tag::Builder {
        crate::model::resource_tag::Builder::default()
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
pub enum ResourceType {
    #[allow(missing_docs)] // documentation missing in model
    EbsSnapshot,
    #[allow(missing_docs)] // documentation missing in model
    Ec2Image,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ResourceType {
    fn from(s: &str) -> Self {
        match s {
            "EBS_SNAPSHOT" => ResourceType::EbsSnapshot,
            "EC2_IMAGE" => ResourceType::Ec2Image,
            other => ResourceType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ResourceType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ResourceType::from(s))
    }
}
impl ResourceType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ResourceType::EbsSnapshot => "EBS_SNAPSHOT",
            ResourceType::Ec2Image => "EC2_IMAGE",
            ResourceType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["EBS_SNAPSHOT", "EC2_IMAGE"]
    }
}
impl AsRef<str> for ResourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about the retention period for which the retention rule is to retain resources.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RetentionPeriod {
    /// <p>The period value for which the retention rule is to retain resources. The period is measured using the unit specified for <b>RetentionPeriodUnit</b>.</p>
    pub retention_period_value: std::option::Option<i32>,
    /// <p>The unit of time in which the retention period is measured. Currently, only <code>DAYS</code> is supported.</p>
    pub retention_period_unit: std::option::Option<crate::model::RetentionPeriodUnit>,
}
impl RetentionPeriod {
    /// <p>The period value for which the retention rule is to retain resources. The period is measured using the unit specified for <b>RetentionPeriodUnit</b>.</p>
    pub fn retention_period_value(&self) -> std::option::Option<i32> {
        self.retention_period_value
    }
    /// <p>The unit of time in which the retention period is measured. Currently, only <code>DAYS</code> is supported.</p>
    pub fn retention_period_unit(&self) -> std::option::Option<&crate::model::RetentionPeriodUnit> {
        self.retention_period_unit.as_ref()
    }
}
impl std::fmt::Debug for RetentionPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RetentionPeriod");
        formatter.field("retention_period_value", &self.retention_period_value);
        formatter.field("retention_period_unit", &self.retention_period_unit);
        formatter.finish()
    }
}
/// See [`RetentionPeriod`](crate::model::RetentionPeriod)
pub mod retention_period {
    /// A builder for [`RetentionPeriod`](crate::model::RetentionPeriod)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) retention_period_value: std::option::Option<i32>,
        pub(crate) retention_period_unit: std::option::Option<crate::model::RetentionPeriodUnit>,
    }
    impl Builder {
        /// <p>The period value for which the retention rule is to retain resources. The period is measured using the unit specified for <b>RetentionPeriodUnit</b>.</p>
        pub fn retention_period_value(mut self, input: i32) -> Self {
            self.retention_period_value = Some(input);
            self
        }
        /// <p>The period value for which the retention rule is to retain resources. The period is measured using the unit specified for <b>RetentionPeriodUnit</b>.</p>
        pub fn set_retention_period_value(mut self, input: std::option::Option<i32>) -> Self {
            self.retention_period_value = input;
            self
        }
        /// <p>The unit of time in which the retention period is measured. Currently, only <code>DAYS</code> is supported.</p>
        pub fn retention_period_unit(mut self, input: crate::model::RetentionPeriodUnit) -> Self {
            self.retention_period_unit = Some(input);
            self
        }
        /// <p>The unit of time in which the retention period is measured. Currently, only <code>DAYS</code> is supported.</p>
        pub fn set_retention_period_unit(
            mut self,
            input: std::option::Option<crate::model::RetentionPeriodUnit>,
        ) -> Self {
            self.retention_period_unit = input;
            self
        }
        /// Consumes the builder and constructs a [`RetentionPeriod`](crate::model::RetentionPeriod)
        pub fn build(self) -> crate::model::RetentionPeriod {
            crate::model::RetentionPeriod {
                retention_period_value: self.retention_period_value,
                retention_period_unit: self.retention_period_unit,
            }
        }
    }
}
impl RetentionPeriod {
    /// Creates a new builder-style object to manufacture [`RetentionPeriod`](crate::model::RetentionPeriod)
    pub fn builder() -> crate::model::retention_period::Builder {
        crate::model::retention_period::Builder::default()
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
pub enum RetentionPeriodUnit {
    #[allow(missing_docs)] // documentation missing in model
    Days,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for RetentionPeriodUnit {
    fn from(s: &str) -> Self {
        match s {
            "DAYS" => RetentionPeriodUnit::Days,
            other => RetentionPeriodUnit::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for RetentionPeriodUnit {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(RetentionPeriodUnit::from(s))
    }
}
impl RetentionPeriodUnit {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            RetentionPeriodUnit::Days => "DAYS",
            RetentionPeriodUnit::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["DAYS"]
    }
}
impl AsRef<str> for RetentionPeriodUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
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
pub enum ServiceQuotaExceededExceptionReason {
    #[allow(missing_docs)] // documentation missing in model
    ServiceQuotaExceeded,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ServiceQuotaExceededExceptionReason {
    fn from(s: &str) -> Self {
        match s {
            "SERVICE_QUOTA_EXCEEDED" => ServiceQuotaExceededExceptionReason::ServiceQuotaExceeded,
            other => ServiceQuotaExceededExceptionReason::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ServiceQuotaExceededExceptionReason {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ServiceQuotaExceededExceptionReason::from(s))
    }
}
impl ServiceQuotaExceededExceptionReason {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ServiceQuotaExceededExceptionReason::ServiceQuotaExceeded => "SERVICE_QUOTA_EXCEEDED",
            ServiceQuotaExceededExceptionReason::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["SERVICE_QUOTA_EXCEEDED"]
    }
}
impl AsRef<str> for ServiceQuotaExceededExceptionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about the tags to assign to the retention rule.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>The tag key.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>The tag value.</p>
    pub value: std::option::Option<std::string::String>,
}
impl Tag {
    /// <p>The tag key.</p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The tag value.</p>
    pub fn value(&self) -> std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &self.key);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {
    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The tag key.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        /// <p>The tag key.</p>
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>The tag value.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        /// <p>The tag value.</p>
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

/// <p>Information about a Recycle Bin retention rule.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RuleSummary {
    /// <p>The unique ID of the retention rule.</p>
    pub identifier: std::option::Option<std::string::String>,
    /// <p>The retention rule description.</p>
    pub description: std::option::Option<std::string::String>,
    /// <p>Information about the retention period for which the retention rule is to retain resources.</p>
    pub retention_period: std::option::Option<crate::model::RetentionPeriod>,
}
impl RuleSummary {
    /// <p>The unique ID of the retention rule.</p>
    pub fn identifier(&self) -> std::option::Option<&str> {
        self.identifier.as_deref()
    }
    /// <p>The retention rule description.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Information about the retention period for which the retention rule is to retain resources.</p>
    pub fn retention_period(&self) -> std::option::Option<&crate::model::RetentionPeriod> {
        self.retention_period.as_ref()
    }
}
impl std::fmt::Debug for RuleSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RuleSummary");
        formatter.field("identifier", &self.identifier);
        formatter.field("description", &self.description);
        formatter.field("retention_period", &self.retention_period);
        formatter.finish()
    }
}
/// See [`RuleSummary`](crate::model::RuleSummary)
pub mod rule_summary {
    /// A builder for [`RuleSummary`](crate::model::RuleSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) identifier: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) retention_period: std::option::Option<crate::model::RetentionPeriod>,
    }
    impl Builder {
        /// <p>The unique ID of the retention rule.</p>
        pub fn identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.identifier = Some(input.into());
            self
        }
        /// <p>The unique ID of the retention rule.</p>
        pub fn set_identifier(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.identifier = input;
            self
        }
        /// <p>The retention rule description.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        /// <p>The retention rule description.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input;
            self
        }
        /// <p>Information about the retention period for which the retention rule is to retain resources.</p>
        pub fn retention_period(mut self, input: crate::model::RetentionPeriod) -> Self {
            self.retention_period = Some(input);
            self
        }
        /// <p>Information about the retention period for which the retention rule is to retain resources.</p>
        pub fn set_retention_period(
            mut self,
            input: std::option::Option<crate::model::RetentionPeriod>,
        ) -> Self {
            self.retention_period = input;
            self
        }
        /// Consumes the builder and constructs a [`RuleSummary`](crate::model::RuleSummary)
        pub fn build(self) -> crate::model::RuleSummary {
            crate::model::RuleSummary {
                identifier: self.identifier,
                description: self.description,
                retention_period: self.retention_period,
            }
        }
    }
}
impl RuleSummary {
    /// Creates a new builder-style object to manufacture [`RuleSummary`](crate::model::RuleSummary)
    pub fn builder() -> crate::model::rule_summary::Builder {
        crate::model::rule_summary::Builder::default()
    }
}
