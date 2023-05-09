mod behind;

use behind::cli;
use crossterm::{
    execute,
    terminal::SetTitle,
    Result,
};
use std::io::stdout;
use crate::behind::log::log_init;

fn main() -> Result<()> {
    execute!(stdout(), SetTitle("HaxRS"))?;
    log_init();
    cli::print_login_logo();
    cli::menu_table().unwrap();
    Ok(())
}

