use std::time::Instant;
use std::collections::HashSet;
use reqwest::blocking::Client;
use serde_json::Value;
use std::process::Command;
use std::io;
use std::io::Write;
use std::net::IpAddr;
use colored::*;
use crate::behind::cli::{error_msg, success_msg};


fn is_ip_reachable(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}


pub (crate) fn selection_1() {
    let mut ip_inp = String::new();
    print!("\nEnter Target IP: ");
    io::stdout().flush().expect("flush failed!");
    io::stdin().read_line(&mut ip_inp).unwrap();
    ip_inp = ip_inp.trim().to_string();
    io::stdout().flush().expect("flush failed!");

    if !is_ip_reachable(&ip_inp) {
        error_msg("IP address not reachable");
        return;
    }


    println!("{}", "loading...".dimmed().bold());

    let start_t = Instant::now();

    println!("{}", "LOG: Fetching IP geolocation information...".dimmed()); // Log message
    let client = Client::new();
    let response = client
        .get(&format!("http://ip-api.com/json/{}", ip_inp))
        .send()
        .unwrap();

    let rg: Value = response.json().unwrap();

    println!("{}", "LOG: Running Nmap scan... (This could take a while.)".dimmed()); // Log message
    let nmap_result = match run_nmap_scan(&ip_inp) {
        Ok(result) => result,
        Err(e) => {
            error_msg(&format!("Nmap scan failed: {}", e));
            return;
        }
    };

    let open_ports = parse_nmap_output(&nmap_result);

    println!("{}", "=".repeat(34));
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
        println!("\nTime: {}\n", format!("{:.2}m", result_t / 60.0).cyan());
    } else {
        println!("\nTime: {}\n", format!("{:.2}s", result_t).cyan());
    }
}

fn parse_nmap_output(output: &str) -> HashSet<u16> {
    let mut open_ports = HashSet::new();
    let lines = output.lines();

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

fn run_nmap_scan(ip: &str) -> Result<String, Box<dyn std::error::Error>> {
    let nmap_path = std::env::current_dir()?.join("nmap-bin").join("nmap.exe");
    let output = Command::new(nmap_path)
        .arg("-Pn")
        .arg("-p")
        .arg("22-443")
        .arg(ip)
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout).into_owned();

    Ok(output_str)
}