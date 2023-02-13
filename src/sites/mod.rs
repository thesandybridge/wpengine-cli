use dialoguer::{Select, theme::ColorfulTheme};
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
pub fn handle_sites(sub_n: &ArgMatches, command: API, headless: Option<&bool>) -> Result<()> {
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
    let next = command.get_sites(Some(page_num)).unwrap();
    let results = next["results"].as_array().unwrap();

    if let Some(true) = headless {
        let r = serde_json::to_string_pretty(results)?;
        println!("{}", &r);
    } else {

        // Handle selection logic
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a site to view...")
            .items(&results
                   .iter()
                   .map(|site| &site["name"])
                   .collect::<Vec<&serde_json::Value>>()
                  )
            .interact()
            .unwrap();

        let item = &results[selection]["id"];
        let site = command.get_site_by_id(
            &item.as_str().unwrap()
            ).unwrap();

        println!("Selection: {}", serde_json::to_string_pretty(&site)?);
    }
    Ok(())
}

