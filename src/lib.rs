use home_config::HomeConfig;
use std::str;
use regex::Regex;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use anyhow::Result;


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
fn set_config(username: String, token: String) -> Result<()> {

    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let data: Data = Data {
        wpengine_user_id: username,
        wpengine_password: token,
        wpengine_api: String::from("https://api.wpengineapi.com/v1")
    };
    config.save_toml(&data).unwrap();

    Ok(())
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
pub fn reset() -> Result<()> {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let file = HomeConfig::path(&config);
    if file.exists() {
        std::fs::remove_file(file)?;
    }

    Ok(())
}

/// Handles the cli for the authentication.
pub fn set_auth() -> Result<()> {
    println!("Authenticate with wpengine.");

    let username: String = Input::new()
        .with_prompt("Enter API Username")
        .interact()
        .unwrap();

    let token: String = Input::new()
        .with_prompt("Enter API Password")
        .interact()
        .unwrap();

    set_config(username, token)?;

    Ok(())
}

/// Handles user authentication.
/// If the user is not authenticated redirect them to authentication.
pub fn init() -> Result<()> {
    if !authenticated() {
        set_auth()?;
    }
    Ok(())
}

pub struct Commands {
    client: reqwest::blocking::Client,
    config: Data,
}

impl Commands {
    /// Creates a new reqwest client instance
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::new();
        let config = get_config(); 
        Self { client, config}
    }
    
    /// Status endpoint to check API health.
    pub fn status(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(&format!("{}/status", &self.config.wpengine_api))
            .basic_auth(
                &self.config.wpengine_user_id, 
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Get all sites from wpengine. Pass an optional page number to show more results.
    pub fn get_sites(&self, page: Option<i32>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(&format!("{}/sites?offset={}", &self.config.wpengine_api, page.unwrap_or(0) * 100))
            .basic_auth(
                &self.config.wpengine_user_id, 
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Get a single site by its ID from the wpengine API
    pub fn get_site_by_id(&self, id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(&format!("{}/sites/{}", &self.config.wpengine_api,  id))
            .basic_auth(
                &self.config.wpengine_user_id, 
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// List all accounts, optional page offset.
    pub fn get_accounts(&self, page: Option<i32>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self 
            .client
            .get(&format!("{}/accounts?offset={}", &self.config.wpengine_api, page.unwrap_or(0) * 100))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// List account by ID.
    pub fn get_account_by_id(&self, id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(&format!("{}/accounts/{}", &self.config.wpengine_api,  id))
            .basic_auth(
                &self.config.wpengine_user_id, 
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }
}

