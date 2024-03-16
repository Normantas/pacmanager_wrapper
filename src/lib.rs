//! `pacmanager_wrapper` is a utility to interact with any package manager on any Linux distro
//! 
//! Example usage:
//! ```
//! use pacmanager_wrapper::{execute_action, PacManagerAction, PacManagerCommand};
//! use futures_lite::{io::BufReader, prelude::*};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a PacManagerAction
//!     let action = PacManagerAction {
//!         pacmanager_command: PacManagerCommand::Install("lolcat".to_string()), // The action we want to do (which includes the package)
//!         internal_config: Default::default(),
//!         non_interactive: true,
//!         custom_flags: None,
//!     };
//!
//!     // Execute the action with APT and BufRead its output
//!     let mut child = execute_action(action, pacmanager_wrapper::PacManager::Apt).await.unwrap();
//!     let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
//!
//!     // Print out the PacManager's stdout
//!     while let Some(line) = lines.next().await {
//!         println!("{}", line.unwrap());
//!     }
//! }
//! ```

mod compatability_layer;
mod pacmanager_exec;
mod error;

pub use error::PacManagerError;
pub use compatability_layer::execute_action;

#[derive(Clone, Debug)]
pub struct InternalConfig {
    pub root_command: String,
    pub interpreter_command: String,
}
impl Default for InternalConfig {
    fn default() -> Self {
        InternalConfig {
            root_command: "sudo".to_string(),
            interpreter_command: "sh".to_string(),
        }
    }
}

/// A package. Could be "lolcat", or maybe a specific version such as "lolcat=100.0.1-3" (if the package manager supports it)
pub type Package = String;

/// The package manager to use
pub enum PacManager {
    Apt,
    Yum,
}

/// The specific command to execute - "install", "update", etc.
#[derive(Clone, Debug)]
pub enum PacManagerCommand {
    /*
    Package management
    */
    /// Installs a package
    Install(Package),
    /// Reinstalls a package
    Reinstall(Package),
    /// Uninstalls a package
    Uninstall(Package),

    /*
    System maintenance
    */
    /// Updates the system package list
    Update,
    /// Upgrades system packages
    Upgrade,

    /*
    Package search
    */
    /// Lists available packages
    List,
    /// Searches for a package
    Search(Package),
    /// Views package details
    View(Package),
}

/// An action of the package manager - the command to execute, custom flags, etc.
/// This is passed to the 'execute_action' function.
#[derive(Clone, Debug)]
pub struct PacManagerAction {
    /// The command that should be executed
    pub pacmanager_command: PacManagerCommand,
    /// The "internal" configuration
    pub internal_config: InternalConfig,
    /// Sets a "non interactive" flag when possible, this prevents confirmation prompts and other things that require manual interaction
    pub non_interactive: bool,
    /// Custom flags which should be passed to the package manager
    pub custom_flags: Option<Vec<String>>,
}
