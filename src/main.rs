mod cli;
mod actions;
mod action_manager;
mod errors;

use action_manager::ActionManager;
use actions::{zsh_config_action::ZshConfigAction, action::{Action, ActionParams}};
use cli::CLI;

fn main() {
  let cli = CLI::new();
  let home_path = cli.get_home_path().unwrap();
  let mut actions: Vec<Box<dyn Action>> = vec![];

  actions.push(Box::new(ZshConfigAction {}));
 
  let action_manager = ActionManager {};
  let action_params = ActionParams {
    cli,
    home_path
  };

  action_manager.verify_all(&actions, &action_params).unwrap();
  action_manager.execute_all(&actions, &action_params);
}