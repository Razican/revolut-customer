//! User methods of the API.

use chrono::{DateTime, NaiveDate, Utc};
use failure::{Error, ResultExt};
use lazy_static::lazy_static;
use reqwest::{header::ACCEPT, StatusCode, Url};
use serde::Deserializer;
use uuid::Uuid;

use super::{AccessToken, Address, User, Wallet};
use crate::{amount::Amount, error, Client, BASE_API_URL};

/// Response to the `current_user()` method.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUserResponse {
    /// User information.
    user: User,
    /// Wallet information.
    wallet: Wallet,
}

/// User client methods
impl Client {
    /// Gets user information.
    pub fn current_user<I>(
        &self,
        user_id: I,
        access_token: &AccessToken,
    ) -> Result<CurrentUserResponse, Error>
    where
        I: AsRef<str>,
    {
        lazy_static! {
            /// URL of the endpoint.
            static ref URL: Url = BASE_API_URL.join("user/current").unwrap();
        }

        let request_builder = self.client.get(URL.clone());

        let mut response = self
            .set_headers(request_builder)
            .header(ACCEPT, "application/json")
            .basic_auth(user_id.as_ref(), Some(&access_token.token))
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

    /// Gets user's wallet information.
    pub fn current_user_wallet<I>(
        &self,
        user_id: I,
        access_token: &AccessToken,
    ) -> Result<Wallet, Error>
    where
        I: AsRef<str>,
    {
        lazy_static! {
            /// URL of the endpoint.
            static ref URL: Url = BASE_API_URL.join("user/current/wallet").unwrap();
        }

        let request_builder = self.client.get(URL.clone());

        let mut response = self
            .set_headers(request_builder)
            .header(ACCEPT, "application/json")
            .basic_auth(user_id.as_ref(), Some(&access_token.token))
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

    /// Gets user's cards information.
    pub fn current_user_cards<I>(
        &self,
        user_id: I,
        access_token: &AccessToken,
    ) -> Result<Vec<Card>, Error>
    where
        I: AsRef<str>,
    {
        lazy_static! {
            /// URL of the endpoint.
            static ref URL: Url = BASE_API_URL.join("user/current/cards").unwrap();
        }

        let request_builder = self.client.get(URL.clone());

        let mut response = self
            .set_headers(request_builder)
            .header(ACCEPT, "application/json")
            .basic_auth(user_id.as_ref(), Some(&access_token.token))
            .send()
            .context(error::Api::RequestFailure)?;

        //panic!("{}", response.text().unwrap());

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

/// Credit card representation.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    id: Uuid,
    owner_id: Uuid,
    last_four: String,
    brand: String, // TODO: enum
    #[serde(deserialize_with = "deserialize_card_expiry_date")]
    expiry_date: NaiveDate, // TODO, only month and year
    expired: bool,
    three_d_verified: bool,
    address: Address,
    postcode: Option<String>,
    issuer: Issuer,
    currency: String, // TODO: enum
    confirmed: bool,
    confirmation_attempts: u8,
    auto_topup: String, // TODO: enum
    auto_topup_reason: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    updated_date: DateTime<Utc>,
    associated_bank_type: String, // TODO: enum
    #[serde(with = "chrono::serde::ts_milliseconds")]
    last_used_date: DateTime<Utc>,
    current_topup: Amount, // TODO: Make sure this is an amount
    credit_repayment: bool,
}

/// Credit card issuer information.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
    bin: String,
    name: Option<String>,
    card_type: CardType,
    card_brand: String, // TODO: enum
    country: String,    // TODO: enum
    currency: String,   // TODO: enum
    supported: bool,
    fee: f64,
    postcode_required: bool,
}

/// Card type.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CardType {
    /// Credit card.
    Credit,
    /// Debit card.
    Debit,
}

/// Deserializes the expiry date of the card information structure.
fn deserialize_card_expiry_date<'de, D>(de: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    use chrono::Duration;
    use serde::de::Deserialize;

    /// Naive year-month representation.
    #[derive(Debug, Clone, Copy, Deserialize)]
    struct NaiveYearMonth {
        year: i32,
        month: u32,
    }

    let mut ym = NaiveYearMonth::deserialize(de)?;
    // The expiry date is the last day of the month.
    if ym.month == 12 {
        ym.year += 1;
        ym.month = 1;
    } else {
        ym.month += 1;
    }
    let mut date = NaiveDate::from_ymd(ym.year, ym.month, 1);
    date -= Duration::days(1);

    Ok(date)
}
