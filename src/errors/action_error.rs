use core::fmt;

#[derive(Debug)]
pub struct ActionError {
    pub action: String,
    pub message: String,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error on action {}: {}", self.action, self.message)
    }
}
