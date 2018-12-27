//! Private methods of the client.

use std::{borrow::Borrow, fmt};

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserializer;
use uuid::Uuid;

use crate::amount::Amount;

mod auth;
mod exchange;
mod transactions;
mod user;

/// API Access token.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct AccessToken {
    /// Actual token string.
    token: String,
}

impl fmt::Display for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl From<String> for AccessToken {
    fn from(token_str: String) -> Self {
        Self { token: token_str }
    }
}

impl From<&str> for AccessToken {
    fn from(token_str: &str) -> Self {
        Self {
            token: token_str.to_owned(),
        }
    }
}

/// User information structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String, // TODO: use Uuid
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_date: DateTime<Utc>,
    address: Address,
    #[serde(deserialize_with = "deserialize_user_birth_date")]
    birth_date: NaiveDate,
    first_name: String,
    last_name: String,
    phone: String, // TODO: properly parse
    email: String, // TODO: email type
    email_verified: bool,
    state: String, // TODO: enum
    referral_code: String,
    kyc: String,
    terms_version: String,
    under_review: bool,
    risk_assessed: bool,
    locale: String, // TODO: enum
    sof: Sof,
}

/// Structure representing an address.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    city: String,
    country: String, // TODO: enum
    postcode: String,
    region: String,
    street_line_1: String,
    street_ine_2: Option<String>,
}

/// Wallet information structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    id: Uuid,
    #[serde(rename = "ref")]
    reference: String,
    state: String,
    base_currency: String, // TODO: Uuid
    total_topup: Amount,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    topup_reset_date: DateTime<Utc>,
    pockets: Box<[Pocket]>,
}

/// Pocket information structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pocket {
    id: Uuid,
    #[serde(rename = "type")]
    pocket_type: String,
    state: String,
    currency: String,
    balance: Amount,
    blocked_amount: Amount,
    closed: bool,
    credit_limit: Amount,
}

/// Unknown `sof` structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sof {
    state: String,
}

/// Deserializes the birth date of the user information structure.
fn deserialize_user_birth_date<'de, D>(de: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Deserialize;

    let (year, month, day) = <(i32, u32, u32)>::deserialize(de)?;
    Ok(NaiveDate::from_ymd(year, month, day))
}
