use crate::{errors::action_error::ActionError, cli::CLI};

pub trait Action {
  fn verification(&self, cli: &CLI) -> Result<(), ActionError>;
  fn execute(&self, cli: &CLI) -> ();
}