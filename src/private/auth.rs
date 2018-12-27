//! Authorization methods of the API.

use failure::{Error, ResultExt};
use lazy_static::lazy_static;
use reqwest::{StatusCode, Url};

use super::{AccessToken, User, Wallet};
use crate::{error, Client, BASE_API_URL};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInResponse {
    /// User information.
    user: User,
    /// Wallet information.
    wallet: Wallet,
    /// Access token.
    access_token: AccessToken,
}

/// Authorization client methods
impl Client {
    /// Signs the user in.
    pub fn sign_in<PH, PW>(&self, phone: PH, password: PW) -> Result<(), Error>
    where
        PH: AsRef<str>,
        PW: AsRef<str>,
    {
        /// Data to send to the endpoint in the JSON body.
        #[derive(Debug, Serialize)]
        struct Data<'d> {
            phone: &'d str,
            password: &'d str,
        }

        lazy_static! {
            /// URL of the endpoint.
            static ref URL: Url = BASE_API_URL.join("signin").unwrap();
        }

        let data = Data {
            phone: phone.as_ref(),
            password: password.as_ref(),
        };

        let request_builder = self.client.post(URL.clone());

        let response = self
            .set_headers(request_builder)
            .json(&data)
            .send()
            .context(error::Api::RequestFailure)?;

        if response.status().is_success() {
            Ok(())
        } else if response.status() == StatusCode::UNAUTHORIZED {
            Err(error::Api::Unauthorized.into())
        } else {
            Err(error::Api::Other {
                status_code: response.status(),
            }
            .into())
        }
    }

    /// Signs the user in.
    pub fn confirm_sign_in<P, C>(&self, phone: P, code: C) -> Result<SignInResponse, Error>
    where
        P: AsRef<str>,
        C: AsRef<str>,
    {
        /// Data to send to the endpoint in the JSON body.
        #[derive(Debug, Serialize)]
        struct Data<'d> {
            phone: &'d str,
            code: &'d str,
        }

        lazy_static! {
            /// URL of the endpoint.
            static ref URL: Url = BASE_API_URL.join("signin/confirm").unwrap();
        }

        let data = Data {
            phone: phone.as_ref(),
            code: &code.as_ref().replace('-', ""),
        };

        let request_builder = self.client.post(URL.clone());

        let mut response = self
            .set_headers(request_builder)
            .json(&data)
            .send()
            .context(error::Api::RequestFailure)?;

        eprintln!("{}", response.status());

        if response.status().is_success() {
            Ok(response.json().context(error::Api::ParseResponse)?)
        } else if response.status() == StatusCode::UNAUTHORIZED {
            Err(error::Api::Unauthorized.into())
        } else {
            Err(error::Api::Other {
                status_code: response.status(),
            }
            .into())
        }
    }
}
