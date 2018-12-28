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
// Required due to bug at
// <https://github.com/colin-kiegel/rust-derive-builder/issues/139>
#![allow(clippy::default_trait_access)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

pub mod amount;
pub mod error;
pub mod private;
mod public;

use failure::{Error, ResultExt};
use lazy_static::lazy_static;
use reqwest::{RequestBuilder, Url};
use uuid::Uuid;

pub use crate::amount::Amount;

lazy_static! {
    /// Base URL for the API.
    static ref BASE_API_URL: Url = Url::parse("https://api.revolut.com/")
                                    .expect("error parsing the base API URL");
}

/// Options for the client configuration.
#[derive(Debug, Clone, Builder, Getters, Setters)]
#[builder(setter(into), default)]
pub struct Options {
    /// Version of the client.
    #[get = "pub"]
    client_version: String,
    /// Version of the API.
    #[get = "pub"]
    api_version: String,
    /// Identification of the device.
    #[get = "pub"]
    device_id: String,
    /// Model of the device.
    #[get = "pub"]
    device_model: String,
    /// User agent of the device.
    #[get = "pub"]
    user_agent: String,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            client_version: "5.29".to_owned(),
            api_version: "1".to_owned(),
            device_id: "SOME-DEVICE-ID".to_owned(),
            device_model: "".to_owned(),
            user_agent: "".to_owned(),
        }
    }
}

impl Options {
    /// Gets the default iPhone options.
    pub fn iphone() -> Self {
        Self {
            device_model: "iPhone8,1".to_owned(), // TODO: get a valid Android device model
            user_agent: "Revolut/com.revolut.revolut (iPhone; iOS 11.1)".to_owned(), // TODO: get a valid Android user agent
            ..Self::default()
        }
    }

    /// Gets the default Android options.
    pub fn android() -> Self {
        Self {
            device_model: "".to_owned(), // TODO: get a valid Android device model
            user_agent: "".to_owned(),   // TODO: get a valid Android user agent
            ..Self::default()
        }
    }
}

/// API client structure.
#[derive(Debug, Clone)]
pub struct Client {
    /// Options for the client.
    options: Options,
    /// HTTP client.
    client: reqwest::Client,
    /// Client ID.
    user_id: Option<Uuid>,
    /// Access token.
    access_token: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            options: Options::default(),
            user_id: None,
            access_token: None,
        }
    }
}

impl Client {
    /// Creates a new client with the given options.
    pub fn with_options(options: Options) -> Self {
        Self {
            options,
            ..Self::default()
        }
    }

    /// Changes the options of the client.
    pub fn set_options(&mut self, options: Options) {
        self.options = options;
    }

    /// Sets the user authentication information for the client.
    pub fn set_auth<I, T>(&mut self, user_id: I, access_token: T) -> Result<(), Error>
    where
        I: AsRef<str>, // TODO: TryInto<Uuid>
        T: Into<String>,
    {
        self.user_id = Some(
            user_id
                .as_ref()
                .parse::<Uuid>()
                .context(error::Api::InvalidUserId)?,
        );
        self.access_token = Some(access_token.into());
        Ok(())
    }

    /// Removes the user authentication information.
    ///
    /// This is effectively logging the user out.
    pub fn unset_auth(&mut self) {
        self.user_id = None;
        self.access_token = None;
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
