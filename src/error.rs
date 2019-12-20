use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: impl ToString) -> Error {
        Error {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "openctm::Error({:?})", self.message)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            message: format!("IO error {}", err),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error {
            message: format!("UTF8 error {}", err),
        }
    }
}

macro_rules! error {
    ($($args:tt)*) => { $crate::error::Error::new(format!($($args)*)) }
}
