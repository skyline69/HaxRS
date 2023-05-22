use colored::*;
use std::io;
use std::io::Write;
use async_recursion::async_recursion;
use crate::behind::constants::INPUT_PROMPT;
use crate::behind::errors::TerminalError;
use crate::behind::selection_1::selection_1;
use crate::behind::selection_2::selection_2;
use crate::behind::selection_3::selection_3;
use crate::behind::update::check_update;

pub fn clear_terminal() -> Result<(), TerminalError> {
    Ok(clearscreen::clear()?)
}

pub enum Command {
    Clear,
    Update,
    Exit,
    Menu,
    Selection1,
    Selection2,
    Selection3,
    Unknown,
    Empty,
}

async fn menu_table_select() -> Result<(), TerminalError> {
    clear_terminal()?;
    print_login_logo();
    match menu_table().await {
        Ok(_) => {},
        Err(e) => {
            error_msg(&format!("Failed to print menu table: {}", e));
            std::process::exit(1);
        }
    }
    Ok(())
}


pub fn command_input() -> Result<Command, TerminalError> {
    print!("{}", INPUT_PROMPT);
    io::stdout().flush()?;
    let mut selection = String::new();
    io::stdin().read_line(&mut selection)?;
    selection = selection.trim().to_string();
    if selection.is_empty() {
        return Ok(Command::Empty);
    }
    Ok(match selection.as_str() {
        "1" => Command::Selection1,
        "2" => Command::Selection2,
        "3" => Command::Selection3,
        "exit" => Command::Exit,
        "clear" => Command::Clear,
        "menu" => Command::Menu,
        "update" => Command::Update,
        _ => Command::Unknown,
    })
}

fn print_menu_table() -> Result<(), TerminalError>{
    println!("\n{}", "Select an Action.".bold().underline());
    println!("╔═══════════╦══════════════════════════════════╦══════════════╗");
    println!("║ {:<9} ║ {:<32} ║ {:<12} ║", "Selection".bold(), "Action".bold(), "Category".bold());
    println!("╠═══════════╬══════════════════════════════════╬══════════════╣");
    println!("║     1     ║ {:<32} ║ {:<12} ║", "Port Scanner".yellow(), "Network".bright_blue());
    println!("║     2     ║ {:<32} ║ {:<12} ║", "Phisher(Powered by ZPhisher)".yellow(), "Phishing".bright_blue());
    println!("║     3     ║ {:<32} ║ {:<12} ║", "URL-masker".yellow(), "Phishing".bright_blue());
    println!("╚═══════════╩══════════════════════════════════╩══════════════╝");
    println!("{}", "Commands".bold().underline());
    println!("'{0}' - Exit\n'{1}' - Clear Terminal\n'{2}' - Show Menu Table\n'{3}' - Check for Updates\n",
             "exit".bright_blue(), "clear".bright_blue(), "menu".bright_blue(), "update".bright_blue());
    Ok(())
}

#[async_recursion]
pub(crate) async fn menu_table() -> Result<(), TerminalError> {
    print_menu_table()?;
    loop {
        ({
            match command_input()? {
                Command::Selection1 => selection_1().await,
                Command::Selection2 => selection_2(),
                Command::Selection3 => selection_3().await,
                Command::Exit => std::process::exit(0),
                Command::Clear => menu_table_select().await,
                Command::Menu => print_menu_table(),
                Command::Update => {
                    if let Err(e) = check_update().await {
                        error_msg(&e.to_string());
                    }
                    Ok(())
                }
                Command::Empty => {
                    error_msg("Please enter command or selection");
                    Ok(())
                }
                Command::Unknown => {
                    error_msg("Invalid Selection");
                    Ok(())
                }
            }
        })?;
    }

}


pub(crate) fn success_msg(msg: &str) {
    println!("{0}: {1}", "Success".bright_green(), msg.green());
}

pub(crate) fn error_msg(msg: &str) {
    println!("{0}: {1}", "Error".bright_red(), msg.red());
}


pub fn print_login_logo() {
    let logo = r#"
     888
     888 .oo.    .oooo.   oooo    ooo
     888P"Y88b  `P  )88b   `88b..8P'
     888   888   .oP"888     Y888'
     888   888  d8(  888   .o8"'88b
    o888o o888o `Y888""8o o88'   888o"#;
    println!("{}", logo.green());
}


