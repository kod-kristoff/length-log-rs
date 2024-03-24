use std::{error, fmt, io};
#[derive(Debug)]
pub enum Error {
    InvalidQuoting(String),
    Io(io::Error),
    Unknown(String),
}
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidQuoting(ref err) => write!(f, "Invalid quoting: '{}'", err),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Unknown(ref err) => write!(f, "Unknown error: {}", err),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
