use home_config::HomeConfig;
use std::str;
use regex::Regex;
use serde::{Deserialize, Serialize};
use dialoguer::Input;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Data {
    pub wpengine_user_id: String,
    pub wpengine_password: String,
    pub wpengine_api: String
}

/// This function will prompt the user for their WPEngine API credentials
/**
 - Stores wpengine API username and password in config file.
    - $HOME/.config/wpe/wpeconfig.toml
*/
fn set_config(username: String, token: String) {

    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let data: Data = Data {
        wpengine_user_id: username,
        wpengine_password: token,
        wpengine_api: String::from("https://api.wpengineapi.com/v1")
    };
    config.save_toml(&data).unwrap();
}

/// Check if username and password are stored in config file.
fn authenticated() -> bool {
    
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let file = HomeConfig::path(&config);

    // Check if config file exists.
    if file.exists() {
        let toml = config.toml::<Data>().unwrap();
        let re = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
        
        // check if username matches UUID format
        // need a better check here, should consider pinging the API for a 200.
        if re.is_match(&toml.wpengine_user_id) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

/// Get username and password from config file.
pub fn get_config() -> Data {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let toml = config.toml::<Data>().unwrap();
    toml
}

/// Reset the config file. This should be used if you change your API token or for debugging.

pub fn reset() {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let file = HomeConfig::path(&config);
    if file.exists() {
        std::fs::remove_file(file).unwrap();
    }
}

/// Handles the cli for the authentication.
pub fn set_auth() {
    println!("Authenticate with wpengine.");

    let username: String = Input::new()
    .with_prompt("Enter API Username")
    .interact()
    .unwrap();

    let token: String = Input::new()
    .with_prompt("Enter API Password")
    .interact()
    .unwrap();
    
    set_config(username, token);
}

/// Handles user authentication.
/// If the user is not authenticated redirect them to authentication.
pub fn init() {
    if !authenticated() {
        set_auth();
    }

