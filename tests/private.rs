//! Private API methods tests.

use std::env;

use revolut_customer::{private::Address, ApiError, Client};

/// Tests the user sign in.
#[test]
fn it_sign_in() {
    dotenv::dotenv().ok();
    let client = Client::default();

    let phone = env::var("TEST_PHONE").unwrap_or("+1555555555".to_owned());
    let password = env::var("TEST_PASSWORD").unwrap_or("9999".to_owned());

    let response = client.sign_in(&phone, &password);
    assert!(
        response.is_ok()
            || (phone == "+1555555555"
                && password == "9999"
                && response.err().unwrap().downcast_ref::<ApiError>().unwrap()
                    == &ApiError::Unauthorized)
    );
}

/// Tests the user sign in confirmation.
#[ignore]
#[test]
fn it_confirm_sign_in() {
    dotenv::dotenv().ok();
    let mut client = Client::default();

    let phone = env::var("TEST_PHONE").expect("no TEST_PHONE provided");
    let code = env::var("TEST_CONFIRM_CODE").expect("no TEST_CONFIRM_CODE provided");

    let response = client.confirm_sign_in(&phone, &code);
    assert!(response.is_ok());
}

/// Tests the user retrieval.
#[test]
fn it_current_user() {
    let mut client = Client::default();

    let user_id = env::var("TEST_USER_ID").expect("TEST_USER_ID environment variable not set");
    let access_token =
        env::var("TEST_ACCESS_TOKEN").expect("TEST_ACCESS_TOKEN environment variable not set");

    client
        .set_auth(user_id, access_token)
        .expect("invalid user ID");
    let response = client.current_user();

    assert!(response.is_ok());
}

/// Tests the user wallet retrieval.
#[test]
fn it_current_user_wallet() {
    dotenv::dotenv().ok();
    let mut client = Client::default();

    let user_id = env::var("TEST_USER_ID").expect("TEST_USER_ID environment variable not set");
    let access_token =
        env::var("TEST_ACCESS_TOKEN").expect("TEST_ACCESS_TOKEN environment variable not set");

    client
        .set_auth(user_id, access_token)
        .expect("invalid user ID");
    let response = client.current_user_wallet();

    assert!(response.is_ok());
}

/// Tests the user cards retrieval.
#[test]
fn it_current_user_cards() {
    dotenv::dotenv().ok();
    let mut client = Client::default();

    let user_id = env::var("TEST_USER_ID").expect("TEST_USER_ID environment variable not set");
    let access_token =
        env::var("TEST_ACCESS_TOKEN").expect("TEST_ACCESS_TOKEN environment variable not set");

    client
        .set_auth(user_id, access_token)
        .expect("invalid user ID");

    let response = client.current_user_cards();

    assert!(response.is_ok());
}

/// Tests the change of the user address.
///
/// It will return the address to the original one after the test.
#[test]
fn it_change_current_user_address() {
    dotenv::dotenv().ok();
    let mut client = Client::default();

    let user_id = env::var("TEST_USER_ID").expect("TEST_USER_ID environment variable not set");
    let access_token =
        env::var("TEST_ACCESS_TOKEN").expect("TEST_ACCESS_TOKEN environment variable not set");

    client
        .set_auth(user_id, access_token)
        .expect("invalid user ID");

    let (user, _wallet) = client.current_user().unwrap();
    let previous_address = user.address();

    let new_address = Address::new("NewCity", "FR", "39325", "NewRegion", "Street 1, 6", None);
    client.change_current_user_address(&new_address).unwrap();

    let (new_user, _wallet) = client.current_user().unwrap();
    assert_eq!(new_user.address(), &new_address);

    client
        .change_current_user_address(previous_address)
        .unwrap();
    let (final_user, _wallet) = client.current_user().unwrap();
    assert_eq!(final_user.address(), previous_address);
}
