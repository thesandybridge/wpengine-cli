use home_config::HomeConfig;
use std::io;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default, Debug)]
struct Data {
    wpengine_user_id: String,
    wpengine_password: String
}

fn read_config() {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig");
    let data: Data = config.toml().unwrap();
    println!("Config: {} {}", data.wpengine_user_id, data.wpengine_password);
}


fn set_config(username: String, token: String) {
    // Stores wpengine API username and password in config file.
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig");
    let data = Data {
        wpengine_user_id: username,
        wpengine_password: token
    };
    println!("{:?}", data);
    config.save_toml(&data).unwrap();
}


pub fn authenticate() {
    println!("Authenticate with wpengine.");

    let mut username = String::new();
    let mut token = String::new();

    println!("Enter API Username:");

    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Enter API Password:");

    io::stdin()
        .read_line(&mut token)
        .expect("Failed to read line");

    set_config(username, token);
    read_config();

}