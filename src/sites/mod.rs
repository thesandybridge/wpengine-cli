use dialoguer::{
    Select,
    Input,
    Confirm,
    theme::ColorfulTheme
};
use clap::ArgMatches;
use anyhow::Result;
use wpe::API;

/// Provides logic for the sites command.
///
/// # Arguments
///
/// * `sub_n` - &ArgMatches
/// * `command` - API
/// * `headless` - Option<&bool>
pub fn init(sub_n: &ArgMatches, api: API, headless: Option<&bool>) -> Result<()> {
    let page = sub_n.get_one::<String>("PAGE");
    let page_num: i32;

    match page {
        Some(x) => {
            page_num = x.parse::<i32>().unwrap();
        },
        None => {
            page_num = 0;
        }
    }

    // Fetch sites and display results. Will also show paginated results.
    let next = api.get_sites(Some(page_num))?;
    let results = next["results"].as_array().unwrap();

    if let Some(true) = headless {
        let r = serde_json::to_string_pretty(results)?;
        println!("{}", &r);
    } else {
        let options = vec!["List All", "Add site", "Update Site", "Delete Site"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&options)
            .interact()
            .unwrap();

        match selection {
            0 => {
                // Handle selection logic
                let site_slection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a site to view...")
                    .items(&results
                           .iter()
                           .map(|site| &site["name"])
                           .collect::<Vec<&serde_json::Value>>()
                          )
                    .interact()
                    .unwrap();

                let item = &results[site_slection]["id"];
                let site = api.get_site_by_id(&item.as_str().unwrap())?;

                println!("Selection: {}", serde_json::to_string_pretty(&site)?);
            },
            1 => {
                println!("Follow the prompts to add a site.");
                let site_name = Input::new()
                    .with_prompt("Enter a site name")
                    .interact()
                    .unwrap();

                let accounts_results = api.get_accounts(Some(0))?;
                let accounts = accounts_results["results"].as_array().unwrap(); 

                let account = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select an account")
                    .items(&accounts
                           .iter()
                           .map(|acc| &acc["name"])
                           .collect::<Vec<&serde_json::Value>>()
                    )
                    .interact()
                    .unwrap();

                let data = wpe::Site {
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
                let site_slection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a site to update.")
                    .items(&results
                           .iter()
                           .map(|site| &site["name"])
                           .collect::<Vec<&serde_json::Value>>()
                          )
                    .interact()
                    .unwrap();

                let site = &results[site_slection]["id"].as_str().unwrap().to_string();
                let site_name: String = Input::new()
                    .with_prompt("Enter a site name")
                    .allow_empty(true)
                    .interact()
                    .unwrap();

                if site_name.is_empty() {
                    println!("cancelling, no value provided.");

                } else {
                    if Confirm::new().with_prompt("Does this data look right?").interact()? {

                        // Need to do something better to handle optional values.
                        let data = wpe::SitePatch {
                            name: Some(site_name)
                        };

                        let update_site = api.update_site(site, &data)?;
                        println!(
                            "Successfully update site: {}",
                            serde_json::to_string_pretty(&update_site)?
                            );

                    } else {
                        init(sub_n, api, headless)?;
                    }
                }
                
            },
            3 => {},
            _ => println!("An error occured with your selection")
        }


        
    }
    Ok(())
}

