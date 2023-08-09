use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct MachineIdError;

impl Error for MachineIdError {}

impl fmt::Display for MachineIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
