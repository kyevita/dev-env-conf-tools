mod action_manager;
mod actions;
mod cli;
mod config;
mod errors;
mod helpers;
mod util;

use action_manager::ActionManager;
use actions::{action::Action, install_oh_my_zsh_action::InstallOhMyZshAction};
use cli::CLI;
use dotenv::dotenv;
use util::temp_files;

fn main() {
    dotenv().ok();
    temp_files::init();

    let app_config = config::load_config();
    let cli = CLI::new();
    let action_manager = ActionManager::new(&cli, &app_config);

    let actions: Vec<Box<dyn Action>> = vec![Box::new(InstallOhMyZshAction {})];

    action_manager.verify_all(&actions).unwrap();
    action_manager.execute_all(&actions);
}
