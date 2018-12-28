//! Error module.

use reqwest::StatusCode;

/// Revolut amount parse error.
#[derive(Debug, Clone, Fail, PartialEq)]
#[fail(display = "the amount {} is not a valid Revolut amount", amount_str)]
pub struct AmountParse {
    pub(crate) amount_str: String,
}

/// API error.
#[derive(Debug, Clone, Copy, Fail, PartialEq)]
pub enum Api {
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
