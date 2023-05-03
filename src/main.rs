mod behind;

use behind::cli;



fn main() {
    cli::clear_terminal();
    //cli::print_menu_table();
    cli::print_login_logo()
}