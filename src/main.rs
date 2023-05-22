mod behind;

use behind::cli;
use crossterm::{
    execute,
    terminal::SetTitle
};
use std::io::stdout;
use crate::behind::constants::WINDOW_TITLE;
use crate::behind::errors::TerminalError;
use crate::behind::log::log_init;

#[tokio::main]
async fn main() -> Result<(), TerminalError> {
    execute!(stdout(), SetTitle(WINDOW_TITLE))?;
    log_init();
    cli::print_login_logo();
    if let Err(e) = cli::menu_table().await {
        cli::error_msg(&format!("Failed to print menu table: {}", e));
        std::process::exit(1);
    }
    Ok(())
}

