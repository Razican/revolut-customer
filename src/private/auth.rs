//! Authorization methods of the API.

use failure::{Error, ResultExt};
use lazy_static::lazy_static;
use reqwest::{StatusCode, Url};

use super::{User, Wallet};
use crate::{error, Client, BASE_API_URL};

/// Authorization client methods
impl Client {
    /// Signs the user in.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use revolut_customer::{Client, error};
    ///
    /// let client = Client::default();
    /// let response = client.sign_in("+1555555555", "9999");
    /// assert_eq!(response.err().unwrap().downcast_ref::<error::Api>().unwrap(),
    ///            &error::Api::Unauthorized);
    /// ```
    ///
    /// Note that the response will be an unauthorized error, since the phone/password combination
    /// is not correct.
    ///
    /// ## Request API specification
    ///
    /// No authentication required.
    ///
    /// ```text
    /// GET https://api.revolut.com/signin
    /// ```
    ///
    /// **Body (JSON encoded):**
    ///
    /// ```json
    /// {
    ///     "phone": "+1555555555",
    ///     "password": "9999"
    /// }
    /// ```
    ///
    /// The response status code will be in the `2XX` range if the phone/password were correct, or
    /// in the `4XX` range if they weren't or the API changed. The response will not have further
    /// information.
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

    /// Confirms the user sign-in.
    ///
    /// This will set the client with the user ID and the access token so that it can perform
    /// further requests that require authentication. That's the reason why the client needs to be
    /// mutable.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// use revolut_customer::{Client, error};
    ///
    /// let mut client = Client::default();
    /// let response = client.confirm_sign_in("+1555555555", "111-111");
    /// assert!(response.is_err());
    /// ```
    ///
    /// Note that the response will be a 400 error, since the phone/code combination is not correct.
    ///
    /// ## Request API specification
    ///
    /// No authentication required.
    ///
    /// ```text
    /// GET https://api.revolut.com/signin/confirm
    /// ```
    ///
    /// **Body (JSON encoded):**
    ///
    /// ```json
    /// {
    ///     "phone": "+1555555555",
    ///     "code": "111-111"
    /// }
    /// ```
    ///
    /// The response status code will be in the `2XX` range if the phone/code were correct, or in
    /// the `4XX` range if they weren't or the API changed. If the response is correct, a JSON
    /// object containing the user, wallet and access token for the user si returned. The
    /// implementation only returns the user and wallet objects, and saves the access token and
    /// user ID to authenticate in future requests.
    ///
    /// The definitions for these objects is shown in the methods that specifically return each of
    /// the types.
    pub fn confirm_sign_in<P, C>(&mut self, phone: P, code: C) -> Result<(User, Wallet), Error>
    where
        P: AsRef<str>,
        C: AsRef<str>,
    {
        /// Response of the sign-in mechanism.
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct SignInResponse {
            /// User information.
            user: User,
            /// Wallet information.
            wallet: Wallet,
            /// Access token.
            access_token: String,
        }

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
        let request_builder = self.set_headers(request_builder).json(&data);

        let mut response = request_builder.send().context(error::Api::RequestFailure)?;

        if response.status().is_success() {
            let res_structure: SignInResponse =
                response.json().context(error::Api::ParseResponse)?;
            self.user_id = Some(res_structure.user.id);
            self.access_token = Some(res_structure.access_token);
            Ok((res_structure.user, res_structure.wallet))
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
