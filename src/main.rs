use clap::{arg, Command};
use anyhow::Result;
use dialoguer::{Select, theme::ColorfulTheme};

/// Setup the CLI and build the commands.
fn cli() -> Command {
    Command::new("wpe")
        .about("WPEngine CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("sites")
                .about("Display list of sites as selection.")
                .arg(arg!(<PAGE> "The page number").required(false))
                .after_help("Selecting one will fetch the site and display more options.")
        )
        .subcommand(
            Command::new("site")
                .about("Fetch a site by its ID.")
                .arg(arg!(<ID> "The site ID"))
                .arg_required_else_help(true)
                .after_help("This command is a headless alternative to the selection dialogue.")
        )
        .subcommand(
            Command::new("accounts")
                .about("Fetch all sites from your wpengine account")
                .subcommand(
                    Command::new("list")
                        .about("Get list of accounts")
                        .arg(arg!(<PAGE> "The page number").required(false))
                ) 
                .subcommand_required(true)
        )
        .subcommand(
            Command::new("account")
                .about("Fetch an account by its ID")
                .arg(arg!(<ID> "The account ID"))
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
        .subcommand(
            Command::new("status")
                .about("Get API status")
        )
}

fn main() -> Result<()> {
    // Check if authentication exists, else handle authentication.
    wpe::init()?;

    let matches = cli().get_matches();
    let command = wpe::Commands::new();

    // Handle logic for each command.
    match matches.subcommand() {
        // Handles [sites] command logic.
        Some(("sites", sub_n)) => {
            let page = sub_n.get_one::<String>("PAGE");
            let page_num: i32;
            // Check for provided page argument, else provide default.
            match page {
                Some(x) => {
                    page_num = x.parse::<i32>().unwrap();
                },
                None => {
                    page_num = 0; 
                }

            }
            // Fetch sites and display results. Will also show paginated results.
            let next = command.get_sites(Some(page_num)).unwrap();
            let results = next["results"].as_array().unwrap();

            // Handle selection logic
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a site to view...")
                .items(&results
                    .iter()
                    .map(|x| &x["name"])
                    .collect::<Vec<&serde_json::Value>>()
                )
                .interact()
                .unwrap();

            let item = &results[selection]["id"];
            let site = command.get_site_by_id(
                &item.as_str().unwrap()
            ).unwrap();

            println!("Selection: {}", serde_json::to_string_pretty(&site)?);
        },
        // Handles [site] command logic.
        Some(("site", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            let res = command.get_site_by_id(id).unwrap();
            println!("{}", serde_json::to_string_pretty(&res)?);
        },
        Some(("accounts", sub_m)) => {
            match sub_m.subcommand() {
                Some(("list", sub_n)) => {
                    let page = sub_n.get_one::<String>("PAGE");
                    let page_num: i32;
                    // Check for provided page argument, else provide default.
                    match page {
                        Some(x) => {
                            page_num = x.parse::<i32>().unwrap();
                        },
                        None => {
                            page_num = 0; 
                        }

                    }
                    // Fetch sites and display results. Will also show paginated results.
                    let next = command.get_accounts(Some(page_num)).unwrap();
                    let results = next["results"].as_array();
                    match results {
                        Some(result) => {
                            println!("Showing {} results...", result.len());
                            for i in result {
                                println!("{} = {}", i["name"], i["id"]);
                            }
                        },
                        None => {
                            println!("Nothing found")
                        }
                    }
                    
                },
                _ => println!("Invalid command. Please use <help> to see full list of commands.")
            }
        },
        // Handles [site] command logic.
        Some(("account", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            let res = command.get_account_by_id(id).unwrap();
            println!("{}", serde_json::to_string_pretty(&res)?);
        },
        // Handles [auth] command logic.
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
        // This endpoint will report the system status and any outages that might be occurring.
        Some(("status", _)) => {
            let status = command.status().unwrap();
            println!("{}", serde_json::to_string_pretty(&status)?)
        }
        _ => println!("Invalid command. Please use <help> to see full list of commands.")
    }
    Ok(())
}
