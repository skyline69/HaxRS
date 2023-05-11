use std::env;
use chrono::Local;
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;
use crate::behind::cli::error_msg;


pub(crate) fn log_init() {
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();


    // check if os is linux and create logs folder in home directory
    let filename: String = if env::consts::OS != "windows"  || env::consts::OS == "linux" {
        if let Err(e) = std::fs::create_dir_all(format!("{}/.haxrs/logs", get_home())) {
            error_msg(&format!("Failed to create log directory: {}", e));
            std::process::exit(1);
        }
        format!("{}/.haxrs/logs/execution-{}.log", get_home(), date)
    } else {
        if let Err(e) = std::fs::create_dir_all("logs") {
            error_msg(&format!("Failed to create log directory: {}", e));
            std::process::exit(1);
        }
        format!("logs/execution-{}.log", date)
    };

    let logfile: FileAppender = {
        match FileAppender::builder().encoder(Box::new(PatternEncoder::new("{d} {l} - {m}\n"))).build(filename) {
            Ok(file_appender) => file_appender,
            Err(e) => {
                error_msg(&format!("Failed to create log file: {}", e));
                std::process::exit(1);
            }
        }
    };
    let config = {
        match Config::builder().appender(Appender::builder().build("logfile", Box::new(logfile))).build(
            Root::builder().appender("logfile").build(LevelFilter::Info),
        ) {
            Ok(config) => config,
            Err(e) => {
                error_msg(&format!("Failed to create log config: {}", e));
                std::process::exit(1);
            }
        }
    };
    // clear the log file on startup
    // check if os is linux and create logs folder in home directory
    if env::consts::OS != "windows"  || env::consts::OS == "linux" {
        if let Err(e) = std::fs::write(format!("{}/.haxrs/logs/execution-{}.log", get_home(), date), "") {
            error_msg(&format!("Failed to clear log file: {}", e));
            std::process::exit(1);
        }
    } else if let Err(e) = std::fs::write(format!("logs/execution-{}.log", date), "") {
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

fn get_home() -> String {
    match env::var("HOME") {
        Ok(home) => home,
        Err(e) => {
            error_msg(&format!("Failed to get home directory: {}", e));
            std::process::exit(1);
        }
    }
}