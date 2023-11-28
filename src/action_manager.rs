use crate::{actions::action::{Action, ActionParams}, errors::action_error::ActionError};

#[derive(Copy, Clone)]
pub struct ActionManager {}

impl ActionManager {
  pub fn verify_all(self, actions: &Vec<Box<dyn Action>>, params: &ActionParams) -> Result<(), ActionError> {
    for action in actions.iter() {
      match action.verification(params.clone()) {
        Ok(_) => continue,
        Err(action_err) => return Err(action_err)
      };
    }

    return Ok(());
  }

  pub fn execute_all(self, actions: &Vec<Box<dyn Action>>, params: &ActionParams) {
    for action in actions.iter() {
      action.execute(params.clone());
    } 
  }
}