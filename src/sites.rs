use crate::api::get_config;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Sites {
    previous: String,
    next: String,
    count: i32,
    results: serde_json::Value,
}

pub fn get_sites() -> Result<(), Box<dyn std::error::Error>> { 
    let config = get_config();
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://api.wpengineapi.com/v1/sites")
        .basic_auth(config.wpengine_user_id, Some(config.wpengine_password))
        .send()?
        .json::<Sites>()?;

    println!("{:#?}", &res.results);
    Ok(())
}