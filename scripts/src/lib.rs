use std::{error::Error, fmt, num::NonZeroI32, process::ExitStatus};

use octocrab::models::repos::Ref;

pub fn ref_sha(reference: Ref) -> Result<String, Box<dyn Error>> {
    match reference.object {
        octocrab::models::repos::Object::Commit { sha, .. } => Ok(sha),
        octocrab::models::repos::Object::Tag { sha, .. } => Ok(sha),
        _ => Err("Unknown reference object.".into()),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExitStatusError(Option<NonZeroI32>);

impl fmt::Display for ExitStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.0 {
            write!(f, "process exited unsuccessfully: {code}")
        } else {
            write!(f, "process exited unsuccessfully: unknown")
        }
    }
}

impl Error for ExitStatusError {}

pub trait ExitStatusExt {
    fn stable_exit_ok(&self) -> Result<(), ExitStatusError>;
}

impl ExitStatusExt for ExitStatus {
    fn stable_exit_ok(&self) -> Result<(), ExitStatusError> {
        match self.code() {
            Some(code) => match NonZeroI32::try_from(code) {
                Ok(code) => Err(ExitStatusError(Some(code))),
                Err(_) => Ok(()),
            },
            None => Err(ExitStatusError(None)),
        }
    }
}
