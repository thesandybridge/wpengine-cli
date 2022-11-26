use crate::api::get_config;

pub fn get_sites() -> Result<(), Box<dyn std::error::Error>> { 
    let config = get_config();
    let client = reqwest::blocking::Client::new();
    let response_body = client.get("https://api.wpengineapi.com/v1/sites")
        .basic_auth(config.wpengine_user_id, Some(config.wpengine_password))
        .send()?
        .text()?;

    let res = serde_json::from_str(&response_body)?;
    println!("{:#?}", &res);
    Ok(())
}