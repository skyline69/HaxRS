use std::{
    collections::HashMap,
    env::consts::{ARCH, OS},
};

use super::cli::error_msg;

pub(crate) const GITHUB_API_LATEST_RELEASE: &str =
    "https://api.github.com/repos/skyline69/HaxRS/releases";

pub fn get_download_urls() -> Vec<String> {
    let cloudflared: HashMap<&str, &str> = HashMap::from([
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

    let localxpose: HashMap<&str, &str> = HashMap::from([
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
        cloudflared.get(OS).unwrap_or(&"").to_string(),
        localxpose.get(OS).unwrap_or(&"").to_string(),
    ]);
    // return
    links
}

pub(crate) const BIN_PATH: &str = "bin";

pub(crate) const WINDOW_TITLE: &str = "HaxRS";
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/113.0";
pub(crate) const INPUT_PROMPT: &str = "Selection: ";
pub(crate) const ZPHISHER_VERSION: &str = "2.3.5";

// Default values for Host and Port
// pub const HOST: &str = "127.0.0.1";
// pub const PORT: &str = "8080";
