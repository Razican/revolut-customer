//! Revolut customer API
//!
//! This crate is meant to interact with the Revolut customer API, not to be confused with the
//! business API. This API is not public, and therefore it's subject to change.
//!
//! The HTTP API is documented for each method in the [`Client`](struct.Client.html) type.

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

pub mod amount;
pub mod private;
mod public;

use derive_builder::Builder;
use failure::{Error, Fail, ResultExt};
use getset::{Getters, Setters};
use lazy_static::lazy_static;
use reqwest::{RequestBuilder, StatusCode, Url};
use serde::Deserialize;
use uuid::Uuid;

pub use crate::amount::Amount;

lazy_static! {
    /// Base URL for the API.
    static ref BASE_API_URL: Url = Url::parse("https://api.revolut.com/")
                                    .expect("error parsing the base API URL");
}

/// API error.
#[derive(Debug, Clone, Fail, PartialEq)]
#[allow(variant_size_differences)]
pub enum ApiError {
    /// Unauthorized use of the API.
    #[fail(display = "unauthorized use of the API")]
    Unauthorized,
    /// The client had not logged in.
    #[fail(display = "the client had not logged in")]
    NotLoggedIn,
    /// Invalid user ID.
    #[fail(display = "the provided user ID is not a valid UUID")]
    InvalidUserId,
    /// Failure performing the request.
    #[fail(display = "failure performing the request")]
    RequestFailure,
    /// The request was not correctly formed.
    #[fail(
        display = "the request was not correctly formed. (message: {}, code: {:?})",
        message, code
    )]
    BadRequest {
        /// Error description.
        message: String,
        /// Revolut's error code
        code: Option<i32>,
    },
    /// The request failed for an unknown reason.
    #[fail(
        display = "request failed for an unknown reason (status code: {})",
        status_code
    )]
    Other {
        /// Status code of the API response.
        status_code: StatusCode,
    },
    /// Error parsing the API response.
    #[fail(display = "could not parse the response")]
    ParseResponse,
}

/// Error response.
#[derive(Debug, Clone, Deserialize)]
struct ErrResponse {
    pub(crate) message: String,
    pub(crate) code: Option<i32>,
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
            device_model: "iPhone8,1".to_owned(),
            user_agent: "Revolut/com.revolut.revolut (iPhone; iOS 11.1)".to_owned(),
        }
    }
}

impl Options {
    /// Gets the default iPhone options.
    pub fn iphone() -> Self {
        Self::default()
    }

    /// Gets the default Android options.
    pub fn android() -> Self {
        Self {
            // TODO: get a valid Android device model
            device_model: "android".to_owned(),
            // TODO: get a valid Android user agent
            user_agent: "Revolut/com.revolut.revolut (android)".to_owned(),
            ..Self::default()
        }
    }
}

/// API client.
///
/// TODO: Client examples
///
/// ## Request API specification
///
/// The API is not publicly available, but reverse engineering what the Revolut iPhone/Android
/// clients do, the following has been determined:
///
/// **Headers:**
///
/// The headers set by the application are the following:
///
/// ```text
/// X-Client-Version: client.version
/// X-Api-Version: 1
/// X-Device-Id: YOUR-DEVICE-ID
/// X-Device-Model: DeviceModel
/// User-Agent: AppUserAgent
/// Accept: application/json
/// ```
///
/// The client sets the following defaults for the default (iPhone) configuration:
///
/// ```text
/// X-Client-Version: 5.29
/// X-Api-Version: 1
/// X-Device-Id: SOME-DEVICE-ID
/// X-Device-Model: iPhone8,1
/// User-Agent: Revolut/com.revolut.revolut (iPhone; iOS 11.1)
/// Accept: application/json
/// ```
///
/// For the authenticated APIs, it uses simple authentication with the User ID as the user and the
/// access token as the password, adding the header:
///
/// ```text
/// Authorization: Basic e3Jldm9sdXQtdXNlci1pZH06e0FjY2Vzc1Rva2VufQ==
/// ```
///
/// The last part is the Base64 encoding of the `{revolut-user-id}:{AccessToken}` pair.
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
                .context(ApiError::InvalidUserId)?,
        );
        self.access_token = Some(access_token.into());
        Ok(())
    }

    /// Gets the logged in access token.
    pub fn user_id(&self) -> Option<Uuid> {
        self.user_id
    }

    /// Gets the logged in access token.
    pub fn access_token(&self) -> Option<&String> {
        self.access_token.as_ref()
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
