use colored::*;
use std::io;
use std::io::Write;
use crate::behind::selection_1::selection_1;


pub fn clear_terminal() {
    clearscreen::clear().unwrap();
}


fn menu_table_select() {
    clear_terminal();
    print_login_logo();
    menu_table()
}



pub (crate) fn menu_table() {
    println!("\n{}", "Select an Action.".bold().underline());
    println!("╔═══════════╦══════════════════════════════════╦══════════════╗");
    println!("║ {:<9} ║ {:<32} ║ {:<12} ║", "Selection".bold(), "Action".bold(), "Category".bold());
    println!("╠═══════════╬══════════════════════════════════╬══════════════╣");
    println!("║     1     ║ {:<32} ║ {:<12} ║", "Port Scanner".yellow(), "Network".bright_blue());
    println!("║     2     ║ {:<32} ║ {:<12} ║", "Phisher(Powered by ZPhisher)".yellow(), "Phishing".bright_blue());
    println!("║     3     ║ {:<32} ║ {:<12} ║", "URL-masker".yellow(), "Phishing".bright_blue());
    println!("╚═══════════╩══════════════════════════════════╩══════════════╝");
    println!("{}", "Commands".bold().underline());
    println!("'{0}' - Exit\n'{1}' - Clear Terminal\n'{2}' - Show Menu Table\n", "exit".bright_blue(), "clear".bright_blue(), "menu".bright_blue());


    loop {
        print!("Selection: ");
        io::stdout().flush().expect("flush failed!");
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).unwrap();
        selection = selection.trim().to_string();
        match selection.as_str() {
            "1" => selection_1(),
            "2" => println!("{}", "This feature isn't available yet.".red()),
            "3" => println!("{}", "This feature isn't available yet.".red()),
            "exit" => std::process::exit(0),
            "clear" => menu_table_select(),
            "menu" => menu_table(),
            _ => {
                error_msg("Invalid Selection");
            }
        }
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