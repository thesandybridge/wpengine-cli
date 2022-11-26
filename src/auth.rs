use home_config::HomeConfig;
use std::io;
use std::str;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default, Debug)]
struct Data {
    wpengine_user_id: String,
    wpengine_password: String
}

/// Stores wpengine API username and password in config file.
/// $HOME/.config/wpe/wpeconfig.toml
fn set_config(username: String, token: String) {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let data: Data = Data {
        wpengine_user_id: username,
        wpengine_password: token
    };
    println!("{:?}", data);
    config.save_toml(&data).unwrap();
}

/// Handles user authentication.
pub fn authenticate() {
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