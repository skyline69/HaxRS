use crate::behind::errors::TerminalError;
use crate::behind::zphisher::kill_pid;
use crate::behind::zphisher::{install_dependencies, setup_directories};

pub fn selection_2() -> Result<(), TerminalError> {
    setup_directories();
    kill_pid();
    install_dependencies();
    Ok(())
}
