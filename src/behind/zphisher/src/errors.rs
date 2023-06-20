use std::error::Error;
use std::fmt;


#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum TerminalError {
    ClearError(clearscreen::Error),
    CommandIOError(std::io::Error),
    ReqwestError(reqwest::Error),
}

impl TerminalError {
    pub fn new(msg: &str) -> TerminalError {
        TerminalError::CommandIOError(std::io::Error::new(std::io::ErrorKind::Other, msg))
    }
}

impl fmt::Display for TerminalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TerminalError::ClearError(err) => {
                log::error!("Could not clear terminal: {}", err);
                write!(f, "Could not clear terminal: {}", err)
            },
            TerminalError::CommandIOError(err) => {
                log::error!("Could not read/write to terminal: {}", err);
                write!(f, "Could not read/write to terminal: {}", err)
            },
            TerminalError::ReqwestError(err) => {
                log::error!("Request error: {}", err);
                write!(f, "Request error: {}", err)
            },
        }
    }
}

impl Error for TerminalError {}

impl From<clearscreen::Error> for TerminalError {
    fn from(err: clearscreen::Error) -> TerminalError {
        TerminalError::ClearError(err)
    }
}

impl From<std::io::Error> for TerminalError {
    fn from(err: std::io::Error) -> TerminalError {
        TerminalError::CommandIOError(err)
    }
}

impl From<reqwest::Error> for TerminalError {
    fn from(err: reqwest::Error) -> TerminalError {
        TerminalError::ReqwestError(err)
    }
}

impl From<&str> for TerminalError {
    fn from(err: &str) -> TerminalError {
        TerminalError::CommandIOError(std::io::Error::new(std::io::ErrorKind::Other, err))
    }
}

