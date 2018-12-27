//! Private API methods tests.

use std::env;

use revolut_customer::{error, private::AccessToken, Client, Options, OptionsBuilder};

/// Gets generic options for the tests.
fn get_options() -> Options {
    dotenv::dotenv().ok();

    let client_version = env::var("CLIENT_VERSION").unwrap_or("5.12.1".to_owned());
    let api_version = env::var("API_VERSION").unwrap_or("1".to_owned());
    let device_id = env::var("DEVICE_ID").unwrap_or("SOME-DEVICE-ID".to_owned());
    let device_model = env::var("DEVICE_MODEL").unwrap_or("iPhone8,1".to_owned());
    let user_agent = env::var("USER_AGENT")
        .unwrap_or("Revolut/com.revolut.revolut (iPhone; iOS 11.1)".to_owned());

    OptionsBuilder::default()
        .client_version(client_version)
        .api_version(api_version)
        .device_id(device_id)
        .device_model(device_model)
        .user_agent(user_agent)
        .build()
        .unwrap()
}

/// Tests the user sign in.
#[test]
fn it_sign_in() {
    let client = Client::new(get_options());

    let phone = env::var("TEST_PHONE").unwrap_or("+1555555555".to_owned());
    let password = env::var("TEST_PASSWORD").unwrap_or("9999".to_owned());

    let response = client.sign_in(&phone, &password);
    assert!(
        response.is_ok()
            || (phone == "+1555555555"
                && password == "9999"
                && response
                    .err()
                    .unwrap()
                    .downcast_ref::<error::Api>()
                    .unwrap()
                    == &error::Api::Unauthorized)
    );
}

/// Tests the user sign in confirmation.
#[ignore]
#[test]
fn it_confirm_sign_in() {
    let client = Client::new(get_options());

    let phone = env::var("TEST_PHONE").expect("no test phone found");
    let code = env::var("TEST_CONFIRM_CODE").expect("no test confirmation code found");

    let response = client.confirm_sign_in(&phone, &code);
    assert!(response.is_ok());
}

/// Tests the user retrieval.
#[test]
fn it_current_user() {
    let client = Client::new(get_options());

    let client_id = env::var("TEST_CLIENT_ID").unwrap_or("some-client-id".to_owned());
    let access_token = env::var("TEST_ACCESS_TOKEN")
        .unwrap_or("some-access-token".to_owned())
        .into();

    let response = client.current_user(&client_id, &access_token);

    assert!(
        response.is_ok()
            || (client_id == "some-client-id"
                && access_token == "some-access-token".into()
                && response
                    .err()
                    .unwrap()
                    .downcast_ref::<error::Api>()
                    .unwrap()
                    == &error::Api::Unauthorized)
    );
}

/// Tests the user wallet retrieval.
#[test]
fn it_current_user_wallet() {
    let client = Client::new(get_options());

    let client_id = env::var("TEST_CLIENT_ID").unwrap_or("some-client-id".to_owned());
    let access_token = env::var("TEST_ACCESS_TOKEN")
        .unwrap_or("some-access-token".to_owned())
        .into();

    let response = client.current_user_wallet(&client_id, &access_token);

    assert!(
        response.is_ok()
            || (client_id == "some-client-id"
                && access_token == AccessToken::from("some-access-token")
                && response
                    .err()
                    .unwrap()
                    .downcast_ref::<error::Api>()
                    .unwrap()
                    == &error::Api::Unauthorized)
    );
}

/// Tests the user cards retrieval.
#[test]
fn it_current_user_cards() {
    let client = Client::new(get_options());

    let client_id = env::var("TEST_CLIENT_ID").unwrap_or("some-client-id".to_owned());
    let access_token = env::var("TEST_ACCESS_TOKEN")
        .unwrap_or("some-access-token".to_owned())
        .into();

    let response = client.current_user_cards(&client_id, &access_token);

    assert!(
        response.is_ok()
            || (client_id == "some-client-id"
                && access_token == "some-access-token".into()
                && response
                    .err()
                    .unwrap()
                    .downcast_ref::<error::Api>()
                    .unwrap()
                    == &error::Api::Unauthorized)
    );
}
