use std::{env, path::PathBuf};

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
    let home_dir = get_home_dir();
    if let Some(mut data_dir) = home_dir {
        data_dir.push(".haxrs");
        return Some(data_dir);
    }

    None
}

pub fn get_home_dir() -> Option<PathBuf> {
    match env::var("HOME") {
        Ok(home) => Some(PathBuf::from(home)),
        Err(e) => {
            log::error!("Failed to get home directory: {}", e);
            error_msg(&format!("Failed to get home directory: {}", e));
            None
        }
    }
}
