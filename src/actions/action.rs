use crate::{cli::CLI, errors::action_error::ActionError};

pub trait Action {
    fn verification(&self, cli: &CLI) -> Result<(), ActionError>;
    fn execute(&self, cli: &CLI) -> ();
}
