use std::process::Command;

use crate::helpers::oh_my_zsh_helper;
use crate::util::temp_files;
use crate::{cli::CLI, config::Configuration, errors::action_error::ActionError};

use super::action::Action;

pub struct InstallOhMyZshAction {}

impl Action for InstallOhMyZshAction {
    fn execute(&self, _cli: &CLI, app_config: Configuration) -> Result<(), ActionError> {
        // Verify current installation
        if oh_my_zsh_helper::verify_installation() {
            return Ok(println!("OhMyZsh! is already installed, skipping..."));
        }

        //Procced with installation
        let response =
            reqwest::blocking::get(app_config.oh_my_zsh_git_install_sh).expect("request failed");
        let content = response.text().expect("body invalid");
        let install_file_path = temp_files::new_temp_file("install_onmyzsh.sh", &content).unwrap();

        let installation = Command::new("sh").arg("-C").arg(install_file_path).spawn();

        match installation {
            Ok(_) => Ok(println!("OhMyZsh! is now installed")),
            Err(err) => Err(ActionError {
                action: self.get_action_name(),
                message: err.to_string(),
            }),
        }
    }
}
