use dialoguer::{
    Select,
    Input,
    Confirm,
    theme::ColorfulTheme
};
use clap::ArgMatches;
use anyhow::Result;
use wpe::*;

/// Provides logic for the sites command.
///
/// # Arguments
///
/// * `sub_n` - &ArgMatches
/// * `command` - API
/// * `headless` - Option<&bool>
pub fn init(sub_n: &ArgMatches, api: API, headless: Option<&bool>) -> Result<()> {
    let page = sub_n.get_one::<String>("PAGE");
    let page_num: u8;

    match page {
        Some(x) => {
            page_num = x.parse::<u8>().unwrap();
        },
        None => {
            page_num = 0;
        }
    }

    // Fetch sites and display results. Will also show paginated results.
    let next = api.get_ssh_keys(Some(page_num))?;
    let results = next["results"].as_array().unwrap();

    // Check for headless mode.
    if let Some(true) = headless {
        match sub_n.subcommand() {
            Some(("list", _)) => {
                println!("{}", serde_json::to_string_pretty(results)?);
            },
            Some(("add", sub)) => {
                let pub_key = sub.get_one::<String>("KEY").unwrap();

                let data = SSHKey {
                    public_key: pub_key.to_string(),
                };

                let add_ssh_key= api.add_ssh_key(&data)?;

                println!("{}", serde_json::to_string_pretty(&add_ssh_key)?);
            },
            Some(("delete", sub)) => {
                let id = sub.get_one::<String>("ID").unwrap();

                api.delete_ssh_key(id)?;
            },
            _ => {
                println!("{}", serde_json::to_string_pretty(results)?);
            }
        }
    } else {
        // Handle logic for when headless mode is not enabled
        let options = vec!["List All", "Add SSH Key", "Delete SSH Key"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&options)
            .interact()?;

        match selection {
            0 => {
                // Handle logic for listing SSH Keys.
                let ssh_keys = &results;
                println!("Selection: {}", serde_json::to_string_pretty(&ssh_keys)?);
            },
            1 => {
                // Logic for adding an SSH Key to the authorized user's account.
                // TODO: fix bug preventing pasting public key.
                println!("Follow the prompts to add an SSH Key.");
                let pub_key: String = Input::new()
                    .with_prompt("Enter your public RSA key")
                    .interact()?;

                let data = SSHKey {
                    public_key: pub_key
                };

                let add_ssh_key= api.add_ssh_key(&data)?;

                println!("Successfully added SSH Key: {}", serde_json::to_string_pretty(&add_ssh_key)?);

            },
            2 => {
                // Logic for deleting an install from an SSH Key.
                // TODO: fix bugs.
                let ssh_keys = &results;
                let ssh_selection = get_selections!(ssh_keys, "Select an SSH Key", "comment");
                let key = ssh_keys[ssh_selection]["uuid"].as_str().unwrap();


                if Confirm::new().with_prompt("Does this data look right?").interact()? {

                    api.delete_ssh_key(&key)?;
                    println!("SSH Key deleted!");

                } else {
                    println!("Cancelling.");
                }
            },
            _ => println!("An error occured with your selection")
        }
    }

    return Ok(());
}
