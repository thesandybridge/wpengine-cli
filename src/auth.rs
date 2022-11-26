use home_config::HomeConfig;
use std::io;
use std::str;
use regex::Regex;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default, Debug)]
struct Data {
    wpengine_user_id: String,
    wpengine_password: String,
    wpengine_api: String
}

/// Stores wpengine API username and password in config file.
/// $HOME/.config/wpe/wpeconfig.toml
fn set_config(username: String, token: String) {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let data: Data = Data {
        wpengine_user_id: username,
        wpengine_password: token,
        wpengine_api: String::from("https://api.wpengineapi.com/v1/")
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
fn get_config() -> Data {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let toml = config.toml::<Data>().unwrap();
    toml
}

/// Handles user authentication.
pub fn authenticate() {

    if !authenticated() {
        println!("Authenticate with wpengine.");

        let mut username = String::new();
        let mut token = String::new();
    
        println!("Enter API Username:");
    
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");
        
        let trimmed_user = username.trim();
    
        println!("Enter API Password:");
    
        io::stdin()
            .read_line(&mut token)
            .expect("Failed to read line");
        
        let trimmed_token = token.trim();
        
        set_config(trimmed_user.to_string(), trimmed_token.to_string());
    }


}