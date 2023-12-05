use core::fmt;

#[derive(Debug)]
pub enum TempFilesErrorActions {
    CreateFile,
    DeleteFile,
    CreateDir,
}
impl fmt::Display for TempFilesErrorActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TempFilesErrorActions::CreateFile => write!(f, "create temp file"),
            TempFilesErrorActions::CreateDir => write!(f, "create temp dir"),
            TempFilesErrorActions::DeleteFile => write!(f, "delete temp file"),
        }
    }
}

#[derive(Debug)]
pub struct TempFilesError<'a> {
    pub action: TempFilesErrorActions,
    pub target: &'a str,
    pub message: String,
}

impl<'a> fmt::Display for TempFilesError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unable to {} {}: {}",
            self.action, self.target, self.message
        )
    }
}
