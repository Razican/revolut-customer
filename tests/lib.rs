//! Base test module.

use std::env;

use revolut_customer::OptionsBuilder;

/// Tests that the options are derived properly.
#[test]
fn ut_options_builder() {
    dotenv::dotenv().ok();

    let client_version = env::var("CLIENT_VERSION").unwrap();
    let api_version = env::var("API_VERSION").unwrap();
    let device_id = env::var("DEVICE_ID").unwrap();
    let device_model = env::var("DEVICE_MODEL").unwrap();
    let user_agent = env::var("USER_AGENT").unwrap();

    assert!(OptionsBuilder::default().build().is_err());
    assert!(OptionsBuilder::default()
        .client_version(client_version.clone())
        .build()
        .is_err());
    assert!(OptionsBuilder::default()
        .api_version(api_version.clone())
        .build()
        .is_err());
    assert!(OptionsBuilder::default()
        .device_id(device_id.clone())
        .build()
        .is_err());
    assert!(OptionsBuilder::default()
        .device_model(device_model.clone())
        .build()
        .is_err());
    assert!(OptionsBuilder::default()
        .device_model(user_agent.clone())
        .build()
        .is_err());

    assert!(OptionsBuilder::default()
        .client_version(client_version)
        .api_version(api_version)
        .device_id(device_id)
        .device_model(device_model)
        .user_agent(user_agent)
        .build()
        .is_ok());
}
