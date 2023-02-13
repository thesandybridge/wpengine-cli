use clap::{arg, Command};
use anyhow::Result;

mod sites;
mod accounts;

/// Setup the CLI and build the commands.
fn cli() -> Command {
    Command::new("wpe")
        .about("WPEngine CLI")
        .arg(arg!(-H --headless "Enables headless mode").required(false))
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
                .arg(arg!(<PAGE> "The page number").required(false))
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
    let command = wpe::API::new();
    let headless = matches.get_one::<bool>("headless");

    // Handle logic for each command.
    match matches.subcommand() {
        // Handles [sites] command logic.
        Some(("sites", sub_n)) => {
            sites::handle_sites(sub_n, command, headless)?;
        },
        // Handles [site] command logic.
        Some(("site", sub_m)) => {
            let id = sub_m.get_one::<String>("ID").unwrap();
            let res = command.get_site_by_id(id).unwrap();
            println!("{}", serde_json::to_string_pretty(&res)?);
        },
        Some(("accounts", sub_n)) => {
            accounts::handle_accounts(sub_n, command, headless)?; 
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
                _ => println!("Error with auth command.")
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
