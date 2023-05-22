use colored::Colorize;
use reqwest::Client;
use serde_json::Value;
use crate::behind::constants::{GITHUB_API_LATEST_RELEASE, USER_AGENT, VERSION};
use crate::behind::errors::VersionCheckError;
use semver::Version;

pub(crate) async fn check_update() -> Result<(), VersionCheckError> {
    let result: Option<Vec<String>> = update_to_latest_version().await?;
    if let Some(latest_version) = result {
        log::info!("Latest version: {}", latest_version[0]);
        let version: &String = &latest_version[0];
        let link: &String = &latest_version[1];
        if version > &VERSION.parse::<String>().unwrap() {
            println!("{}\n{}: {} ({})", format!("Your Version: {}", VERSION.bold()).dimmed(), "Update available".yellow(), version.bright_yellow().bold(), link.bright_blue());
        }
    } else {
        println!("{}", "You are using the latest version.".dimmed());
    }
    Ok(())
}


pub(crate) async fn update_to_latest_version() -> Result<Option<Vec<String>>, VersionCheckError> {
    log::info!("Checking latest version...");
    println!("{0}", "Checking latest version...".dimmed());
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .build()?;
    let response = client.get(GITHUB_API_LATEST_RELEASE).send().await?;
    let json: Value = serde_json::from_str(&response.text().await?)?;
    // get version of latest release
    let latest_version_str = json[0]["name"]
        .as_str()
        .ok_or(VersionCheckError::VersionNotFound)?;
    // parse versions using semver
    let latest_version = Version::parse(latest_version_str)?;
    let current_version = Version::parse(VERSION)?;
    // get url of latest release
    let latest_release_url = json[0]["html_url"]
        .as_str()
        .ok_or(VersionCheckError::VersionNotFound)?;
    if latest_version > current_version {
        let result: Vec<String> = Vec::from([latest_version.to_string(), latest_release_url.to_string()]);
        return Ok(Some(result))
    }
    Ok(None)
}
