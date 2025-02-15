use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DirectoryNotFoundError {
    pub message: String,
    pub code: i32,
}

impl fmt::Display for DirectoryNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyCustomError: {} (code: {})", self.message, self.code)
    }
}

impl Error for DirectoryNotFoundError {}
