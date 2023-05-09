use reqwest::blocking::Client;
use std::io::{stdin, stdout, Write};
use colored::Colorize;

pub(crate) fn selection_3() {
    print!("\nURL (http or https): ");
    stdout().flush().unwrap();
    let mut url_input = String::new();
    stdin().read_line(&mut url_input).unwrap();

    // Validate the URL
    log::info!("Validating URL: {}", url_input.trim());
    let parsed_url = url::Url::parse(&url_input);
    match parsed_url {
        Ok(_) => {
            print!("Mask-Domain: ");
            stdout().flush().unwrap();
            let mut mask_domain = String::new();
            stdin().read_line(&mut mask_domain).unwrap();

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
                        let short_url = client.get(isgd_url).send().unwrap().text().unwrap();
                        let final_url = format!("https://{}@{}", mask_domain.trim(), short_url.trim().replace("https://", ""));

                        println!("End-URL: {}\n", final_url.blue());
                    } else {
                        log::error!("Invalid Mask Domain: the host part of the mask domain must contain a period: {}", mask_domain.trim());
                        println!("{0}{1}", "Invalid Mask Domain: the host part of the mask domain must contain a period: ".red(), mask_domain.bright_red());
                    }
                },
                Err(e) => {
                    log::error!("Invalid Mask Domain: {}", e.to_string());
                    println!("{0}{1}", "Invalid Mask Domain: ".red(), e.to_string().bright_red())
                },
            }
        },
        Err(e) => {
            log::error!("Invalid URL: {}", e.to_string());
            println!("{0}{1}", "Invalid URL: ".red(), e.to_string().bright_red())
        },
    }
}
