#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>This is the API Reference for <a href="https://docs.aws.amazon.com/rekognition/latest/dg/images.html">Amazon Rekognition Image</a>,
//! <a href="https://docs.aws.amazon.com/rekognition/latest/customlabels-dg/what-is.html">Amazon Rekognition Custom Labels</a>,
//! <a href="https://docs.aws.amazon.com/rekognition/latest/dg/video.html">Amazon Rekognition Stored Video</a>,      
//! <a href="https://docs.aws.amazon.com/rekognition/latest/dg/streaming-video.html">Amazon Rekognition Streaming Video</a>.
//! It provides descriptions of actions, data types, common parameters,
//! and common errors.</p>
//!
//! <p>    
//! <b>Amazon Rekognition Image</b>
//! </p>
//!
//! <ul>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! </ul>
//!
//!
//!
//!
//!
//! <p>
//! <b>Amazon Rekognition Custom Labels</b>
//! </p>
//! <ul>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! </ul>
//!
//! <p>
//! <b>Amazon Rekognition Video Stored Video</b>
//! </p>
//!
//! <ul>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! </ul>
//!
//! <p>
//! <b>Amazon Rekognition Video Streaming Video</b>
//! </p>
//!
//! <ul>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! <li>
//! <p></p>
//! </li>
//! </ul>
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
/// Generated accessors for nested fields
mod lens;
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
/// Paginators for the service
pub mod paginator;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::Blob;
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("rekognition", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
