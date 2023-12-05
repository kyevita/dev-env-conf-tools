use git2::Repository;

use crate::{
    cli::CLI,
    config::Configuration,
    errors::action_error::ActionError,
    helpers::{oh_my_zsh_helper, zsh_helper},
};

use super::action::Action;

const POWERLEVEL10K_THEME_VALUE: &str = "powerlevel10k/powerlevel10k";

pub struct InstallPowerlevel10kAction {}

impl Action for InstallPowerlevel10kAction {
    fn verification(&self, _cli: &CLI, _app_config: Configuration) -> Result<(), ActionError> {
        //Verify if OhMyZsh is installed
        if !oh_my_zsh_helper::verify_installation() {
            return Err(ActionError {
                action: self.get_action_name(),
                message: String::from("OhMyZsh! is not installed on this system"),
            });
        }

        Ok(())
    }

    fn execute(&self, _cli: &CLI, app_config: Configuration) -> Result<(), ActionError> {
        let custom_themes_path = oh_my_zsh_helper::get_custom_themes_path();
        match Repository::clone(&app_config.powerlevel10k_git, custom_themes_path) {
            Ok(_) => println!("Powerlevel10k has been installed"),
            Err(err) => {
                return Err(ActionError {
                    action: self.get_action_name(),
                    message: String::from(err.message()),
                })
            }
        };

        zsh_helper::modify_zshrc_variable(
            zsh_helper::ZshVariables::ZshTheme,
            POWERLEVEL10K_THEME_VALUE,
        );
        Ok(())
    }
}
