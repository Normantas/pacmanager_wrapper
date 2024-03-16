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

    // Print out the pacmanager's stdout
    while let Some(line) = lines.next().await {
        println!("{}", line.unwrap());
    }
}
