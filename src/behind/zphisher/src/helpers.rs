use std::path::{Path, PathBuf};
use std::env::consts::{ARCH, OS};
use std::process::exit;
use colored::Colorize;

pub fn error_msg(msg: &str) {
    println!("{0} | {1}", "Error".bright_red(), msg.red());
}


pub fn get_server_dir() -> Option<PathBuf> {
    let data_dir = get_data_dir();

    if let Some(mut server_dir) = data_dir {
        server_dir.push("zphisher"); // haxrs/zphisher
        server_dir.push(".server"); // haxrs/zphisher/.server

        log::info!("server_dir: {:?}", server_dir);

        return Some(server_dir);
    }

    None
}

pub fn get_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    if let Some(mut data_dir) = dirs::data_dir() {
        data_dir.push("HaxRS");
        return Some(data_dir);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let home_dir = get_home_dir();
        if let Some(mut data_dir) = home_dir {
            data_dir.push(".haxrs");
            return Some(data_dir);
        }
    }

    None
}

pub fn get_sites_dir() -> Option<PathBuf> {
    // Windows: C:\Users\user\AppData\Roaming\HaxRS\zphisher\.server\sites
    // Linux: /home/user/.haxrs/zphisher/.server/sites

    if let Some(mut sites_dir) = get_server_dir() {
        sites_dir.push("sites");
        return Some(sites_dir);
    }

    None
}

#[cfg(not(target_os = "windows"))]
pub fn get_home_dir() -> Option<PathBuf> {
    match std::env::var("HOME") {
        Ok(home) => Some(PathBuf::from(home)),
        Err(e) => {
            log::error!("Failed to get home directory: {}", e);
            error_msg(&format!("Failed to get home directory: {}", e));
            None
        }
    }
}


pub fn files_exist(bin_path: &Path) -> bool {
    #[cfg(target_os = "windows")] let windows_amd = ["loclx.exe", "cloudflared-windows-amd64.exe"];
    #[cfg(target_os = "windows")] let windows_intel= ["loclx.exe", "cloudflared-windows-386.exe"];

    #[cfg(target_os = "windows")]
    if windows_amd.iter().all(|x| bin_path.join(x).exists()) | windows_intel.iter().all(|x| bin_path.join(x).exists()) {
        return true;
    }

    #[cfg(not(target_os = "windows"))] let linux_amd = ["loclx", "cloudflared-linux-amd64"];
    #[cfg(not(target_os = "windows"))] let linux_intel = ["loclx", "cloudflared-linux-386"];

    #[cfg(not(target_os = "windows"))]
    if linux_amd.iter().all(|x| bin_path.join(x).exists()) | linux_intel.iter().all(|x| bin_path.join(x).exists()) {
        return true;
    }
    false
}

pub fn get_download_urls<'a>() -> [&'a str; 2] {
    let cloudflare_download_url = match (ARCH, OS) {
        ("x86_64", "linux") => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64",
        ("aarch64", "linux") => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm64",
        (_, "windows") => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-windows-amd64.exe",
        _ => {
            error_msg("Unsupported architecture or OS for Cloudflare");
            exit(1);
        }
    };


    let localxpose_download_url = match (ARCH, OS) {
        ("x86_64", "linux") => "https://api.localxpose.io/api/v2/downloads/loclx-linux-amd64.zip",
        ("aarch64", "linux") => "https://api.localxpose.io/api/v2/downloads/loclx-linux-arm64.zip",
        (_, "windows") => "https://api.localxpose.io/api/v2/downloads/loclx-windows-amd64.zip",
        _ => {
            error_msg("Unsupported architecture or OS for LocalXPose");
            exit(1);
        }
    };


    [cloudflare_download_url, localxpose_download_url]
}

pub fn get_cloudflare_file() -> PathBuf {
    match (ARCH, OS) {
        ("x86_64", "linux") => match get_server_dir() {
            Some(mut server_dir) => {
                server_dir.push("cloudflared-linux-amd64");
                server_dir
            }
            None => {
                error_msg("Failed to get server directory");
                exit(1);
            }
        }
        ("aarch64", "linux") => match get_server_dir() {
            Some(mut server_dir) => {
                server_dir.push("cloudflared-linux-arm64");
                server_dir
            }
            None => {
                error_msg("Failed to get server directory");
                exit(1);
            }
        }
        (_, "windows") => match get_server_dir() {
            Some(mut server_dir) => {
                server_dir.push("cloudflared-windows-amd64.exe");
                server_dir
            }
            None => {
                error_msg("Failed to get server directory");
                exit(1);
            }
        }
        _ => {
            error_msg("Unsupported architecture or OS for Cloudflare");
            exit(1);
        }
    }
}