use crate::{
    actions::action::Action, cli::CLI, config::Configuration, errors::action_error::ActionError,
};

#[derive(Clone, Copy)]
pub struct ActionManager<'a> {
    cli: &'a CLI,
    app_config: &'a Configuration,
}

impl<'a> ActionManager<'a> {
    pub fn new(cli: &'a CLI, app_config: &'a Configuration) -> ActionManager<'a> {
        ActionManager { cli, app_config }
    }

    pub fn verify_all(self, actions: &Vec<Box<dyn Action>>) -> Result<(), ActionError> {
        for action in actions.iter() {
            match action.verification(self.cli, self.app_config.clone()) {
                Ok(_) => continue,
                Err(action_err) => return Err(action_err),
            };
        }

        return Ok(());
    }

    pub fn execute_all(self, actions: &Vec<Box<dyn Action>>) {
        for action in actions.iter() {
            action.execute(self.cli, self.app_config.clone()).unwrap();
        }
    }
}
