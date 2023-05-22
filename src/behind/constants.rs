use std::env;
use std::path::PathBuf;
use crate::behind::cli::error_msg;

pub(crate) const GITHUB_API_LATEST_RELEASE: &str = "https://api.github.com/repos/skyline69/HaxRS/releases";
pub(crate) const CLOUDFLARE_DOWNLOAD_URL: &str = "https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-windows-amd64.exe";
pub(crate) const LOCALXPOSE_DOWNLOAD_URL: &str = "https://api.localxpose.io/api/v2/downloads/loclx-windows-amd64.zip";
pub(crate) const BIN_PATH: &str = "bin";

pub(crate) const WINDOW_TITLE: &str = "HaxRS";
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/113.0";
pub(crate) const INPUT_PROMPT: &str = "Selection: ";
pub(crate) const ZPHISHER_VERSION: &str = "2.3.5";

pub(crate) fn linux_server_dir() -> Option<PathBuf> {
    let data_dir = linux_data_dir();
    if let Some(mut server_dir) = data_dir {
        server_dir.push("zphisher");
        server_dir.push(".server");
        return Some(server_dir);
    }
    None
}

pub(crate) fn windows_server_dir() -> Option<PathBuf> {
    let data_dir = windows_data_dir();
    log::info!("data_dir: {:?}", data_dir);
    if let Some(mut server_dir) = data_dir {
        server_dir.push("zphisher"); // haxrs/zphisher
        server_dir.push(".server");// haxrs/zphisher/.server
        log::info!("server_dir: {:?}", server_dir);
        return Some(server_dir);
    }
    None
}

pub(crate) fn linux_data_dir() -> Option<PathBuf> {
    let home_dir = get_home_dir();
    if let Some(mut data_dir) = home_dir {
        data_dir.push(".haxrs");
        return Some(data_dir);
    }
    None
    //home_dir.join(".haxrs")
}

pub(crate) fn windows_data_dir() -> Option<PathBuf> {
    if let Some(mut data_dir) = dirs::data_dir() {
        data_dir.push("HaxRS");
        return Some(data_dir);
    }
    None
}

pub(crate) fn get_home_dir() -> Option<PathBuf> {
    match env::var("HOME") {
        Ok(home) => Some(PathBuf::from(home)),
        Err(e) => {
            log::error!("Failed to get home directory: {}", e);
            error_msg(&format!("Failed to get home directory: {}", e));
            None
        }
    }
}

// Default values for Host and Port
// pub const HOST: &str = "127.0.0.1";
// pub const PORT: &str = "8080";