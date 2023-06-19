use std::time::Instant;
use std::collections::HashSet;
use reqwest::Client;
use serde_json::{Value, to_string_pretty};
use std::process::Command;
use std::io;
use std::io::Write;
use std::net::IpAddr;
use colored::*;
use crate::behind::cli::{error_msg, success_msg};
#[cfg(target_os = "windows")] use std::env;
use zphisher::errors::TerminalError;

fn is_ip_reachable(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}



pub async fn selection_1() -> Result<(), TerminalError> {
    let mut ip_inp = String::new();
    print!("\nEnter Target IP: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut ip_inp)?;
    ip_inp = ip_inp.trim().to_string();
    io::stdout().flush()?;

    let mut port_inp = String::new();
    print!("How many Ports should be scanned?(1 - 65535): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut port_inp)?; // Changed from ip_inp to port_inp
    port_inp = port_inp.trim().to_string();
    io::stdout().flush()?;

    let port_inp: u32 = match port_inp.parse::<u32>() {
        Ok(num) => {
            if num > 0 && num < 65536u32 {
                num
            } else {
                log::error!("Port input is not between 1 and 65535: {}", port_inp);
                error_msg("Port input is not between 1 and 65535");
                return Ok(());
            }
        },
        Err(_) => {
            log::error!("Port input is not a number: {}", port_inp);
            error_msg("Port input is not a number");
            return Ok(());
        }
    };


    log::info!("IP address entered: {}", ip_inp);
    if !is_ip_reachable(&ip_inp) {
        log::error!("IP address not reachable: {}", ip_inp);
        error_msg("IP address not reachable");
        return Ok(());
    }
    println!("{}", "=" .repeat(50).bright_blue());
    println!("{}: {}\n{}: {}", "Target IP".purple(), ip_inp.bright_purple(), "Ports to scan".purple(), port_inp.to_string().bright_purple());
    println!("{}", "loading...".dimmed().bold());

    let start_t = Instant::now();

    println!("{}", "LOG: Fetching IP geolocation information...".dimmed());
    // Log message
    log::info!("Fetching IP geolocation information for {}", ip_inp);
    let client = Client::new();
    let response = client.get(&format!("http://ip-api.com/json/{}", ip_inp)).send().await?;

    if response.status().is_success() {
        let rg: Value = {
            match response.json().await {
                Ok(json) => json,
                Err(e) => {
                    log::error!("Failed to get response from ip-api.com: {}", e);
                    error_msg(&format!("Failed to get response from ip-api.com: {}", e));
                    return Ok(());
                }
            }
        };

        log::info!("IP geolocation information JSON: {}", {
        match to_string_pretty(&rg) {
            Ok(json) => json,
            Err(e) => {
                log::error!("Failed to convert IP geolocation information to JSON: {}", e);
                error_msg(&format!("Failed to convert IP geolocation information to JSON: {}", e));
                return Ok(());
            }
        }
    });
        println!("{}", "LOG: Running Nmap scan... (This could take a while.)".dimmed()); // Log message
        let nmap_result = match run_nmap_scan(&ip_inp, port_inp) {
            Ok(result) => result,
            Err(e) => {
                error_msg(&format!("Nmap scan failed: {}", e));
                return Ok(());
            }
        };


        let open_ports = parse_nmap_output(&nmap_result);

        println!("{}", "=".repeat(50));
        success_msg(&format!("IP: {}", rg["query"].to_string().replace('\"', "")));
        success_msg(&format!("Country: {}", rg["country"].to_string().replace('\"', "")));
        success_msg(&format!("City: {}", rg["city"]).replace('\"', ""));
        success_msg(&format!("Organisation: {}", rg["org"].to_string().replace('\"', "")));

        if !open_ports.is_empty() {
            for port in open_ports {
                println!(
                    "[{}] Port {}   State: OPEN",
                    "+".green().bold(),
                    port.to_string().cyan(),
                );
            }
        } else {
            error_msg("No open ports found");
        }

        let end_t = Instant::now();
        let duration = end_t.duration_since(start_t);
        let result_t = duration.as_secs_f64();

        if result_t >= 60f64 {
            println!("\nTime: {}\n", format!("{:.2}m", result_t / 60f64).cyan());
        } else {
            println!("\nTime: {}\n", format!("{:.2}s", result_t).cyan());
        };
    } else {
        log::error!("Failed to get response from ip-api.com: {}", response.status());
        error_msg(&format!("Failed to get response from IP geolocation API: {}", response.status()));
    }
    Ok(())
}

fn parse_nmap_output(output: &str) -> HashSet<u16> {
    let mut open_ports = HashSet::new();
    let lines = output.lines();
    log::info!("Parsing Nmap output...");
    for line in lines {
        if line.contains("open") {
            if let Some(port) = line.split('/').next() {
                if let Ok(port_number) = port.parse::<u16>() {
                    open_ports.insert(port_number);
                }
            }
        }
    }

    open_ports
}

fn run_nmap_scan(ip: &str, port_count: u32) -> Result<String, Box<dyn std::error::Error>> {
    log::info!("Checking for nmap binary...");
    #[cfg(target_os = "windows")]
    {
        let nmap_path = env::current_dir()?.join("nmap-bin").join("nmap.exe");
        let output = Command::new(nmap_path).arg("-Pn").arg("-p").arg(format!("0-{}", port_count)).arg(ip).output()?;
        let output_str = String::from_utf8_lossy(&output.stdout).into_owned();
        log::info!("Nmap scan output: {}", output_str);
        Ok(output_str)
    }
    # [cfg(not(target_os = "windows"))]
    {
        let output = Command::new("nmap").arg("-Pn").arg("-p").arg(format!("0-{}", port_count)).arg(ip).output()?;
        let output_str = String::from_utf8_lossy(&output.stdout).into_owned();
        log::info!("Nmap scan output: {}", output_str);
        Ok(output_str)
    }
}
