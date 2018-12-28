//! Private methods of the client.

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserializer;
use uuid::Uuid;

use crate::amount::Amount;

mod auth;
mod exchange;
mod transactions;
mod user;

/// User information structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User ID.
    #[get = "pub"]
    #[deref]
    id: Uuid,
    /// User creation date.
    #[get = "pub"]
    #[deref]
    #[serde(with = "chrono::serde::ts_milliseconds")]
    created_date: DateTime<Utc>,
    /// Address of the user.
    #[get = "pub"]
    address: Address,
    /// Birth date of the user.
    #[get = "pub"]
    #[deref]
    #[serde(deserialize_with = "deserialize_user_birth_date")]
    birth_date: NaiveDate,
    /// First name of the user.
    #[get = "pub"]
    first_name: String,
    /// Last name of the user.
    #[get = "pub"]
    last_name: String,
    /// Phone of the user.
    #[get = "pub"]
    phone: String, // TODO: properly parse
    /// Email of the user.
    #[get = "pub"]
    email: String, // TODO: struct Email
    /// Wether the email is verified
    #[get = "pub"]
    #[deref]
    email_verified: bool,
    /// State of the user.
    #[get = "pub"]
    state: String, // TODO: enum
    /// Referral code.
    #[get = "pub"]
    referral_code: String,
    /// Unknown.
    #[get = "pub"]
    kyc: String,
    /// Accepted terms and conditions version.
    #[get = "pub"]
    terms_version: String,
    /// Wether the user is under review.
    #[get = "pub"]
    #[deref]
    under_review: bool,
    /// Wether the user risk has been assessed (unknown meaning.)
    #[get = "pub"]
    #[deref]
    risk_assessed: bool,
    /// Locale of the user.
    #[get = "pub"]
    locale: String, // TODO: enum
    /// Unknown "sof" structure.
    #[get = "pub"]
    sof: Sof,
}

/// Structure representing an address.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// City of the address.
    #[get = "pub"]
    city: String,
    /// Country of the address.
    #[get = "pub"]
    country: String, // TODO: enum
    /// Post code of the address.
    #[get = "pub"]
    postcode: String,
    /// Region of the address.
    #[get = "pub"]
    region: String,
    /// Street address, line 1.
    #[get = "pub"]
    street_line_1: String,
    /// Street address, line 2.
    #[get = "pub"]
    street_ine_2: Option<String>,
}

/// Wallet information structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    /// Wallet ID.
    #[get = "pub"]
    #[deref]
    id: Uuid,
    /// Reference of the wallet.
    #[serde(rename = "ref")]
    #[get = "pub"]
    reference: String,
    /// State of the wallet.
    #[get = "pub"]
    state: String,
    /// Base currency of the wallet.
    #[get = "pub"]
    base_currency: String, // TODO: enum
    /// Total topped up since the last reset.
    #[get = "pub"]
    #[deref]
    total_topup: Amount,
    /// Topup reset date.
    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[get = "pub"]
    #[deref]
    topup_reset_date: DateTime<Utc>,
    /// Pockets of the wallet.
    pockets: Box<[Pocket]>,
}

impl Wallet {
    /// Pockets of the wallet.
    pub fn pockets(&self) -> &[Pocket] {
        &self.pockets
    }
}

/// Pocket information structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Pocket {
    /// Pocket ID.
    #[get = "pub"]
    #[deref]
    id: Uuid,
    /// Pocket type.
    #[serde(rename = "type")]
    #[get = "pub"]
    pocket_type: String,
    /// State of the pocket.
    #[get = "pub"]
    state: String,
    /// Currency of the pocket.
    #[get = "pub"]
    currency: String,
    /// Balance of the pocket.
    #[get = "pub"]
    #[deref]
    balance: Amount,
    /// Blocked balance.
    #[get = "pub"]
    #[deref]
    blocked_amount: Amount,
    /// Wether the pocket is closed.
    #[get = "pub"]
    #[deref]
    closed: bool,
    /// Credit limit.
    #[get = "pub"]
    #[deref]
    credit_limit: Amount,
}

/// Unknown `sof` structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Sof {
    /// State of the "sof".
    #[get = "pub"]
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
