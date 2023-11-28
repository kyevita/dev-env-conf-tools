use std::path::PathBuf;

use crate::{errors::action_error::ActionError, cli::CLI};

#[derive(Debug, Clone)]
pub struct ActionParams {
  pub cli: CLI,
  pub home_path: PathBuf
}
pub trait Action {
  fn verification(&self, params: ActionParams) -> Result<(), ActionError>;
  fn execute(&self, params: ActionParams) -> ();
}