use crate::behind::errors::TerminalError;
#[cfg(target_os = "linux")]
use crate::behind::zphisher::kill_pid;
use crate::behind::zphisher::{install_dependencies, setup_directories};

pub(crate) fn selection_2() -> Result<(), TerminalError> {
    setup_directories();
    #[cfg(target_os = "linux")]
    kill_pid();
    install_dependencies();
    Ok(())
}
