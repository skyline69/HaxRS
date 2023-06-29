mod behind;

use behind::cli;
use crossterm::{
    execute,
    terminal::SetTitle
};
use std::io::stdout;
use colored::Colorize;
use crate::behind::constants::WINDOW_TITLE;
use zphisher::errors::TerminalError;
use crate::behind::log::log_init;

#[tokio::main]
async fn main() -> Result<(), TerminalError> {
    execute!(stdout(), SetTitle(WINDOW_TITLE))?;
    ctrlc::set_handler(move || {
        eprintln!("\n{}", "Goodbye and thanks for using Hax! :D".green());
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
    log_init();
    cli::print_hax_logo();
    if let Err(e) = cli::menu_table().await {
        cli::error_msg(&format!("Failed to print menu table: {:?}", e));
        std::process::exit(1);
    }
    Ok(())
}

