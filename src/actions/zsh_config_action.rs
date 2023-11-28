use std::{fs, any::type_name};

use crate::errors::action_error::ActionError;

use super::action::{Action, ActionParams};


pub struct ZshConfigAction {}

impl Action for ZshConfigAction {
  fn verification(&self, params: ActionParams) -> Result<(), crate::errors::action_error::ActionError> {
    let home_dir  = params.home_path;

    match fs::metadata(home_dir.join(".zshrc")) {
      Ok(_) => Ok(println!("ZSH config found!")),
      Err(_) => Err(
        ActionError {
          action: String::from(type_name::<Self>()),
          message: String::from("ZSH config file not found on {}") 
        }
      )
    }
  }

  fn execute(&self, params: ActionParams) {
    let cli = params.cli;

    println!("Do you want to copy your zsh config? THIS WILL OVERRIDE YOUR SAVED CONFIGURATION");

    let option = cli.read_line().unwrap();
    let valid_option = "Y";

    if option.ne(valid_option) {
      return;
    } 

    match fs::copy(params.home_path.as_path().join(".zshrc"), "./config-files/.zshrc") {
      Ok(_) => print!("ZSH config: ✔"),
      Err(error) => print!("Error while moving ZSH config: {}", error.to_string())
    }
  }
}