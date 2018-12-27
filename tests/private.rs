//! Private API methods tests.

use std::env;

use revolut_customer::{error, Client, Options, OptionsBuilder};

/// Gets generic options for the tests.
fn get_options() -> Options {
    dotenv::dotenv().ok();

    let client_version = env::var("CLIENT_VERSION").unwrap();
    let api_version = env::var("API_VERSION").unwrap();
    let device_id = env::var("DEVICE_ID").unwrap();
    let device_model = env::var("DEVICE_MODEL").unwrap();
    let user_agent = env::var("USER_AGENT").unwrap();

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
