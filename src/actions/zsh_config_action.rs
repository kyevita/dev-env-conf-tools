use std::{any::type_name, fs};

use crate::{cli::CLI, errors::action_error::ActionError};

use super::action::Action;

pub struct ZshConfigAction {}

impl Action for ZshConfigAction {
    fn verification(&self, cli: &CLI) -> Result<(), crate::errors::action_error::ActionError> {
        let home_dir = cli.get_home_path().unwrap();

        match fs::metadata(home_dir.join(".zshrc")) {
            Ok(_) => Ok(println!("ZSH config found!")),
            Err(_) => Err(ActionError {
                action: String::from(type_name::<Self>()),
                message: String::from("ZSH config file not found on {}"),
            }),
        }
    }

    fn execute(&self, cli: &CLI) {
        println!(
            "Do you want to copy your zsh config? THIS WILL OVERRIDE YOUR SAVED CONFIGURATION"
        );
        let option = cli.read_line().unwrap();
        let valid_option = "Y";

        if option.ne(valid_option) {
            return;
        }

        let home_dir = cli.get_home_path().unwrap();
        match fs::copy(home_dir.as_path().join(".zshrc"), "./config-files/.zshrc") {
            Ok(_) => print!("ZSH config: ✔"),
            Err(error) => print!("Error while moving ZSH config: {}", error.to_string()),
        }
    }
}
