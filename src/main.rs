mod behind;

use behind::cli;
use crossterm::{
    execute,
    terminal::SetTitle,
    Result,
};
use std::io::stdout;


fn main() -> Result<()> {
    execute!(stdout(), SetTitle("HaxRS"))?;
    cli::print_login_logo();
    cli::menu_table();
    Ok(())
}