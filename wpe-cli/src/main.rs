use clap::{arg, Command};
use anyhow::Result;
mod sites;
mod installs;
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
                .subcommand(
                    Command::new("list")
                        .about("List sites.")
                        .arg(arg!(<ID> "Account ID").required(false))
                )
                .subcommand(
                    Command::new("add")
                        .about("Add a site using headless mode")
                        .arg(arg!(<NAME> "Site name").required(true))
                        .arg(arg!(<ID> "Account ID").required(true))
                )
                .subcommand(
                    Command::new("update")
                        .about("Update the name of a site.")
                        .arg(arg!(<NAME> "Site name").required(true))
                        .arg(arg!(<ID> "Site ID").required(true))
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a site.")
                        .arg(arg!(<ID> "Site ID").required(true))
                )
        )
        .subcommand(
            Command::new("installs")
                .about("Display list of installs as selection.")
                .arg(arg!(<PAGE> "The page number").required(false))
                .after_help("Selecting one will fetch the site and display more options.")
                .subcommand(
                    Command::new("list")
                        .about("List sites.")
                        .arg(arg!(<ID> "Account ID").required(false))
                )
                .subcommand(
                    Command::new("add")
                        .about("Add a site using headless mode")
                        .arg(arg!(<NAME> "Site name").required(true))
                        .arg(arg!(<ACCOUNT> "Account ID").required(true))
                        .arg(arg!(<SITE> "Site ID").required(true))
                        .arg(arg!(<ENV> "Environment").required(true))
                )
                .subcommand(
                    Command::new("update")
                        .about("Update the name of a site.")
                        .arg(arg!(<ID> "Install ID").required(true))
                        .arg(arg!(<SITE> "Site ID").required(false))
                        .arg(arg!(<ENV> "Environment").required(false))
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete an install.")
                        .arg(arg!(<ID> "Install ID").required(true))
                )
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
        .subcommand(
            Command::new("swagger")
                .about("Get API swagger")
        )
}

fn main() -> Result<()> {
    // Check if authentication exists, else handle authentication.
    wpe::init()?;

    // Handle missing cursor when pressing ctrl-c to quit.
    ctrlc::set_handler(move || {
        let term = console::Term::stdout();
        let _ = term.show_cursor();
    })?;

    // Initiate CLI commands.
    let matches = cli().get_matches();
    let command = wpe::API::new();
    let headless = matches.get_one::<bool>("headless");

    // Handle logic for each command.
    match matches.subcommand() {
        Some(("sites", sub_n)) => {
            // Initialize [sites] command logic.
            sites::init(sub_n, command, headless)?;
        },
        Some(("installs", sub_n)) => {
            installs::init(sub_n, command, headless)?;
        }
        Some(("accounts", sub_n)) => {
            // Initialize [accounts] command logic.
            accounts::init(sub_n, command, headless)?;
        },
        Some(("account", sub_m)) => {
            // This will eventually be moved to the [accounts] command.
            let id = sub_m.get_one::<String>("ID").unwrap();
            let res = command.get_account_by_id(id)?;
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
                _ => println!("Error with auth command.")
            }
        },
        Some(("status", _)) => {
            // This endpoint will report the system status
            // and any outages that might be occurring.
            let status = command.status()?;
            println!("{}", serde_json::to_string_pretty(&status)?)
        },
        Some(("swagger", _)) => {
            // This endpoint will report the system status
            // and any outages that might be occurring.
            let swagger = command.swagger()?;
            println!("{}", serde_json::to_string_pretty(&swagger)?)
        }
        _ => println!("Invalid command. Please use <help> to a see full list of commands.")
    }
    Ok(())
}
