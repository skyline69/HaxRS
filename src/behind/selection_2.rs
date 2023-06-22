use zphisher::errors::TerminalError;

use zphisher::zphisher::{install_dependencies, kill_pid, main_menu, setup_directories};

pub async fn selection_2() -> Result<(), TerminalError> {
    setup_directories();
    kill_pid();
    install_dependencies().await;
    main_menu().await?;
    Ok(())
}
