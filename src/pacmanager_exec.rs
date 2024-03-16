//! Actually executes the pacmanager process

use async_process::{Child, Command, Stdio};
use std::io::Error;

pub fn execute_command(
    pacman_command: String,
    interpreter_command: String,
) -> Result<Child, Error> {
    let child = Command::new(interpreter_command)
        .arg("-c")
        .arg(pacman_command)
        .stdout(Stdio::piped())
        .spawn()?;

    Ok(child)
}
