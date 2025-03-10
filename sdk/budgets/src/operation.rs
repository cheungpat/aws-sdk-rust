// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateBudget`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_budget`](crate::client::Client::create_budget).
///
/// See [`crate::client::fluent_builders::CreateBudget`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateBudget {
    _private: (),
}
impl CreateBudget {
    /// Creates a new builder-style object to manufacture [`CreateBudgetInput`](crate::input::CreateBudgetInput)
    pub fn builder() -> crate::input::create_budget_input::Builder {
        crate::input::create_budget_input::Builder::default()
    }
    /// Creates a new `CreateBudget` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateBudget {
    type Output =
        std::result::Result<crate::output::CreateBudgetOutput, crate::error::CreateBudgetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_budget_error(response)
        } else {
            crate::operation_deser::parse_create_budget_response(response)
        }
    }
}

/// Operation shape for `CreateBudgetAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_budget_action`](crate::client::Client::create_budget_action).
///
/// See [`crate::client::fluent_builders::CreateBudgetAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateBudgetAction {
    _private: (),
}
impl CreateBudgetAction {
    /// Creates a new builder-style object to manufacture [`CreateBudgetActionInput`](crate::input::CreateBudgetActionInput)
    pub fn builder() -> crate::input::create_budget_action_input::Builder {
        crate::input::create_budget_action_input::Builder::default()
    }
    /// Creates a new `CreateBudgetAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateBudgetAction {
    type Output = std::result::Result<
        crate::output::CreateBudgetActionOutput,
        crate::error::CreateBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_budget_action_error(response)
        } else {
            crate::operation_deser::parse_create_budget_action_response(response)
        }
    }
}

/// Operation shape for `CreateNotification`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_notification`](crate::client::Client::create_notification).
///
/// See [`crate::client::fluent_builders::CreateNotification`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateNotification {
    _private: (),
}
impl CreateNotification {
    /// Creates a new builder-style object to manufacture [`CreateNotificationInput`](crate::input::CreateNotificationInput)
    pub fn builder() -> crate::input::create_notification_input::Builder {
        crate::input::create_notification_input::Builder::default()
    }
    /// Creates a new `CreateNotification` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateNotification {
    type Output = std::result::Result<
        crate::output::CreateNotificationOutput,
        crate::error::CreateNotificationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_notification_error(response)
        } else {
            crate::operation_deser::parse_create_notification_response(response)
        }
    }
}

/// Operation shape for `CreateSubscriber`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_subscriber`](crate::client::Client::create_subscriber).
///
/// See [`crate::client::fluent_builders::CreateSubscriber`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateSubscriber {
    _private: (),
}
impl CreateSubscriber {
    /// Creates a new builder-style object to manufacture [`CreateSubscriberInput`](crate::input::CreateSubscriberInput)
    pub fn builder() -> crate::input::create_subscriber_input::Builder {
        crate::input::create_subscriber_input::Builder::default()
    }
    /// Creates a new `CreateSubscriber` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSubscriber {
    type Output = std::result::Result<
        crate::output::CreateSubscriberOutput,
        crate::error::CreateSubscriberError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_subscriber_error(response)
        } else {
            crate::operation_deser::parse_create_subscriber_response(response)
        }
    }
}

/// Operation shape for `DeleteBudget`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_budget`](crate::client::Client::delete_budget).
///
/// See [`crate::client::fluent_builders::DeleteBudget`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteBudget {
    _private: (),
}
impl DeleteBudget {
    /// Creates a new builder-style object to manufacture [`DeleteBudgetInput`](crate::input::DeleteBudgetInput)
    pub fn builder() -> crate::input::delete_budget_input::Builder {
        crate::input::delete_budget_input::Builder::default()
    }
    /// Creates a new `DeleteBudget` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteBudget {
    type Output =
        std::result::Result<crate::output::DeleteBudgetOutput, crate::error::DeleteBudgetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_budget_error(response)
        } else {
            crate::operation_deser::parse_delete_budget_response(response)
        }
    }
}

/// Operation shape for `DeleteBudgetAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_budget_action`](crate::client::Client::delete_budget_action).
///
/// See [`crate::client::fluent_builders::DeleteBudgetAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteBudgetAction {
    _private: (),
}
impl DeleteBudgetAction {
    /// Creates a new builder-style object to manufacture [`DeleteBudgetActionInput`](crate::input::DeleteBudgetActionInput)
    pub fn builder() -> crate::input::delete_budget_action_input::Builder {
        crate::input::delete_budget_action_input::Builder::default()
    }
    /// Creates a new `DeleteBudgetAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteBudgetAction {
    type Output = std::result::Result<
        crate::output::DeleteBudgetActionOutput,
        crate::error::DeleteBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_budget_action_error(response)
        } else {
            crate::operation_deser::parse_delete_budget_action_response(response)
        }
    }
}

/// Operation shape for `DeleteNotification`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_notification`](crate::client::Client::delete_notification).
///
/// See [`crate::client::fluent_builders::DeleteNotification`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteNotification {
    _private: (),
}
impl DeleteNotification {
    /// Creates a new builder-style object to manufacture [`DeleteNotificationInput`](crate::input::DeleteNotificationInput)
    pub fn builder() -> crate::input::delete_notification_input::Builder {
        crate::input::delete_notification_input::Builder::default()
    }
    /// Creates a new `DeleteNotification` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteNotification {
    type Output = std::result::Result<
        crate::output::DeleteNotificationOutput,
        crate::error::DeleteNotificationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_notification_error(response)
        } else {
            crate::operation_deser::parse_delete_notification_response(response)
        }
    }
}

/// Operation shape for `DeleteSubscriber`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_subscriber`](crate::client::Client::delete_subscriber).
///
/// See [`crate::client::fluent_builders::DeleteSubscriber`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSubscriber {
    _private: (),
}
impl DeleteSubscriber {
    /// Creates a new builder-style object to manufacture [`DeleteSubscriberInput`](crate::input::DeleteSubscriberInput)
    pub fn builder() -> crate::input::delete_subscriber_input::Builder {
        crate::input::delete_subscriber_input::Builder::default()
    }
    /// Creates a new `DeleteSubscriber` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSubscriber {
    type Output = std::result::Result<
        crate::output::DeleteSubscriberOutput,
        crate::error::DeleteSubscriberError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_subscriber_error(response)
        } else {
            crate::operation_deser::parse_delete_subscriber_response(response)
        }
    }
}

/// Operation shape for `DescribeBudget`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_budget`](crate::client::Client::describe_budget).
///
/// See [`crate::client::fluent_builders::DescribeBudget`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudget {
    _private: (),
}
impl DescribeBudget {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetInput`](crate::input::DescribeBudgetInput)
    pub fn builder() -> crate::input::describe_budget_input::Builder {
        crate::input::describe_budget_input::Builder::default()
    }
    /// Creates a new `DescribeBudget` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBudget {
    type Output =
        std::result::Result<crate::output::DescribeBudgetOutput, crate::error::DescribeBudgetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_response(response)
        }
    }
}

/// Operation shape for `DescribeBudgetAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_budget_action`](crate::client::Client::describe_budget_action).
///
/// See [`crate::client::fluent_builders::DescribeBudgetAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetAction {
    _private: (),
}
impl DescribeBudgetAction {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetActionInput`](crate::input::DescribeBudgetActionInput)
    pub fn builder() -> crate::input::describe_budget_action_input::Builder {
        crate::input::describe_budget_action_input::Builder::default()
    }
    /// Creates a new `DescribeBudgetAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBudgetAction {
    type Output = std::result::Result<
        crate::output::DescribeBudgetActionOutput,
        crate::error::DescribeBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_action_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_action_response(response)
        }
    }
}

/// Operation shape for `DescribeBudgetActionHistories`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_budget_action_histories`](crate::client::Client::describe_budget_action_histories).
///
/// See [`crate::client::fluent_builders::DescribeBudgetActionHistories`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetActionHistories {
    _private: (),
}
impl DescribeBudgetActionHistories {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetActionHistoriesInput`](crate::input::DescribeBudgetActionHistoriesInput)
    pub fn builder() -> crate::input::describe_budget_action_histories_input::Builder {
        crate::input::describe_budget_action_histories_input::Builder::default()
    }
    /// Creates a new `DescribeBudgetActionHistories` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBudgetActionHistories {
    type Output = std::result::Result<
        crate::output::DescribeBudgetActionHistoriesOutput,
        crate::error::DescribeBudgetActionHistoriesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_action_histories_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_action_histories_response(response)
        }
    }
}

/// Operation shape for `DescribeBudgetActionsForAccount`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_budget_actions_for_account`](crate::client::Client::describe_budget_actions_for_account).
///
/// See [`crate::client::fluent_builders::DescribeBudgetActionsForAccount`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetActionsForAccount {
    _private: (),
}
impl DescribeBudgetActionsForAccount {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetActionsForAccountInput`](crate::input::DescribeBudgetActionsForAccountInput)
    pub fn builder() -> crate::input::describe_budget_actions_for_account_input::Builder {
        crate::input::describe_budget_actions_for_account_input::Builder::default()
    }
    /// Creates a new `DescribeBudgetActionsForAccount` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBudgetActionsForAccount {
    type Output = std::result::Result<
        crate::output::DescribeBudgetActionsForAccountOutput,
        crate::error::DescribeBudgetActionsForAccountError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_actions_for_account_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_actions_for_account_response(response)
        }
    }
}

/// Operation shape for `DescribeBudgetActionsForBudget`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_budget_actions_for_budget`](crate::client::Client::describe_budget_actions_for_budget).
///
/// See [`crate::client::fluent_builders::DescribeBudgetActionsForBudget`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetActionsForBudget {
    _private: (),
}
impl DescribeBudgetActionsForBudget {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetActionsForBudgetInput`](crate::input::DescribeBudgetActionsForBudgetInput)
    pub fn builder() -> crate::input::describe_budget_actions_for_budget_input::Builder {
        crate::input::describe_budget_actions_for_budget_input::Builder::default()
    }
    /// Creates a new `DescribeBudgetActionsForBudget` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBudgetActionsForBudget {
    type Output = std::result::Result<
        crate::output::DescribeBudgetActionsForBudgetOutput,
        crate::error::DescribeBudgetActionsForBudgetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_actions_for_budget_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_actions_for_budget_response(response)
        }
    }
}

/// Operation shape for `DescribeBudgetNotificationsForAccount`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_budget_notifications_for_account`](crate::client::Client::describe_budget_notifications_for_account).
///
/// See [`crate::client::fluent_builders::DescribeBudgetNotificationsForAccount`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetNotificationsForAccount {
    _private: (),
}
impl DescribeBudgetNotificationsForAccount {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetNotificationsForAccountInput`](crate::input::DescribeBudgetNotificationsForAccountInput)
    pub fn builder() -> crate::input::describe_budget_notifications_for_account_input::Builder {
        crate::input::describe_budget_notifications_for_account_input::Builder::default()
    }
    /// Creates a new `DescribeBudgetNotificationsForAccount` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBudgetNotificationsForAccount {
    type Output = std::result::Result<
        crate::output::DescribeBudgetNotificationsForAccountOutput,
        crate::error::DescribeBudgetNotificationsForAccountError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_notifications_for_account_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_notifications_for_account_response(
                response,
            )
        }
    }
}

/// Operation shape for `DescribeBudgetPerformanceHistory`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_budget_performance_history`](crate::client::Client::describe_budget_performance_history).
///
/// See [`crate::client::fluent_builders::DescribeBudgetPerformanceHistory`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgetPerformanceHistory {
    _private: (),
}
impl DescribeBudgetPerformanceHistory {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetPerformanceHistoryInput`](crate::input::DescribeBudgetPerformanceHistoryInput)
    pub fn builder() -> crate::input::describe_budget_performance_history_input::Builder {
        crate::input::describe_budget_performance_history_input::Builder::default()
    }
    /// Creates a new `DescribeBudgetPerformanceHistory` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBudgetPerformanceHistory {
    type Output = std::result::Result<
        crate::output::DescribeBudgetPerformanceHistoryOutput,
        crate::error::DescribeBudgetPerformanceHistoryError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budget_performance_history_error(response)
        } else {
            crate::operation_deser::parse_describe_budget_performance_history_response(response)
        }
    }
}

/// Operation shape for `DescribeBudgets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_budgets`](crate::client::Client::describe_budgets).
///
/// See [`crate::client::fluent_builders::DescribeBudgets`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeBudgets {
    _private: (),
}
impl DescribeBudgets {
    /// Creates a new builder-style object to manufacture [`DescribeBudgetsInput`](crate::input::DescribeBudgetsInput)
    pub fn builder() -> crate::input::describe_budgets_input::Builder {
        crate::input::describe_budgets_input::Builder::default()
    }
    /// Creates a new `DescribeBudgets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBudgets {
    type Output = std::result::Result<
        crate::output::DescribeBudgetsOutput,
        crate::error::DescribeBudgetsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_budgets_error(response)
        } else {
            crate::operation_deser::parse_describe_budgets_response(response)
        }
    }
}

/// Operation shape for `DescribeNotificationsForBudget`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_notifications_for_budget`](crate::client::Client::describe_notifications_for_budget).
///
/// See [`crate::client::fluent_builders::DescribeNotificationsForBudget`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeNotificationsForBudget {
    _private: (),
}
impl DescribeNotificationsForBudget {
    /// Creates a new builder-style object to manufacture [`DescribeNotificationsForBudgetInput`](crate::input::DescribeNotificationsForBudgetInput)
    pub fn builder() -> crate::input::describe_notifications_for_budget_input::Builder {
        crate::input::describe_notifications_for_budget_input::Builder::default()
    }
    /// Creates a new `DescribeNotificationsForBudget` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeNotificationsForBudget {
    type Output = std::result::Result<
        crate::output::DescribeNotificationsForBudgetOutput,
        crate::error::DescribeNotificationsForBudgetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_notifications_for_budget_error(response)
        } else {
            crate::operation_deser::parse_describe_notifications_for_budget_response(response)
        }
    }
}

/// Operation shape for `DescribeSubscribersForNotification`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_subscribers_for_notification`](crate::client::Client::describe_subscribers_for_notification).
///
/// See [`crate::client::fluent_builders::DescribeSubscribersForNotification`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSubscribersForNotification {
    _private: (),
}
impl DescribeSubscribersForNotification {
    /// Creates a new builder-style object to manufacture [`DescribeSubscribersForNotificationInput`](crate::input::DescribeSubscribersForNotificationInput)
    pub fn builder() -> crate::input::describe_subscribers_for_notification_input::Builder {
        crate::input::describe_subscribers_for_notification_input::Builder::default()
    }
    /// Creates a new `DescribeSubscribersForNotification` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSubscribersForNotification {
    type Output = std::result::Result<
        crate::output::DescribeSubscribersForNotificationOutput,
        crate::error::DescribeSubscribersForNotificationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_subscribers_for_notification_error(response)
        } else {
            crate::operation_deser::parse_describe_subscribers_for_notification_response(response)
        }
    }
}

/// Operation shape for `ExecuteBudgetAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`execute_budget_action`](crate::client::Client::execute_budget_action).
///
/// See [`crate::client::fluent_builders::ExecuteBudgetAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ExecuteBudgetAction {
    _private: (),
}
impl ExecuteBudgetAction {
    /// Creates a new builder-style object to manufacture [`ExecuteBudgetActionInput`](crate::input::ExecuteBudgetActionInput)
    pub fn builder() -> crate::input::execute_budget_action_input::Builder {
        crate::input::execute_budget_action_input::Builder::default()
    }
    /// Creates a new `ExecuteBudgetAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExecuteBudgetAction {
    type Output = std::result::Result<
        crate::output::ExecuteBudgetActionOutput,
        crate::error::ExecuteBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_execute_budget_action_error(response)
        } else {
            crate::operation_deser::parse_execute_budget_action_response(response)
        }
    }
}

/// Operation shape for `UpdateBudget`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_budget`](crate::client::Client::update_budget).
///
/// See [`crate::client::fluent_builders::UpdateBudget`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateBudget {
    _private: (),
}
impl UpdateBudget {
    /// Creates a new builder-style object to manufacture [`UpdateBudgetInput`](crate::input::UpdateBudgetInput)
    pub fn builder() -> crate::input::update_budget_input::Builder {
        crate::input::update_budget_input::Builder::default()
    }
    /// Creates a new `UpdateBudget` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateBudget {
    type Output =
        std::result::Result<crate::output::UpdateBudgetOutput, crate::error::UpdateBudgetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_budget_error(response)
        } else {
            crate::operation_deser::parse_update_budget_response(response)
        }
    }
}

/// Operation shape for `UpdateBudgetAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_budget_action`](crate::client::Client::update_budget_action).
///
/// See [`crate::client::fluent_builders::UpdateBudgetAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateBudgetAction {
    _private: (),
}
impl UpdateBudgetAction {
    /// Creates a new builder-style object to manufacture [`UpdateBudgetActionInput`](crate::input::UpdateBudgetActionInput)
    pub fn builder() -> crate::input::update_budget_action_input::Builder {
        crate::input::update_budget_action_input::Builder::default()
    }
    /// Creates a new `UpdateBudgetAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateBudgetAction {
    type Output = std::result::Result<
        crate::output::UpdateBudgetActionOutput,
        crate::error::UpdateBudgetActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_budget_action_error(response)
        } else {
            crate::operation_deser::parse_update_budget_action_response(response)
        }
    }
}

/// Operation shape for `UpdateNotification`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_notification`](crate::client::Client::update_notification).
///
/// See [`crate::client::fluent_builders::UpdateNotification`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateNotification {
    _private: (),
}
impl UpdateNotification {
    /// Creates a new builder-style object to manufacture [`UpdateNotificationInput`](crate::input::UpdateNotificationInput)
    pub fn builder() -> crate::input::update_notification_input::Builder {
        crate::input::update_notification_input::Builder::default()
    }
    /// Creates a new `UpdateNotification` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateNotification {
    type Output = std::result::Result<
        crate::output::UpdateNotificationOutput,
        crate::error::UpdateNotificationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_notification_error(response)
        } else {
            crate::operation_deser::parse_update_notification_response(response)
        }
    }
}

/// Operation shape for `UpdateSubscriber`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_subscriber`](crate::client::Client::update_subscriber).
///
/// See [`crate::client::fluent_builders::UpdateSubscriber`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSubscriber {
    _private: (),
}
impl UpdateSubscriber {
    /// Creates a new builder-style object to manufacture [`UpdateSubscriberInput`](crate::input::UpdateSubscriberInput)
    pub fn builder() -> crate::input::update_subscriber_input::Builder {
        crate::input::update_subscriber_input::Builder::default()
    }
    /// Creates a new `UpdateSubscriber` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSubscriber {
    type Output = std::result::Result<
        crate::output::UpdateSubscriberOutput,
        crate::error::UpdateSubscriberError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_subscriber_error(response)
        } else {
            crate::operation_deser::parse_update_subscriber_response(response)
        }
    }
}
