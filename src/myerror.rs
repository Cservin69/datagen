use std::error::Error;
use std::fmt;
use std::io;
use serde_json;

#[derive(Debug)]
pub enum MyError {
    IoError(io::Error),
    JsonError(serde_json::Error),
}

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IoError(e) => write!(f, "IO error: {}", e),
            MyError::JsonError(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> Self {
        MyError::IoError(e)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(e: serde_json::Error) -> Self {
        MyError::JsonError(e)
    }
}