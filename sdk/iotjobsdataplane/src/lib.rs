#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>AWS IoT Jobs is a service that allows you to define a set of jobs — remote operations that are sent to
//! and executed on one or more devices connected to AWS IoT. For example, you can define a job that instructs a
//! set of devices to download and install application or firmware updates, reboot, rotate certificates, or perform
//! remote troubleshooting operations.</p>
//! <p> To create a job, you make a job document which is a description of the remote operations to be
//! performed, and you specify a list of targets that should perform the operations. The targets can be individual
//! things, thing groups or both.</p>
//! <p> AWS IoT Jobs sends a message to inform the targets that a job is available. The target starts the
//! execution of the job by downloading the job document, performing the operations it specifies, and reporting its
//! progress to AWS IoT. The Jobs service provides commands to track the progress of a job on a specific target and
//! for all the targets of the job</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::Blob;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("iotjobsdataplane", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
