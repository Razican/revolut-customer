//! User methods of the API.

use chrono::{DateTime, NaiveDate, Utc};
use failure::{Error, ResultExt};
use getset::Getters;
use lazy_static::lazy_static;
use reqwest::{header::ACCEPT, StatusCode, Url};
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::{Address, User, Wallet};
use crate::{amount::Amount, error, Client, ErrResponse, BASE_API_URL};

/// User client methods.
///
/// They require the client to have loaded the authentication mechanisms.
impl Client {
    /// Gets user information.
    ///
    /// Make sure the client has the authentication information.
    pub fn current_user(&self) -> Result<(User, Wallet), Error> {
        if let (&Some(ref user_id), &Some(ref access_token)) = (&self.user_id, &self.access_token) {
            /// Response to the `current_user()` method.
            #[derive(Debug, Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct CurrentUserResponse {
                /// User information.
                user: User,
                /// Wallet information.
                wallet: Wallet,
            }

            lazy_static! {
                /// URL of the endpoint.
                static ref URL: Url = BASE_API_URL.join("user/current").unwrap();
            }

            let request_builder = self.client.get(URL.clone());

            let mut response = self
                .set_headers(request_builder)
                .header(ACCEPT, "application/json")
                .basic_auth(&user_id, Some(access_token))
                .send()
                .context(error::Api::RequestFailure)?;

            if response.status().is_success() {
                let res_structure: CurrentUserResponse =
                    response.json().context(error::Api::ParseResponse)?;
                Ok((res_structure.user, res_structure.wallet))
            } else if response.status() == StatusCode::UNAUTHORIZED {
                Err(error::Api::Unauthorized.into())
            } else {
                Err(error::Api::Other {
                    status_code: response.status(),
                }
                .into())
            }
        } else {
            Err(error::Api::NotLoggedIn.into())
        }
    }

    /// Gets user's wallet information.
    ///
    /// Make sure the client has the authentication information.
    pub fn current_user_wallet(&self) -> Result<Wallet, Error> {
        if let (&Some(ref user_id), &Some(ref access_token)) = (&self.user_id, &self.access_token) {
            lazy_static! {
                /// URL of the endpoint.
                static ref URL: Url = BASE_API_URL.join("user/current/wallet").unwrap();
            }

            let request_builder = self.client.get(URL.clone());

            let mut response = self
                .set_headers(request_builder)
                .header(ACCEPT, "application/json")
                .basic_auth(user_id, Some(&access_token))
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
        } else {
            Err(error::Api::NotLoggedIn.into())
        }
    }

    /// Gets user's cards information.
    ///
    /// Make sure the client has the authentication information.
    pub fn current_user_cards(&self) -> Result<Vec<Card>, Error> {
        if let (&Some(ref user_id), &Some(ref access_token)) = (&self.user_id, &self.access_token) {
            lazy_static! {
                /// URL of the endpoint.
                static ref URL: Url = BASE_API_URL.join("user/current/cards").unwrap();
            }

            let request_builder = self.client.get(URL.clone());

            let mut response = self
                .set_headers(request_builder)
                .header(ACCEPT, "application/json")
                .basic_auth(user_id, Some(&access_token))
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
        } else {
            Err(error::Api::NotLoggedIn.into())
        }
    }

    /// Changes the address of the current user.
    ///
    /// This method will set the address of the user to the given one. **Note**: Make sure the
    /// client has the authentication information.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # dotenv::dotenv().ok();
    /// let mut client = Client::default();
    ///
    /// let user_id = env::var("TEST_USER_ID")
    ///     .expect("TEST_USER_ID environment variable not set");
    /// let access_token = env::var("TEST_ACCESS_TOKEN")
    ///     .expect("TEST_ACCESS_TOKEN environment variable not set");
    ///
    /// client
    ///    .set_auth(user_id, access_token)
    ///    .expect("invalid user ID");
    ///
    /// # let (user, _wallet) = client.current_user().unwrap();
    /// # let previous_address = user.address();
    ///
    /// let new_address = Address::new(
    ///     "NewCity",
    ///     "FR",
    ///     "39325",
    ///     "NewRegion",
    ///     "Street 1, 6",
    ///     None);
    /// client.change_current_user_address(&new_address).unwrap();
    ///
    /// let (new_user, _wallet) = client.current_user().unwrap();
    /// assert_eq!(new_user.address(), &new_address);
    ///
    /// # client
    /// #   .change_current_user_address(previous_address)
    /// #   .unwrap();
    /// # let (final_user, _wallet) = client.current_user().unwrap();
    /// # assert_eq!(final_user.address(), previous_address);
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
    pub fn change_current_user_address(&self, address: &Address) -> Result<(), Error> {
        if let (&Some(ref user_id), &Some(ref access_token)) = (&self.user_id, &self.access_token) {
            /// Data structure to send to the API.
            #[derive(Debug, Serialize)]
            struct SentData<'d> {
                address: &'d Address,
            }

            lazy_static! {
                /// URL of the endpoint.
                static ref URL: Url = BASE_API_URL.join("user/current").unwrap();
            }

            let request_builder = self.client.patch(URL.clone());

            let mut response = self
                .set_headers(request_builder)
                .header(ACCEPT, "application/json")
                .basic_auth(user_id, Some(&access_token))
                .json(&SentData { address })
                .send()
                .context(error::Api::RequestFailure)?;

            if response.status().is_success() {
                Ok(())
            } else if response.status() == StatusCode::UNAUTHORIZED {
                Err(error::Api::Unauthorized.into())
            } else if response.status() == StatusCode::BAD_REQUEST {
                let err_response: ErrResponse =
                    response.json().context(error::Api::ParseResponse)?;
                Err(error::Api::BadRequest {
                    code: err_response.code,
                    message: err_response.message,
                }
                .into())
            } else {
                Err(error::Api::Other {
                    status_code: response.status(),
                }
                .into())
            }
        } else {
            Err(error::Api::NotLoggedIn.into())
        }
    }
}

/// Credit card representation.
#[derive(Debug, Clone, PartialEq, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// Card ID.
    #[get]
    #[deref]
    id: Uuid,
    /// Owner's user ID.
    #[get = "pub"]
    #[deref]
    owner_id: Uuid,
    /// Last four digits of the card.
    #[get = "pub"]
    last_four: String,
    /// Brand of the card.
    #[get = "pub"]
    brand: String, // TODO: enum
    /// Expiry date of the card.
    #[serde(deserialize_with = "deserialize_card_expiry_date")]
    #[get = "pub"]
    #[deref]
    expiry_date: NaiveDate, // TODO, only month and year
    /// Wether the card is expired.
    #[get = "pub"]
    #[deref]
    expired: bool,
    /// Unknown.
    #[get = "pub"]
    #[deref]
    three_d_verified: bool,
    /// Address associated with the card.
    #[get = "pub"]
    address: Address,
    /// Post code associated with the card.
    #[get = "pub"]
    postcode: Option<String>,
    /// Issuer of the card.
    #[get = "pub"]
    issuer: Issuer,
    /// Currency of the card.
    #[get = "pub"]
    currency: String, // TODO: enum
    /// Wether the card is confirmed.
    #[get = "pub"]
    #[deref]
    confirmed: bool,
    /// Number of attempts performed to confirm the card.
    #[get = "pub"]
    #[deref]
    confirmation_attempts: u8,
    /// Auto-topup status.
    #[get = "pub"]
    auto_topup: String, // TODO: enum
    /// Reason for the auto-topup status.
    #[get = "pub"]
    auto_topup_reason: String,
    /// Card creation date.
    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[get = "pub"]
    #[deref]
    created_date: DateTime<Utc>,
    /// Card update date.
    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[get = "pub"]
    #[deref]
    updated_date: DateTime<Utc>,
    /// Type of the associated bank.
    #[get = "pub"]
    associated_bank_type: String, // TODO: enum
    /// Last time used.
    #[serde(with = "chrono::serde::ts_milliseconds")]
    #[get = "pub"]
    #[deref]
    last_used_date: DateTime<Utc>,
    /// Current topup amount.
    #[get = "pub"]
    #[deref]
    current_topup: Amount, // TODO: Make sure this is an amount
    /// Credit repayment.
    #[get = "pub"]
    #[deref]
    credit_repayment: bool,
}

/// Credit card issuer information.
#[derive(Debug, Clone, PartialEq, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
    /// Bank Identification Number
    #[get = "pub"]
    bin: String,
    /// Name of the issuer.
    #[get = "pub"]
    name: Option<String>,
    /// Type of card.
    #[get = "pub"]
    #[deref]
    card_type: CardType,
    /// Brand of the card.
    #[get = "pub"]
    card_brand: String, // TODO: enum
    /// Country of the card.
    #[get = "pub"]
    country: String, // TODO: enum
    /// Currency of the card.
    #[get = "pub"]
    currency: String, // TODO: enum
    /// Wether the card is supported.
    #[get = "pub"]
    #[deref]
    supported: bool,
    /// Fee for using the card.
    #[get = "pub"]
    #[deref]
    fee: f64,
    /// Wether the postcode is required for operation.
    #[get = "pub"]
    #[deref]
    postcode_required: bool,
}

/// Card type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
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
