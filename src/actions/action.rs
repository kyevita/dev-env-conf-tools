use std::any::type_name;

use crate::{cli::CLI, config::Configuration, errors::action_error::ActionError};

pub trait Action {
    fn execute(&self, cli: &CLI, app_config: Configuration) -> Result<(), ActionError>;
    fn verification(&self, _cli: &CLI, _app_config: Configuration) -> Result<(), ActionError> {
        Ok(())
    }

    fn get_action_name(&self) -> String {
        return String::from(type_name::<Self>());
    }
}
