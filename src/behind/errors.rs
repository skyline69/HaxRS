use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VersionCheckError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    VersionNotFound,
    VersionParseError(semver::Error),
}

#[derive(Debug)]
pub enum TerminalError {
    ClearError(clearscreen::Error),
    CommandIOError(std::io::Error),
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
impl fmt::Display for VersionCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionCheckError::ReqwestError(err) =>
                {
                    log::error!("Reqwest error: {}", err);
                    write!(f, "Reqwest error: {}", err)
                },
            VersionCheckError::SerdeJsonError(err) => {
                log::error!("Serde json error: {}", err);
                write!(f, "Serde json error: {}", err)
            },
            VersionCheckError::VersionNotFound => {
                log::error!("Version not found in the provided JSON");
                write!(f, "Version not found in the provided JSON")
            },
            VersionCheckError::VersionParseError(err) => {
                log::error!("Version parse error: {}", err);
                write!(f, "Version parse error: {}", err)
            },
        }
    }
}

impl Error for VersionCheckError {}

impl From<reqwest::Error> for VersionCheckError {
    fn from(err: reqwest::Error) -> VersionCheckError {
        VersionCheckError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for VersionCheckError {
    fn from(err: serde_json::Error) -> VersionCheckError {
        VersionCheckError::SerdeJsonError(err)
    }
}

impl From<semver::Error> for VersionCheckError {
    fn from(err: semver::Error) -> VersionCheckError {
        VersionCheckError::VersionParseError(err)
    }
}