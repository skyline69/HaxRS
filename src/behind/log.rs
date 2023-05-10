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

    let filename = format!("logs/execution-{}.log", date);

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
    if let Err(e) = std::fs::write(format!("logs/execution-{}.log", date), "") {
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