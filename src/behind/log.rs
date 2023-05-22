use crate::behind::cli::error_msg;
use chrono::Local;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

use super::helpers::get_data_dir;

pub fn log_init() {
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();

    let log_dir_path: String = match get_data_dir() {
        Some(mut log_dir) => {
            log_dir.push("logs");
            match log_dir.to_str() {
                Some(s) => s.to_owned(),
                None => {
                    error_msg("Failed to convert log directory to string");
                    std::process::exit(1);
                }
            }
        }
        None => {
            error_msg("Failed to get home directory");
            std::process::exit(1);
        }
    };

    if let Err(e) = std::fs::create_dir_all(&log_dir_path) {
        error_msg(&format!("Failed to create log directory: {}", e));
        std::process::exit(1);
    }

    let filename = format!("{}/execution-{}.log", log_dir_path, date);

    let logfile: FileAppender = {
        match FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}\n")))
            .build(filename.clone())
        {
            Ok(file_appender) => file_appender,
            Err(e) => {
                error_msg(&format!("Failed to create log file: {}", e));
                std::process::exit(1);
            }
        }
    };
    let config = {
        match Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        {
            Ok(config) => config,
            Err(e) => {
                error_msg(&format!("Failed to create log config: {}", e));
                std::process::exit(1);
            }
        }
    };
    // clear the log file on startup
    if let Err(e) = std::fs::write(&filename, "") {
        error_msg(&format!("Failed to clear log file: {}", e));
        std::process::exit(1);
    }

    match log4rs::init_config(config) {
        Ok(_) => {}
        Err(e) => {
            error_msg(&format!("Failed to initialize log config: {}", e));
            std::process::exit(1);
        }
    }
}
