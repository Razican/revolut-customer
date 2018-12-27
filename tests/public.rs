//! Public API methods tests.

use std::env;

use revolut_customer::{Options, OptionsBuilder};

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
