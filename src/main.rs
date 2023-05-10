mod behind;

use behind::cli;
use crossterm::{
    execute,
    terminal::SetTitle
};
use std::io::stdout;
use crate::behind::errors::TerminalError;
use crate::behind::log::log_init;

fn main() -> Result<(), TerminalError> {
    execute!(stdout(), SetTitle("HaxRS"))?;
    log_init();
    cli::print_login_logo();
    if let Err(e) = cli::menu_table() {
        cli::error_msg(&format!("Failed to print menu table: {}", e));
        std::process::exit(1);
    }
    Ok(())
}

