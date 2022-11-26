use api::Data;
mod api;

struct SiteCli {
    command: String
}

struct Site {
    client: reqwest::blocking::Client,
}

impl Site {
    /// Creates a new reqwest client instance
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::new();
        Self { client }
    }

    /// Get all sites from wpengine API
    pub fn get_sites(config: &Data) -> Result<(), Box<dyn std::error::Error>> {
        let res = Self::new().client.get("https://api.wpengineapi.com/v1/sites")
            .basic_auth(&config.wpengine_user_id, Some(&config.wpengine_password))
            .send()?
            .json::<serde_json::Value>()?;

        for i in res["results"].as_array().unwrap() {
            println!("{}", i["id"]);
        }
        Ok(())
    }

    /// Get a single site by its ID from the wpengine API
    pub fn get_site_by_id(config: &Data, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let res = Self::new().client.get(&format!("https://api.wpengineapi.com/v1/sites/{}", id))
            .basic_auth(&config.wpengine_user_id, Some(&config.wpengine_password))
            .send()?
            .json::<serde_json::Value>()?;

        println!("{}", res["name"]);
        Ok(())
    }
}
fn main() {
    api::auth();
    
    let command = std::env::args().nth(1).expect("no command given");
    let args = SiteCli {
        command
    };

    // Switch to listen for commands and execute proper functions.
    match args.command.as_str() {
        "sites" => {
            let config = api::get_config();
            Site::get_sites(&config).unwrap();
        },
        "site" => {
            let config = api::get_config();
            let id = std::env::args().nth(2).expect("no id given");
            Site::get_site_by_id(&config, &id).unwrap();
        },
        "auth" => {
            api::set_auth();
        },
        _ => println!("Invalid command"),
    }
}
