use crate::{PacManagerAction, PacManagerCommand};

fn install(package: &str) -> String {
    format!("-S {package}")
}

fn reinstall(package: &str) -> String {
    format!("-S {package}")
}

fn uninstall(package: &str) -> String {
    format!("-R {package}")
}

fn update() -> String {
    "-Sy".to_string()
}

fn upgrade() -> String {
    "-Su".to_string()
}

fn list_pac() -> String {
    "-Qq".to_string()
}

fn search_pac(package: &str) -> String {
    format!("-Ss {package}")
}

fn view_pac_details(package: &str) -> String {
    format!("-Si {package}")
}

/// Gets the pacmanager-specific command for this action
fn get_action_command(action: PacManagerAction) -> String {
    match action.pacmanager_command {
        // Package management
        PacManagerCommand::Install(package) => install(&package),
        PacManagerCommand::Reinstall(package) => reinstall(&package),
        PacManagerCommand::Uninstall(package) => uninstall(&package),
        // System maintenance
        PacManagerCommand::Update => update(),
        PacManagerCommand::Upgrade => upgrade(),
        // Package search
        PacManagerCommand::List => list_pac(),
        PacManagerCommand::Search(package) => search_pac(&package),
        PacManagerCommand::View(package) => view_pac_details(&package),
    }
}

pub fn get_command(action: PacManagerAction) -> String {
    let mut pacmanager_command = String::new();
    let action_command = get_action_command(action.clone());

    pacmanager_command = format!(
        "{pacmanager_command}{} ",
        action.internal_config.root_command
    );
    pacmanager_command = format!("{pacmanager_command}pacman");
    pacmanager_command = format!("{pacmanager_command} {action_command}");
    if action.non_interactive {
        pacmanager_command = format!("{pacmanager_command} --noconfirm");
    }
    if let Some(extra_flags) = action.custom_flags {
        for extra_flag in extra_flags {
            pacmanager_command = format!("{pacmanager_command} {extra_flag}");
        }
    }

    pacmanager_command
}

#[cfg(test)]
mod tests {
    use crate::{PacManagerAction, PacManagerCommand};

    use super::get_command;

    #[test]
    fn install_package() {
        let dummy_action = PacManagerAction {
            pacmanager_command: PacManagerCommand::Install("Test".to_string()),
            internal_config: Default::default(),
            non_interactive: true,
            custom_flags: None,
        };

        assert_eq!(
            "sudo pacman -S Test --noconfirm",
            get_command(dummy_action)
        );
    }

    #[test]
    fn reinstall_package() {
        let dummy_action = PacManagerAction {
            pacmanager_command: PacManagerCommand::Reinstall("Test".to_string()),
            internal_config: Default::default(),
            non_interactive: false,
            custom_flags: Some(vec![]),
        };

        assert_eq!(
            "sudo pacman -S Test",
            get_command(dummy_action)
        );
    }

    #[test]
    fn remove_package() {
        let dummy_action = PacManagerAction {
            pacmanager_command: PacManagerCommand::Uninstall("Test".to_string()),
            internal_config: Default::default(),
            non_interactive: true,
            custom_flags: Some(vec!["-w".to_string()]),
        };

        assert_eq!(
            "sudo pacman -R Test --noconfirm -w",
            get_command(dummy_action)
        );
    }

    #[test]
    fn update_package_list() {
        let dummy_action = PacManagerAction {
            pacmanager_command: PacManagerCommand::Update,
            internal_config: Default::default(),
            non_interactive: false,
            custom_flags: Some(vec!["--print".to_string()]),
        };

        assert_eq!(
            "sudo pacman -Sy --print",
            get_command(dummy_action)
        );
    }

    #[test]
    fn update_system_packages() {
        let dummy_action = PacManagerAction {
            pacmanager_command: PacManagerCommand::Upgrade,
            internal_config: Default::default(),
            non_interactive: true,
            custom_flags: None,
        };

        assert_eq!(
            "sudo pacman -Su --noconfirm",
            get_command(dummy_action)
        );
    }
}
