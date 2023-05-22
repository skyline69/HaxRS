use crate::behind::errors::TerminalError;
use crate::behind::zphisher::{install_dependencies, kill_pid, setup_directories};

pub(crate) fn selection_2() -> Result<(), TerminalError> {
    setup_directories();
    kill_pid();
    install_dependencies();
    Ok(())
}
