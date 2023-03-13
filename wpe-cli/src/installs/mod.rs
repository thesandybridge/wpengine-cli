use dialoguer::{
    Select,
    Input,
    Confirm,
    theme::ColorfulTheme
};
use clap::ArgMatches;
use anyhow::Result;
use wpe::API;

const ENV: [&str; 3] = ["development", "staging", "production"];

fn get_install_data(api: &API) -> Result<(String, String, String, String)>{
    let install: String = Input::new()
        .with_prompt("Enter a install name")
        .interact()?;

    let accounts_results = api.get_accounts(Some(0))?;
    let accounts = accounts_results["results"].as_array().unwrap();

    let account = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an account")
        .items(&accounts
               .iter()
               .map(|acc| &acc["name"])
               .collect::<Vec<&serde_json::Value>>()
              )
        .interact()?;

    let sites_results = api.get_accounts(Some(0))?;
    let sites = sites_results["results"].as_array().unwrap();

    let site = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an account")
        .items(&sites
               .iter()
               .map(|s| &s["id"])
               .collect::<Vec<&serde_json::Value>>()
              )
        .interact()?;

    let environment = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an environment")
        .items(&ENV)
        .interact()?;

    return Ok((
            install,
            accounts[account]["id"].to_string(),
            sites[site]["id"].to_string(),
            ENV[environment].to_string()
    ));
}

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
    let next = api.get_installs(Some(page_num))?;
    let results = next["results"].as_array().unwrap();

    // Check for headless mode.
    if let Some(true) = headless {
        match sub_n.subcommand() {
            Some(("list", sub)) => {
                if let Some(id) = sub.get_one::<String>("ID") {
                    let install = api.get_install_by_id(id)?;
                    println!("{}", serde_json::to_string_pretty(&install)?);

                } else {
                    println!("{}", serde_json::to_string_pretty(results)?);
                }
            },
            Some(("add", sub)) => {
                let name = sub.get_one::<String>("NAME").unwrap();
                let account_id = sub.get_one::<String>("ACCOUNT").unwrap();
                let site_id = sub.get_one::<String>("SITE").unwrap();
                let env = sub.get_one::<String>("ENV").unwrap();

                let data = wpe::Install {
                    name: name.to_string(),
                    account_id: account_id.to_string(),
                    site_id: site_id.to_string(),
                    environment: env.to_string()
                };

                let add_install = api.add_install(&data)?;

                println!("{}", serde_json::to_string_pretty(&add_install)?);
            },
            Some(("update", sub)) => {
                let install_id = sub.get_one::<String>("ID").unwrap();
                let site_id = sub.get_one::<String>("SITE");
                let env = sub.get_one::<String>("ENV");

                let data = wpe::InstallPatch {
                    site_id: site_id.cloned(),
                    environment: env.cloned()
                };

                let update_site = api.update_install(install_id, &data)?;

                println!("{}", serde_json::to_string_pretty(&update_site)?);
            },
            Some(("delete", sub)) => {
                let id = sub.get_one::<String>("ID").unwrap();

                api.delete_install(id)?;
            },
            _ => {
                println!("{}", serde_json::to_string_pretty(results)?);
            }
        }
    } else {
        // Handle logic for when headless mode is not enabled
        let options = vec!["List All", "Add Install", "Update Install", "Delete Install"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&options)
            .interact()?;

        match selection {
            0 => {
                // Handle logic for listing sites.
                let install_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select an install to view...")
                    .items(&results
                        .iter()
                        .map(|install| &install["name"])
                        .collect::<Vec<&serde_json::Value>>()
                    )
                    .interact()?;

                let item = &results[install_selection]["id"];
                let install = api.get_install_by_id(&item.as_str().unwrap())?;

                println!("Selection: {}", serde_json::to_string_pretty(&install)?);
            },
            1 => {
                // Logic for adding a site.
                println!("Follow the prompts to add a install.");
                let (install, account, site, env) = get_install_data(&api)?;

                let data = wpe::Install {
                    name: install,
                    account_id: account,
                    site_id: site,
                    environment: env

                };

                let add_install= api.add_install(&data)?;

                println!(
                    "Successfully added install: {}",
                    serde_json::to_string_pretty(&add_install)?
                );
            },
            2 => {
                // Logic for updating a site.
                let install_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a site to update.")
                    .items(&results
                        .iter()
                        .map(|i| &i["name"])
                        .collect::<Vec<&serde_json::Value>>()
                    )
                    .interact()?;

                let install = &results[install_selection]["id"].as_str().unwrap().to_string();
                let site_id: String = Input::new()
                    .with_prompt("Enter a site ID.")
                    .allow_empty(true)
                    .interact()?;

                let environment: String = Input::new()
                    .with_prompt("Enter an environment name.")
                    .allow_empty(true)
                    .interact()?;

                let site: Option<String>;
                let env: Option<String>;

                if site_id.is_empty() {
                    site = Some(site_id.clone());
                } else {
                    site = None
                }

                if environment.is_empty() {
                    env = Some(environment)
                } else {
                    env = None
                }

                if site_id.is_empty() {
                    println!("cancelling, no value provided.");

                    if Confirm::new().with_prompt("Does this data look right?").interact()? {

                        // Need to do something better to handle optional values.
                        let data = wpe::InstallPatch {
                            site_id: site,
                            environment: env
                        };

                        let update = api.update_install(install, &data)?;
                        println!(
                            "Successfully update install: {}",
                            serde_json::to_string_pretty(&update)?
                        );

                    } else {
                        // Recursively call init to show prompts again.
                        init(sub_n, api, headless)?;
                    }
                }
            },
            3 => {
                // Logic for deleting a site.
                let install_slection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a site to update.")
                    .items(&results
                        .iter()
                        .map(|i| &i["name"])
                        .collect::<Vec<&serde_json::Value>>()
                    )
                    .interact()?;

                let install = &results[install_slection]["id"].as_str().unwrap().to_string();
                if Confirm::new().with_prompt("Does this data look right?").interact()? {

                    api.delete_install(install)?;
                    println!("Install deleted!");

                } else {
                    println!("Cancelling.");
                }
            },
            _ => println!("An error occured with your selection")
        }
    }
    Ok(())
}
