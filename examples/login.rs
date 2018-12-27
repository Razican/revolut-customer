//! Example code of the library to get a client ID and an access token.
#![forbid(anonymous_parameters)]
#![warn(clippy::pedantic)]
#![deny(
    clippy::all,
    variant_size_differences,
    unused_results,
    unused_qualifications,
    unused_import_braces,
    unsafe_code,
    trivial_numeric_casts,
    trivial_casts,
    missing_docs,
    unused_extern_crates,
    missing_debug_implementations,
    missing_copy_implementations
)]

use std::{env, io::stdin};

use failure::{Error, ResultExt};
use revolut_customer::{Client, Options, OptionsBuilder};

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter_causes() {
            println!("caused by: {}", e);
        }

        ::std::process::exit(1);
    }
}

/// Execution of the program
fn run() -> Result<(), Error> {
    println!("Welcome to Revolut client login example.");

    println!("Phone: ");
    let mut phone = String::new();
    let _ = stdin()
        .read_line(&mut phone)
        .context("unable to read the phone")?;
    println!();

    println!("Password/PIN: ");
    let mut password = String::new();
    let _ = stdin()
        .read_line(&mut password)
        .context("unable to read the password")?;
    println!();

    println!("Trying to sign in phone {}", phone);
    let client = Client::new(get_options());
    client
        .sign_in(phone.trim(), password.trim())
        .context("error signing in")?;
    println!();

    println!("Log in successful, you should receive an SMS with the code");
    println!("Code: ");
    let mut code = String::new();
    let _ = stdin()
        .read_line(&mut code)
        .context("unable to read the code")?;
    println!();

    println!(
        "{:?}",
        client
            .confirm_sign_in(phone.trim(), code.trim())
            .context("error confirming the login")?
    );

    Ok(())
}

/// Gets generic options for the tests.
fn get_options() -> Options {
    let _ = dotenv::dotenv().ok();

    let client_version = env::var("CLIENT_VERSION").unwrap_or_else(|_| "5.12.1".to_owned());
    let api_version = env::var("API_VERSION").unwrap_or_else(|_| "1".to_owned());
    let device_id = env::var("DEVICE_ID").unwrap_or_else(|_| "SOME-DEVICE-ID".to_owned());
    let device_model = env::var("DEVICE_MODEL").unwrap_or_else(|_| "iPhone8,1".to_owned());
    let user_agent = env::var("USER_AGENT")
        .unwrap_or_else(|_| "Revolut/com.revolut.revolut (iPhone; iOS 11.1)".to_owned());

    OptionsBuilder::default()
        .client_version(client_version)
        .api_version(api_version)
        .device_id(device_id)
        .device_model(device_model)
        .user_agent(user_agent)
        .build()
        .unwrap()
}
