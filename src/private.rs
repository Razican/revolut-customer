//! Private methods of the client.

use chrono::{DateTime, NaiveDate, Utc};
use failure::{Error, ResultExt};
use lazy_static::lazy_static;
use reqwest::{StatusCode, Url};
use serde::Deserializer;

use crate::{amount::Amount, error, Client, BASE_API_URL};

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

/// API Access token.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct AccessToken {
    /// Actual token string.
    token: String,
}

/// Public client methods.
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

/// Structure representing an address.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    city: String,
    country: String,
    postcode: String,
    region: String,
    street_line_1: String,
    street_ine_2: Option<String>,
}

/// Unknown `sof` structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sof {
    state: String,
}

/// User information structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_date: DateTime<Utc>,
    address: Address,
    #[serde(deserialize_with = "deserialize_user_birth_date")]
    birth_date: NaiveDate,
    first_name: String,
    last_name: String,
    phone: String,
    email: String,
    email_verified: bool,
    state: String,
    referral_code: String,
    kyc: String,
    terms_version: String,
    under_review: bool,
    risk_assessed: bool,
    locale: String,
    sof: Sof,
}

/// Pocket information structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pocket {
    id: String,
    #[serde(rename = "type")]
    pocket_type: String,
    state: String,
    currency: String,
    balance: Amount,
    blocked_amount: Amount,
    closed: bool,
    credit_limit: Amount,
}

/// Wallet information structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    id: String,
    #[serde(rename = "ref")]
    reference: String,
    state: String,
    base_currency: String,
    total_topup: Amount,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    topup_reset_date: DateTime<Utc>,
    pockets: Box<[Pocket]>,
}

/// Deserializes the birth date of the user information structure.
fn deserialize_user_birth_date<'de, D>(de: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{SeqAccess, Visitor};
    use std::fmt;

    struct SeqVisitor;

    impl<'de> Visitor<'de> for SeqVisitor {
        type Value = (i32, u32, u32);

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array of 3 integers")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            use serde::de::Error as SerdeError;

            let mut date = (0_i32, 0_u32, 0_u32);
            date.0 = seq
                .next_element()?
                .ok_or_else(|| A::Error::custom("first integer not found"))?;
            date.1 = seq
                .next_element()?
                .ok_or_else(|| A::Error::custom("second integer not found"))?;
            date.2 = seq
                .next_element()?
                .ok_or_else(|| A::Error::custom("third integer not found"))?;

            Ok(date)
        }
    }

    let (year, month, day) = de.deserialize_seq(SeqVisitor)?;
    Ok(NaiveDate::from_ymd(year, month, day))
}
