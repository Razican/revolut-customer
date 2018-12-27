//! Base test module.

use std::env;

use revolut_customer::OptionsBuilder;

/// Tests that the options are derived properly.
#[test]
fn ut_options_builder() {
    dotenv::dotenv().ok();

    let client_version = env::var("CLIENT_VERSION").unwrap_or("5.12.1".to_owned());
    let api_version = env::var("API_VERSION").unwrap_or("1".to_owned());
    let device_id = env::var("DEVICE_ID").unwrap_or("SOME-DEVICE-ID".to_owned());
    let device_model = env::var("DEVICE_MODEL").unwrap_or("iPhone8,1".to_owned());
    let user_agent = env::var("USER_AGENT")
        .unwrap_or("Revolut/com.revolut.revolut (iPhone; iOS 11.1)".to_owned());

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
        .client_version(client_version.clone())
        .api_version(api_version.clone())
        .device_id(device_id.clone())
        .device_model(device_model.clone())
        .user_agent(user_agent)
        .build()
        .is_ok());

    assert!(OptionsBuilder::default()
        .client_version(client_version)
        .api_version(api_version)
        .device_id(device_id)
        .device_model(device_model)
        .build()
        .is_ok());
}
