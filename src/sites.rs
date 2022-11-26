use crate::api::get_config;

pub fn get_sites() -> Result<(), Box<dyn std::error::Error>> { 
    let config = get_config();
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://api.wpengineapi.com/v1/sites")
        .basic_auth(config.wpengine_user_id, Some(config.wpengine_password))
        .send()?
        .json::<serde_json::Value>()?;

    for i in res["results"].as_array().unwrap() {
        println!("{}", i["name"]);
    }
    Ok(())
}