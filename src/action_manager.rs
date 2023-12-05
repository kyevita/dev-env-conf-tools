use crate::{actions::action::Action, cli::CLI, errors::action_error::ActionError};

#[derive(Clone, Copy)]
pub struct ActionManager<'a> {
    cli: &'a CLI,
}

impl<'a> ActionManager<'a> {
    pub fn new(cli: &CLI) -> ActionManager<'_> {
        ActionManager { cli }
    }

    pub fn verify_all(self, actions: &Vec<Box<dyn Action>>) -> Result<(), ActionError> {
        for action in actions.iter() {
            match action.verification(self.cli) {
                Ok(_) => continue,
                Err(action_err) => return Err(action_err),
            };
        }

        return Ok(());
    }

    pub fn execute_all(self, actions: &Vec<Box<dyn Action>>) {
        for action in actions.iter() {
            action.execute(self.cli);
        }
    }
}
