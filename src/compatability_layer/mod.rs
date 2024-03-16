use async_process::Child;

use crate::{error::PacManagerError, pacmanager_exec, PacManager, PacManagerAction};

mod apt;
mod yum;

/// Executes the specified action.
/// Returns a `Child` object to the pacmanager's process.
pub async fn execute_action(
    action: PacManagerAction,
    pacmanager: PacManager,
) -> Result<Child, PacManagerError> {
    let pacmanager_command = match pacmanager {
        PacManager::Apt => apt::get_command(action.clone()),
        PacManager::Yum => yum::get_command(action.clone()),
    };

    let output = pacmanager_exec::execute_command(
        pacmanager_command,
        action.internal_config.interpreter_command,
    );
    match output {
        Ok(pacmanager_output) => Ok(pacmanager_output),
        Err(pacmanager_err) => Err(PacManagerError::InternalPacManagerError(
            pacmanager_err.to_string(),
        )),
    }
}
