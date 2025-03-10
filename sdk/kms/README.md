# aws-sdk-kms

**Please Note: The SDK is currently in Developer Preview and is intended strictly for
feedback purposes only. Do not use this SDK for production workloads.**

Key Management Service (KMS) is an encryption and key management web service. This guide describes the KMS operations that you can call programmatically. For general information about KMS, see the [_Key Management Service Developer Guide_](https://docs.aws.amazon.com/kms/latest/developerguide/).

We recommend that you use the Amazon Web Services SDKs to make programmatic API calls to KMS.

If you need to use FIPS 140-2 validated cryptographic modules when communicating with Amazon Web Services, use the FIPS endpoint in your preferred Amazon Web Services Region. For more information about the available FIPS endpoints, see [Service endpoints](https://docs.aws.amazon.com/general/latest/gr/kms.html#kms_region) in the Key Management Service topic of the _Amazon Web Services General Reference_.

Clients must support TLS (Transport Layer Security) 1.0. We recommend TLS 1.2. Clients must also support cipher suites with Perfect Forward Secrecy (PFS) such as Ephemeral Diffie-Hellman (DHE) or Elliptic Curve Ephemeral Diffie-Hellman (ECDHE). Most modern systems such as Java 7 and later support these modes.

__Signing Requests__

Requests must be signed by using an access key ID and a secret access key. We strongly recommend that you _do not_ use your Amazon Web Services account (root) access key ID and secret key for everyday work with KMS. Instead, use the access key ID and secret access key for an IAM user. You can also use the Amazon Web Services Security Token Service to generate temporary security credentials that you can use to sign requests.

All KMS operations require [Signature Version 4](https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html).

__Logging API Requests__

KMS supports CloudTrail, a service that logs Amazon Web Services API calls and related events for your Amazon Web Services account and delivers them to an Amazon S3 bucket that you specify. By using the information collected by CloudTrail, you can determine what requests were made to KMS, who made the request, when it was made, and so on. To learn more about CloudTrail, including how to turn it on and find your log files, see the [CloudTrail User Guide](https://docs.aws.amazon.com/awscloudtrail/latest/userguide/).

__Additional Resources__

For more information about credentials and request signing, see the following:
  - [Amazon Web Services Security Credentials](https://docs.aws.amazon.com/general/latest/gr/aws-security-credentials.html) - This topic provides general information about the types of credentials used to access Amazon Web Services.
  - [Temporary Security Credentials](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp.html) - This section of the _IAM User Guide_ describes how to create and use temporary security credentials.
  - [Signature Version 4 Signing Process](https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html) - This set of topics walks you through the process of signing a request using an access key ID and a secret access key.

__Commonly Used API Operations__

Of the API operations discussed in this guide, the following will prove the most useful for most applications. You will likely perform operations other than these, such as creating keys and assigning policies, by using the console.
  - Encrypt
  - Decrypt
  - GenerateDataKey
  - GenerateDataKeyWithoutPlaintext

## Getting Started

> Examples are available for many services and operations, check out the
> [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-kms` to
your project, add the following to your **Cargo.toml** file:

```toml
[dependencies]
aws-config = "0.11.0"
aws-sdk-kms = "0.11.0"
tokio = { version = "1", features = ["full"] }
```

## Using the SDK

Until the SDK is released, we will be adding information about using the SDK to the
[Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
additional sections for the guide by opening an issue and describing what you are trying to do.

## Getting Help

* [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
* [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) – For bug reports & feature requests
* [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
* [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)

## License

This project is licensed under the Apache-2.0 License.

