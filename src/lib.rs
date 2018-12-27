//! Revolut customer API
//!
//! This crate is meant to interact with the Revolut customer API, not to be confused with the
//! business API. This API is not public, and therefore it's subject to change.

#![forbid(anonymous_parameters)]
#![warn(clippy::pedantic)]
#![deny(
    clippy::all,
    variant_size_differences,
    unused_results,
    unused_qualifications,
    unused_import_braces,
    unsafe_code,
    trivial_numeric_casts,
    trivial_casts,
    missing_docs,
    unused_extern_crates,
    missing_debug_implementations,
    missing_copy_implementations
)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

pub mod amount;
pub mod error;
pub mod private;
mod public;

use lazy_static::lazy_static;
use reqwest::{RequestBuilder, Url};

pub use crate::amount::Amount;

lazy_static! {
    /// Base URL for the API.
    static ref BASE_API_URL: Url = Url::parse("https://api.revolut.com/")
                                    .expect("error parsing the base API URL");
}

/// Options for the client configuration.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Options {
    /// Version of the client.
    client_version: String,
    /// Version of the API.
    api_version: String,
    /// Identification of the device.
    device_id: String,
    /// Model of the device.
    device_model: String,
    /// User agent of the device.
    #[builder(default = "String::from(\"Revolut/com.revolut.revolut (iPhone; iOS 11.1)\")")]
    user_agent: String,
}

/// API client structure.
#[derive(Debug, Clone)]
pub struct Client {
    /// Options for the client.
    options: Options,
    /// HTTP client.
    client: reqwest::Client,
}

impl Client {
    /// Creates a new client with the given options.
    pub fn new(options: Options) -> Self {
        Self {
            options,
            client: reqwest::Client::new(),
        }
    }

    /// Sets the headers with the provided documentation.
    fn set_headers(&self, mut request_builder: RequestBuilder) -> RequestBuilder {
        if !self.options.client_version.is_empty() {
            request_builder =
                request_builder.header("X-Client-Version", self.options.client_version.as_str());
        }
        if !self.options.api_version.is_empty() {
            request_builder =
                request_builder.header("X-Api-Version", self.options.api_version.as_str());
        }
        if !self.options.device_id.is_empty() {
            request_builder =
                request_builder.header("X-Device-Id", self.options.device_id.as_str());
        }
        if !self.options.device_model.is_empty() {
            request_builder =
                request_builder.header("X-Device-Model", self.options.device_id.as_str());
        }
        if !self.options.user_agent.is_empty() {
            request_builder = request_builder.header(
                reqwest::header::USER_AGENT,
                self.options.user_agent.as_str(),
            );
        }
        request_builder
    }
}
