use chrono::Local;
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;


pub(crate) fn log_init() {
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();

    let filename = format!("logs/execution-{}.log", date);

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}\n")))
        .build(filename)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .unwrap();
    // clear the log file on startup
    std::fs::write(format!("logs/execution-{}.log", date), "").unwrap();
    log4rs::init_config(config).unwrap();
}