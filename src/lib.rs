use home_config::HomeConfig;
use std::str;
use regex::Regex;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use anyhow::Result;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
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
    let data: Config = Config {
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
        let toml = config.toml::<Config>().unwrap();
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
pub fn get_config() -> Config {
    let config = HomeConfig::with_config_dir("wpe", "wpeconfig.toml");
    let toml = config.toml::<Config>().unwrap();
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

#[derive(Serialize, Deserialize, Debug)]
enum Environment {
    Production,
    Staging,
    Development
}

pub struct API {
    client: reqwest::blocking::Client,
    config: Config,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Site {
    pub name: String,
    pub account_id: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SitePatch {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Install {
    name: String,
    account_id: String,
    site_id: String,
    environment: Environment
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallPatch {
    site_id: Option<String>,
    environment: Option<Environment>
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct User {
    account_id: String,
    first_name: String,
    last_name: String,
    email: String,
    roles: String,
    install_ids: Vec<String>
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct UserPatch {
    roles: Option<String>,
    install_ids: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AccountUser {
    user: User
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AccountUserPatch {
    user: UserPatch
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Domain {
    name: String,
    primary: bool
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DomainPatch {
    primary: Option<bool>,
    redirect_to: Option<String>
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SSHKey {
    public_key: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Cache {
    r#type: String
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Backup {
    description: String,
    notification_emails: Vec<String>
}

impl API {
    /// Creates a new reqwest client instance
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::new();
        let config = get_config(); 
        Self { client, config}
    }
    
    /// Status endpoint to check API health.
    pub fn status(&self) -> Result<serde_json::Value, anyhow::Error> {
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

    pub fn swagger(&self) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .get(&format!("{}/swagger", &self.config.wpengine_api))
            .basic_auth(
                &self.config.wpengine_user_id, 
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Get all sites from wpengine. Pass an optional page number to show more results.
    pub fn get_sites(&self, page: Option<i32>) -> Result<serde_json::Value, anyhow::Error> {
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
    pub fn get_site_by_id(&self, id: &str) -> Result<serde_json::Value, anyhow::Error> {
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

    /// Try to add a site.
    pub fn add_site(&self, body: &Site) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .post(&format!("{}/sites", &self.config.wpengine_api))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(body)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn update_site(&self, id: &str, body: &SitePatch) 
        -> Result<serde_json::Value, anyhow::Error> {

        let res = self
            .client
            .patch(&format!("{}/sites/{}", &self.config.wpengine_api, id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(body)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Try to delete a specific install.
    pub fn delete_site(&self, id: &str ) -> Result<serde_json::Value, anyhow::Error>{
        let res = self
            .client
            .delete(&format!("{}/sites/{}", &self.config.wpengine_api, id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Get all installs from wpengine. Pass an optional page number to show more results.
    pub fn get_installs(&self, page: Option<i32>) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .get(&format!("{}/installs?offset={}", &self.config.wpengine_api, page.unwrap_or(0) * 100))
            .basic_auth(
                &self.config.wpengine_user_id, 
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Get a single install by its ID from the wpengine API
    pub fn get_install_by_id(&self, id: &str) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .get(&format!("{}/installs/{}", &self.config.wpengine_api,  id))
            .basic_auth(
                &self.config.wpengine_user_id, 
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Try to add an install instance.
    pub fn add_install(&self, body: &Install) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .post(&format!("{}/installs", &self.config.wpengine_api))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(body)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn update_install(&self, install_id: &str, body: &InstallPatch) 
        -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .patch(&format!("{}/installs/{}", &self.config.wpengine_api, install_id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(body)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn purge_cache(&self, id: &str, body: String) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .post(&format!("{}/installs/{}/purge_cache", &self.config.wpengine_api, id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(&Cache {
                r#type: body
            })
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn backup(&self, id: &str, backup: &Backup) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .post(&format!("{}/installs/{}/backups", &self.config.wpengine_api, id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(backup)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn get_backup(&self, install_id: &str, backup_id: &str) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .get(&format!("{}/installs/{}/backups/{}", &self.config.wpengine_api, install_id, backup_id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Try to delete a specific install.
    pub fn delete_install(&self, id: &str ) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .delete(&format!("{}/installs/{}", &self.config.wpengine_api, id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// List all accounts, optional page offset.
    pub fn get_accounts(&self, page: Option<i32>) -> Result<serde_json::Value, anyhow::Error> {
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

    /// Get the currently authenticated user's account details.
    pub fn get_user(&self) -> Result<serde_json::Value, anyhow::Error> {
        let res = self 
            .client
            .get(&format!("{}/user", &self.config.wpengine_api))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }
    /// List account by ID.
    pub fn get_account_by_id(&self, id: &str) -> Result<serde_json::Value, anyhow::Error> {
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

    /// Add a user to a specific account.
    pub fn add_user(&self, id: &str, user: &AccountUser) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .post(&format!("{}/accounts/{}/account_users", &self.config.wpengine_api, id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(user)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn get_user_by_id(&self, account_id: &str, user_id: &str) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .get(&format!("{}/accounts/{}/account_users/{}", &self.config.wpengine_api, account_id, user_id))
            .basic_auth(
                &self.config.wpengine_user_id, 
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn update_user(&self, account_id: &str, user_id: &str, body: &AccountUserPatch) 
        -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .patch(&format!("{}/accounts/{}/account_users/{}", &self.config.wpengine_api, account_id, user_id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(body)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Try to delete a user from an account.
    pub fn delete_user(&self, account_id: &str, user_id: &str ) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .delete(&format!("{}/accounts/{}/account_users/{}", &self.config.wpengine_api, account_id, user_id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Get a list of ssh keys for authorized user.
    pub fn get_ssh_keys(&self, page: Option<i32>) -> Result<serde_json::Value, anyhow::Error> {
        let res = self 
            .client
            .get(&format!("{}/ssh_keys?offset={}", &self.config.wpengine_api, page.unwrap_or(0) * 100))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Add an ssh key to the authorized users account.
    pub fn add_ssh_key(&self, ssh_key: &SSHKey) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .post(&format!("{}/ssh_keys", &self.config.wpengine_api))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(ssh_key)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Delete an ssh key from the authorized users account.
    pub fn delete_ssh_key(&self, id: &str) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .delete(&format!("{}/ssh_keys/{}", &self.config.wpengine_api, id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    /// Get domains from an install
    pub fn get_domains(&self, id: &str,  page: Option<i32>) -> Result<serde_json::Value, anyhow::Error> {
        let res = self 
            .client
            .get(&format!("{}/installs/{}/domains?offset={}", &self.config.wpengine_api, id, page.unwrap_or(0) * 100))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn get_domain_by_id(&self, install_id: &str, domain_id: &str) -> Result<serde_json::Value, anyhow::Error> {
        let res = self 
            .client
            .get(&format!("{}/installs/{}/domains/{}", &self.config.wpengine_api, install_id, domain_id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn add_domain(&self, id: &str, domain: &Domain) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .post(&format!("{}/installs/{}/domains", &self.config.wpengine_api, id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(domain)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn update_domain(&self, install_id: &str, domain_id: &str, data: &DomainPatch) 
        -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .patch(&format!("{}/installs/{}/domains/{}", &self.config.wpengine_api, install_id, domain_id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .json(data)
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }

    pub fn delete_domain(&self, install_id: &str, domain_id: &str) -> Result<serde_json::Value, anyhow::Error> {
        let res = self
            .client
            .delete(&format!("{}/installs/{}/domains/{}", &self.config.wpengine_api, install_id, domain_id))
            .basic_auth(
                &self.config.wpengine_user_id,
                Some(&self.config.wpengine_password)
            )
            .send()?
            .json::<serde_json::Value>()?;

        Ok(res)
    }
}

