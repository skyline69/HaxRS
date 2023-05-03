use std::io::{Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


pub fn clear_terminal() {
    print!("{esc}", esc = 27 as char);
}
#[allow(unused)]
pub fn print_menu_table() {
    todo!("Make the menu table thing.")
}

pub fn print_login_logo() {
    let mut stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).expect("Error at setting color");
    writeln!(&mut stdout, r#"
     888
     888 .oo.    .oooo.   oooo    ooo
     888P"Y88b  `P  )88b   `88b..8P'
     888   888   .oP"888     Y888'
     888   888  d8(  888   .o8"'88b
    o888o o888o `Y888""8o o88'   888o
"#).expect("Failed to print login logo");
}