use colored::Colorize;
use reqwest::blocking::Client;
use serde_json::Value;
use crate::behind::constants::{GITHUB_API_LATEST_RELEASE, USER_AGENT, VERSION};
use crate::behind::errors::VersionCheckError;


pub(crate) fn check_update() -> Result<(), VersionCheckError> {
    if let Some(latest_version) = update_to_latest_version()? {
        log::info!("Latest version: {}", latest_version[0]);
        let version: &String = &latest_version[0];
        let link: &String = &latest_version[1];
        if version > &VERSION.parse::<String>().unwrap() {
            println!("{}\n{}: {} ({})", format!("Your Version: {}", VERSION.bold()).dimmed(), "Update available".yellow(), version.bright_yellow().bold(), link.bright_blue());
        }
    } else {
        log::error!("Error checking latest version");
        println!("{}: {}", "Error checking latest version".bright_red(), "Couldn't check latest version".red());
    }
    Ok(())
}

pub(crate) fn update_to_latest_version() -> Result<Option<Vec<String>>, VersionCheckError> {
    println!("{0}", "Checking latest version...".dimmed());
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .build()?;
    let response = client.get(GITHUB_API_LATEST_RELEASE).send()?;
    let json: Value = serde_json::from_str(&response.text()?)?;
    log::info!("Latest release (JSON Response): {}", json);
    let latest_version = json["name"]
        .as_str()
        .ok_or(VersionCheckError::VersionNotFound)?;
    // get url of latest release
    let latest_release_url = json["html_url"]
        .as_str()
        .ok_or(VersionCheckError::VersionNotFound)?;
    if latest_version > VERSION {
        let result: Vec<String> = Vec::from([latest_version.to_string(), latest_release_url.to_string()]);
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

