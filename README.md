# PacManager - A universal library for managing packages
A utility to interact with any package manager on any Linux distro

## Package manager support
Currently supported:
 - Apt
 - Yum

## Example
Taken from `examples/install.rs`
```
use pacmanager_wrapper::{execute_action, PacManagerAction, PacManagerCommand};
use futures_lite::{io::BufReader, prelude::*};

#[tokio::main]
async fn main() {
    // Create a PacManagerAction
    let action = PacManagerAction {
        pacmanager_command: PacManagerCommand::Install("lolcat".to_string()), // The action we want to do (which includes the package)
        internal_config: Default::default(),
        non_interactive: true,
        custom_flags: None,
    };

    // Execute the action with APT and BufRead its output
    let mut child = execute_action(action, pacmanager_wrapper::PacManager::Apt).await.unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    // Print out the PacManager's stdout
    while let Some(line) = lines.next().await {
        println!("{}", line.unwrap());
    }
}
```