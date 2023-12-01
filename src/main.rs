mod cli;
mod actions;
mod action_manager;
mod errors;

use action_manager::ActionManager;
use actions::{zsh_config_action::ZshConfigAction, action::Action};
use cli::CLI;

fn main() {
  let cli = CLI::new();
  let action_manager = ActionManager::new(&cli);
  let actions: Vec<Box<dyn Action>> = vec![
    Box::new(ZshConfigAction {})
  ];

  action_manager.verify_all(&actions).unwrap();
  action_manager.execute_all(&actions);
}