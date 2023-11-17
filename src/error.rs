use std::fmt;
use serde_json;

#[derive(Debug)]
pub enum TaskError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TaskError::Io(ref err) => write!(f, "IO error: {}", err),
            TaskError::Json(ref err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl std::error::Error for TaskError {}

impl From<std::io::Error> for TaskError {
    fn from(err: std::io::Error) -> TaskError {
        TaskError::Io(err)
    }
}

impl From<serde_json::Error> for TaskError {
    fn from(err: serde_json::Error) -> TaskError {
        TaskError::Json(err)
    }
}
