//! # vaultrs
//!
//! A rust crate for interacting with the Hashicorp Vault API
//!
//! This crate encompasses functions for interacting with the HTTP API available on
//! [Hashicorp Vault](https://!www.vaultproject.io/) servers. It uses
//! [rustify](https://!github.com/jmgilman/rustify) in order to construct accurate
//! representations of each of the endpoints available with the API. It then wraps
//! these into more usable functions intended to be consumed by users of this crate.
//!
//! The following functionality is currently supported:
//!
//! * [PKI Secrets Engine](https://!www.vaultproject.io/docs/secrets/pki)
//!
//! ## Installation
//!
//! ```
//! cargo add vaultrs
//! ```
//!
//! ## Usage
//!
//! ```
//! use vaultrs::api::pki::requests::GenerateCertificateRequest;
//! use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
//! use vaultrs::pki::cert;
//!
//! // Create a client
//! let client = VaultClient::new(
//!     VaultClientSettingsBuilder::default()
//!         .address("https://!127.0.0.1:8200")
//!         .token("TOKEN")
//!         .build()
//!         .unwrap()
//! ).unwrap();
//!
//! // Generate a certificate using the PKI backend
//! let resp = cert::generate(
//!     &client,
//!     "pki",
//!     "my_role",
//!     Some(GenerateCertificateRequest::builder().common_name("test.com")),
//! );
//!
//! // List all generated certificates
//! let res = cert::list(&client, "pki");
//! ```
//!
//! ## Error Handling
//!
//! All errors generated by this crate are wrapped in the `ClientError` enum
//! provided by the crate.
#[macro_use]
extern crate derive_builder;

pub mod api;
pub mod client;
pub mod error;
pub mod kv2;
pub mod pki;
pub mod sys;
