use colored::*;
use crate::errors::TerminalError;


/// For detailed error messages
#[macro_export] macro_rules! error_msg {
    ($msg: expr) => {
        println!("{} | {}", "Error".bright_red(), $msg.red());
        println!("{}[{}{}] {}{}\n",
        " ".repeat("Error | ".len()),
        "E".red(), line!().to_string().bright_red(),
        "Source: ".dimmed(),
        file!().to_string().dimmed().underline());
    }
}

pub fn clear_terminal() -> Result<(), TerminalError> {
    Ok(clearscreen::clear()?)
}

pub fn log_msg(msg: &str) {
    println!("{0} {1}", "LOG:".bright_blue(), msg.dimmed());
}

/// For simple error messages
pub fn error_msg(msg: &str) {
    println!("{} | {}", "Error".bright_red(), msg.red());
}

pub fn notify_msg(msg: &str) {
    println!("{}{}{} {}", "[".red(), "-".white(), "]".red(), msg);
}