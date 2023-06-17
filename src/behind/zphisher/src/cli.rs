use colored::*;
use crate::errors::TerminalError;

pub fn clear_terminal() -> Result<(), TerminalError> {
    Ok(clearscreen::clear()?)
}

pub fn log_msg(msg: &str) {
    println!("{0} {1}", "LOG:".bright_blue(), msg.dimmed());
}

pub fn error_msg(msg: &str) {
    println!("{0} | {1}", "Error".bright_red(), msg.red());
}
