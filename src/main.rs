use clap::{arg, Command};
use anyhow::Result;

struct Site {
    client: reqwest::blocking::Client,
    config: wpe::Data,
}

impl Site {
    /// Creates a new reqwest client instance
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::new();
        let config = wpe::get_config(); 
        Self { client, config}
    }

    /// Get all sites from wpengine API
    pub fn get_sites(&self, page: i8) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(&format!("{}/sites?offset={}00", &self.config.wpengine_api, page))
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
}

fn cli() -> Command {
    Command::new("wpe")
        .about("WPEngine CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("sites")
                .about("Fetch all sites from your wpengine account")
                .subcommand(
                    Command::new("list")
                        .about("Get list of sites")
                        .arg(arg!(<PAGE> "The page number").required(false))
                ) 
                .subcommand_required(true)
        )
        .subcommand(
            Command::new("site")
                .about("Fetch a site by its ID")
                .arg(arg!(<ID> "The site ID"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("auth")
                .about("Authenticate with WP Engine API")
                .subcommand(
                    Command::new("login")
                    .about("Login to WP Engine API")
                )
                .subcommand(
                    Command::new("reset")
                        .about("Reset authentication")
                )
                .subcommand_required(true)
        )
}

fn main() -> Result<()> {
    // Check if authentication exists, else handle authentication.
    wpe::init()?;

    let matches = cli().get_matches();
    let site = Site::new();

    // Switch to listen for commands and execute proper functions.
    match matches.subcommand() {
        Some(("sites", sub_m)) => {
            match sub_m.subcommand() {
                Some(("list", sub_n)) => {
                    let page = sub_n.get_one::<String>("PAGE");
                    match page {
                        Some(x) => {
                            let n = x.parse::<i8>().unwrap();
                            let next = site
                                .get_sites(n)
                                .unwrap();
                            let results = next["results"].as_array().unwrap();
                            println!("Showing {} results...", results.len());
                            for i in results {
                                println!("{} = {}", i["name"], i["id"]);
                            }
                        },
                        None => {
                            let next = site.get_sites(0).unwrap();
                            let results = next["results"].as_array().unwrap();
                            println!("Showing {} results...", results.len());
                            for i in results {
                                println!("{} = {}", i["name"], i["id"]);
                            }
                        }
                    }
                },
                _ => {}
            }
        },
        Some(("site", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            let res = site.get_site_by_id(id).unwrap();
            println!("{}", serde_json::to_string_pretty(&res)?);
        },
        Some(("auth", sub_m)) => {
            match sub_m.subcommand() {
                Some(("login", _)) => {
                    wpe::set_auth()?;
                },
                Some(("reset", _)) => {
                    wpe::reset()?;
                },
                _ => {}
            }
        },
        _ => println!("Invalid command"),
    }
    Ok(())
}
