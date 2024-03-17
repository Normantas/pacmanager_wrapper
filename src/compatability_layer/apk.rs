use crate::{PacManagerAction, PacManagerCommand, PacManagerError};

fn install(package: &str) -> String {
    format!("add {package}")
}

fn reinstall(_package: &str) -> Result<String, PacManagerError> {
    Err(PacManagerError::UnimplementedAction)
}

fn uninstall(package: &str) -> String {
    format!("del {package}")
}

fn update() -> String {
    "update".to_string()
}

fn upgrade() -> String {
    "upgrade".to_string()
}

fn list_pac() -> Result<String, PacManagerError> {
    Err(PacManagerError::UnimplementedAction)
}

fn search_pac(package: &str) -> String {
    format!("search {package}")
}

fn view_pac_details(package: &str) -> String {
    format!("info {package}")
}

/// Gets the pacmanager-specific command for this action
fn get_action_command(action: PacManagerAction) -> Result<String, PacManagerError> {
    Ok(match action.pacmanager_command {
        // Package management
        PacManagerCommand::Install(package) => install(&package),
        PacManagerCommand::Reinstall(package) => reinstall(&package)?,
        PacManagerCommand::Uninstall(package) => uninstall(&package),
        // System maintenance
        PacManagerCommand::Update => update(),
        PacManagerCommand::Upgrade => upgrade(),
        // Package search
        PacManagerCommand::List => list_pac()?,
        PacManagerCommand::Search(package) => search_pac(&package),
        PacManagerCommand::View(package) => view_pac_details(&package),
    })
}

pub fn get_command(action: PacManagerAction) -> Result<String, PacManagerError> {
    let mut pacmanager_command = String::new();
    let action_command = get_action_command(action.clone())?;

    pacmanager_command = format!(
        "{pacmanager_command}{} ",
        action.internal_config.root_command
    );
    pacmanager_command = format!("{pacmanager_command}apk");
    pacmanager_command = format!("{pacmanager_command} {action_command}");
    if let Some(extra_flags) = action.custom_flags {
        for extra_flag in extra_flags {
            pacmanager_command = format!("{pacmanager_command} {extra_flag}");
        }
    }

    Ok(pacmanager_command)
}

#[cfg(test)]
mod tests {
    use crate::{InternalConfig, PacManagerAction, PacManagerCommand};

    use super::get_command;

    #[test]
    fn install_package() {
        let dummy_action = PacManagerAction {
            pacmanager_command: PacManagerCommand::Install("Test".to_string()),
            internal_config: InternalConfig { root_command: "doas".to_string(), ..Default::default() },
            non_interactive: true,
            custom_flags: None,
        };

        assert_eq!(
            "doas apk add Test",
            get_command(dummy_action).unwrap()
        );
    }

    #[test]
    fn remove_package() {
        let dummy_action = PacManagerAction {
            pacmanager_command: PacManagerCommand::Uninstall("Test".to_string()),
            internal_config: InternalConfig { root_command: "doas".to_string(), ..Default::default() },
            non_interactive: true,
            custom_flags: Some(vec!["-q".to_string()]),
        };

        assert_eq!(
            "doas apk del Test -q",
            get_command(dummy_action).unwrap()
        );
    }

    #[test]
    fn update_package_list() {
        let dummy_action = PacManagerAction {
            pacmanager_command: PacManagerCommand::Update,
            internal_config: InternalConfig { root_command: "doas".to_string(), ..Default::default() },
            non_interactive: false,
            custom_flags: Some(vec!["--quiet".to_string()]),
        };

        assert_eq!(
            "doas apk update --quiet",
            get_command(dummy_action).unwrap()
        );
    }

    #[test]
    fn update_system_packages() {
        let dummy_action = PacManagerAction {
            pacmanager_command: PacManagerCommand::Upgrade,
            internal_config: InternalConfig { root_command: "doas".to_string(), ..Default::default() },
            non_interactive: true,
            custom_flags: None,
        };

        assert_eq!(
            "doas apk upgrade",
            get_command(dummy_action).unwrap()
        );
    }
}
