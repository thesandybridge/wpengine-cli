use dialoguer::{Select, theme::ColorfulTheme};
use clap::ArgMatches;
use anyhow::Result;
use wpe::API;

/// Handles logic for the accounts command.
///
/// # Arguments
///
/// * `sub_n` - &ArgMatches
/// * `command` - API
/// * `headless` - Option<&bool>
pub fn init(sub_n: &ArgMatches, command: API, headless: Option<&bool>) -> Result<()> {
    let page = sub_n.get_one::<String>("PAGE");
    let page_num: u8;
    // Check for provided page argument, else provide default.
    match page {
        Some(x) => {
            page_num = x.parse::<u8>().unwrap();
        },
        None => {
            page_num = 0;
        }

    }
    // Fetch sites and display results. Will also show paginated results.
    let next = command.get_accounts(Some(page_num)).unwrap();
    let results = next["results"].as_array().unwrap();

    if let Some(true) = headless {
        let r = serde_json::to_string_pretty(results)?;
        println!("{}", &r);
    } else {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a site to view...")
            .items(&results
                   .iter()
                   .map(|account| &account["name"])
                   .collect::<Vec<&serde_json::Value>>()
                  )
            .interact()
            .unwrap();

        let item = &results[selection]["id"];
        let account = command.get_account_by_id(
            &item.as_str().unwrap()
            ).unwrap();

        println!("Selection: {}", serde_json::to_string_pretty(&account)?);
    }

    Ok(())
}
