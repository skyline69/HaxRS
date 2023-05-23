use std::path::PathBuf;
use std::collections::HashMap;
use std::env::consts::{ARCH, OS};

use crate::behind::cli::error_msg;

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


pub fn get_download_urls() -> Vec<String> {
    let cloudflare_download_url: HashMap<&str, &str> = HashMap::from([
        ("windows", "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-windows-amd64.exe"),
        ("linux", {
            match ARCH {
                "x86_64" => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64",
                "aarch64" => "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-arm64",
                _ => {
                    error_msg("Unsupported architecture");
                    std::process::exit(1);
                }
            }
        })
    ]);

    let localxpose_download_url: HashMap<&str, &str> = HashMap::from([
        ("linux", {
            match ARCH {
                "x86_64" => "https://api.localxpose.io/api/v2/downloads/loclx-linux-amd64.zip",
                "aarch64" => "https://api.localxpose.io/api/v2/downloads/loclx-linux-arm64.zip",
                _ => {
                    error_msg("Unsupported architecture");
                    std::process::exit(1);
                }
            }
        }),
        (
            "windows",
            "https://api.localxpose.io/api/v2/downloads/loclx-windows-amd64.zip",
        ),
    ]);
    // Get values
    let links: Vec<String> = Vec::from([
        {
            match cloudflare_download_url.get(OS) {
                Some(url) => url.to_string(),
                None => {
                    error_msg("Unsupported OS");
                    std::process::exit(1);
                }
            }
        },
        {
            match localxpose_download_url.get(OS) {
                Some(url) => url.to_string(),
                None => {
                    error_msg("Unsupported OS");
                    std::process::exit(1);
                }
            }
        }
    ]);


    links
}