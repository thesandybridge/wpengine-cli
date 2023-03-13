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

    let sites_results = api.get_sites(Some(1))?;
    let sites = sites_results["results"].as_array().unwrap();

    let site = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a site")
        .items(&sites
            .iter()
            .map(|s| &s["name"])
            .collect::<Vec<&serde_json::Value>>()
        )
        .interact()?;

    let install: String = Input::new()
        .with_prompt("Enter an install name")
        .interact()?;

    let environment = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an environment")
        .items(&ENV)
        .interact()?;

    println!("{}", &accounts[account]["id"]);

    return Ok((
        install,
        accounts[account]["id"].as_str().unwrap().to_string(),
        sites[site]["id"].as_str().unwrap().to_string(),
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
    let next = api.get_sites(Some(page_num))?;
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
                let site_id = sub.get_one::<String>("SITE").unwrap();
                let env = sub.get_one::<String>("ENV").unwrap();

                let data = wpe::InstallPatch {
                    site_id: site_id.to_string(),
                    environment: env.to_string()
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
                    .with_prompt("Select a site to view...")
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
                let site_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a site to update")
                    .items(&results
                        .iter()
                        .map(|i| &i["name"])
                        .collect::<Vec<&serde_json::Value>>()
                    )
                    .interact()?;

                let site_id = results[site_selection]["id"].as_str().unwrap();

                let selected_site = api.get_site_by_id(site_id)?;
                let installs = selected_site["installs"].as_array().unwrap();

                let install_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select an Install")
                    .items(&installs
                        .iter()
                        .map(|i| &i["name"])
                        .collect::<Vec<&serde_json::Value>>()
                    )
                    .interact()?;

                let install = &installs[install_selection];

                let environment = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select an environment")
                    .items(&ENV)
                    .interact()?;

                let env = ENV[environment];

                let data = wpe::InstallPatch {
                    site_id: site_id.to_string(),
                    environment: env.to_string()
                };

                println!("{}", serde_json::to_string_pretty(&data)?);

                if Confirm::new().with_prompt("Does this data look right?").interact()? {

                    let update = api.update_install(install["id"].as_str().unwrap(), &data)?;
                    println!(
                        "Successfully updated install: {}",
                        serde_json::to_string_pretty(&update)?
                    );

                } else {
                    // Recursively call init to show prompts again.
                    init(sub_n, api, headless)?;
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

    return Ok(());
}

