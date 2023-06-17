use std::error::Error;
use std::fmt;
use zphisher::errors::TerminalError;

#[derive(Debug)]
pub enum VersionCheckError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    VersionNotFound,
    VersionParseError(semver::Error),
}

pub struct MyClearScreenError(clearscreen::Error);

impl From<MyClearScreenError> for TerminalError {
    fn from(err: MyClearScreenError) -> TerminalError {
        TerminalError::ClearError(err.0)
    }
}

pub struct MyIOError(std::io::Error);

impl From<MyIOError> for TerminalError {
    fn from(err: MyIOError) -> TerminalError {
        TerminalError::CommandIOError(err.0)
    }
}

pub struct MyReqwestError(reqwest::Error);

impl From<MyReqwestError> for TerminalError {
    fn from(err: MyReqwestError) -> TerminalError {
        TerminalError::ReqwestError(err.0)
    }
}

pub struct MyStrError(&'static str);

impl From<MyStrError> for TerminalError {
    fn from(err: MyStrError) -> TerminalError {
        TerminalError::CommandIOError(std::io::Error::new(std::io::ErrorKind::Other, err.0))
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