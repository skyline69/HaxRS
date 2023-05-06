mod behind;

use behind::cli;

fn main() {
    cli::clear_terminal();
    cli::print_login_logo();
    cli::menu_table()
}