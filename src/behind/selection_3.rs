use reqwest::Client;
use std::io::{stdin, stdout, Write};
use colored::Colorize;
use crate::behind::cli::error_msg;
use zphisher::errors::TerminalError;
use crate::error_msg;

pub async fn selection_3() -> Result<(), TerminalError> {
    print!("\nURL (http or https): ");
    stdout().flush()?;
    let mut url_input = String::new();
    stdin().read_line(&mut url_input)?;

    // Validate the URL
    log::info!("Validating URL: {}", url_input.trim());
    let parsed_url = url::Url::parse(&url_input);
    match parsed_url {
        Ok(_) => {
            print!("Mask-Domain: ");
            stdout().flush()?;
            let mut mask_domain = String::new();
            stdin().read_line(&mut mask_domain)?;

            // Validate the mask domain
            let mask_url = format!("http://{}", mask_domain.trim());  // Temporarily append a scheme for validation
            match url::Url::parse(&mask_url) {
                    Ok(url) => {
                    if url.host_str().map_or(false, |h| h.contains('.')) {
                        println!("{}", "loading...".dimmed().bold());

                        let client = Client::new();
                        let isgd_url = format!(
                            "https://is.gd/create.php?format=simple&url={}",
                            url_input.trim()
                        );
                        log::info!("Sending request to {}", isgd_url);
                        let short_url = {
                            match client.get(&isgd_url).send().await {
                                Ok(response) => {
                                    if let Ok(short_url) = response.text().await {
                                        short_url
                                    } else {
                                        log::error!("Failed to get response from is.gd");
                                        error_msg!("Failed to get response from is.gd");
                                        return Ok(());
                                    }
                                },
                                Err(e) => {
                                    log::error!("Failed to send request to is.gd: {}", e.to_string());
                                    error_msg!(&format!("Failed to send request to is.gd: {}", e));
                                    return Ok(());
                                }
                            }
                        };
                        let final_url = format!("https://{}@{}", mask_domain.trim(), short_url.trim().replace("https://", ""));

                        println!("End-URL: {}\n", final_url.blue());
                        Ok(())
                    } else {
                        log::error!("Invalid Mask Domain: the host part of the mask domain must contain a period: {}", mask_domain.trim());
                        error_msg(&format!("Invalid Mask Domain: the host part of the mask domain must contain a period: {}", mask_domain.trim()));
                        Ok(())
                    }
                },
                Err(e) => {
                    log::error!("Invalid Mask Domain: {}", e.to_string());
                    error_msg(&format!("Invalid Mask Domain: {}", e));
                    Ok(())
                },
            }
        },
        Err(e) => {
            log::error!("Invalid URL: {}", e.to_string());
            error_msg(&format!("Invalid URL: {}", e));
            Ok(())
        },
    }
}
