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
    let next = api.get_sites(Some(page_num))?;
    let results = next["results"].as_array().unwrap();

    // Check for headless mode.
    if let Some(true) = headless {
        match sub_n.subcommand() {
            Some(("list", sub)) => {
                if let Some(id) = sub.get_one::<String>("ID") {
                    let site = api.get_site_by_id(&id.as_str())?;
                    println!("{}", serde_json::to_string_pretty(&site)?);

                } else {
                    println!("{}", serde_json::to_string_pretty(results)?);
                }
            },
            Some(("add", sub)) => {
                let name = sub.get_one::<String>("NAME").unwrap();
                let id = sub.get_one::<String>("ID").unwrap();

                let data = Site {
                    name: name.to_string(),
                    account_id: id.to_string()
                };

                let add_site = api.add_site(&data)?;

                println!("{}", serde_json::to_string_pretty(&add_site)?);
            },
            Some(("update", sub)) => {
                let name = sub.get_one::<String>("NAME");
                let id = sub.get_one::<String>("ID").unwrap();

                let data = SitePatch {
                    name: name.cloned()
                };

                let update_site = api.update_site(id.as_str(), &data)?;

                println!("{}", serde_json::to_string_pretty(&update_site)?);
            },
            Some(("delete", sub)) => {
                let id = sub.get_one::<String>("ID").unwrap();

                api.delete_site(&id.as_str())?;
            },
            _ => {
                println!("{}", serde_json::to_string_pretty(results)?);
            }
        }
    } else {
        // Handle logic for when headless mode is not enabled
        let options: [&str; 4] = ["List All", "Add site", "Update Site", "Delete Site"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&options)
            .interact()?;

        match selection {
            0 => {
                // Handle logic for listing sites.
                let site_selections = get_selections!(results, "Select a site to view", "name");

                let item = &results[site_selections]["id"];
                let site = api.get_site_by_id(&item.as_str().unwrap())?;

                println!("Selection: {}", serde_json::to_string_pretty(&site)?);
            },
            1 => {
                // Logic for adding a site.
                println!("Follow the prompts to add a site.");
                let site_name = Input::new()
                    .with_prompt("Enter a site name")
                    .interact()?;

                let accounts_results = api.get_accounts(Some(0))?;
                let accounts = accounts_results["results"].as_array().unwrap();

                let account = get_selections!(accounts, "Select an account", "name");

                let data = Site {
                    name: site_name,
                    account_id: accounts[account]["id"].as_str().unwrap().to_string()
                };

                let add_site = api.add_site(&data)?;

                println!(
                    "Successfully added site: {}",
                    serde_json::to_string_pretty(&add_site)?
                );
            },
            2 => {
                // Logic for updating a site.
                let site_selections = get_selections!(results, "Select a site to update", "name");

                let site = &results[site_selections]["id"].as_str().unwrap().to_string();
                let site_name: String = Input::new()
                    .with_prompt("Enter a site name")
                    .allow_empty(true)
                    .interact()?;

                if site_name.is_empty() {
                    println!("cancelling, no value provided.");

                } else {
                    if Confirm::new().with_prompt("Does this data look right?").interact()? {

                        // Need to do something better to handle optional values.
                        let data = SitePatch {
                            name: Some(site_name)
                        };

                        let update_site = api.update_site(site.as_str(), &data)?;
                        println!(
                            "Successfully update site: {}",
                            serde_json::to_string_pretty(&update_site)?
                            );

                    } else {
                        // Recursively call init to show prompts again.
                        init(sub_n, api, headless)?;
                    }
                }

            },
            3 => {
                // Logic for deleting a site.
                let site_slection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a site to update.")
                    .items(&results
                           .iter()
                           .map(|site| &site["name"])
                           .collect::<Vec<&serde_json::Value>>()
                          )
                    .interact()?;

                let site = &results[site_slection]["id"].as_str().unwrap().to_string();

                if Confirm::new().with_prompt("Are you sure?").interact()? {

                        api.delete_site(site)?;
                        println!("Site deleted!");

                    } else {
                        println!("Cancelling.");
                    }
            },
            _ => println!("An error occured with your selection")
        }

    }
    Ok(())
}

