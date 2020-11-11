use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::string::FromUtf8Error;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    // Generic(String),
    ParseUtf8(FromUtf8Error),
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    System(std::io::Error),
    Mpsc(std::sync::mpsc::RecvError),
    Ble(btleplug::Error),
    BleUuid(btleplug::api::ParseUUIDError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // CliError::Generic(ref e) => {
            //     log::error!("Failed with generic error: {}", e);
            //     e.fmt(f)
            // },
            CliError::ParseUtf8(ref e) => {
                log::error!("Failed to parse utf8 string");
                e.fmt(f)
            },
            CliError::ParseFloat(ref e) => {
                log::error!("Failed to parse input as floating point value");
                e.fmt(f)
            },
            CliError::ParseInt(ref e) => {
                log::error!("Failed to parse input as integer value");
                e.fmt(f)
            },
            CliError::System(ref e) => {
                log::error!("Failed to run system command");
                e.fmt(f)
            },
            CliError::Mpsc(ref e) => {
                log::error!("Failed with multithreading messaging error");
                e.fmt(f)
            }
            CliError::Ble(ref e) => {
                log::error!("Failed with ble error: {:#?}", e);
                e.fmt(f)
            }
            CliError::BleUuid(ref e) => {
                log::error!("Failed with ble uuid error: {:#?}", e);
                e.fmt(f)
            }
        }
    }
}

impl From<FromUtf8Error> for CliError {
    fn from(err: FromUtf8Error) -> CliError {
        CliError::ParseUtf8(err)
    }
}

impl From<ParseFloatError> for CliError {
    fn from(err: ParseFloatError) -> CliError {
        CliError::ParseFloat(err)
    }
}

impl From<ParseIntError> for CliError {
    fn from(err: ParseIntError) -> CliError {
        CliError::ParseInt(err)
    }
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError {
        CliError::System(err)
    }
}

impl From<std::sync::mpsc::RecvError> for CliError {
    fn from(err: std::sync::mpsc::RecvError) -> CliError {
        CliError::Mpsc(err)
    }
}

impl From<btleplug::Error> for CliError {
    fn from(err: btleplug::Error) -> CliError {
        CliError::Ble(err)
    }
}

impl From<btleplug::api::ParseUUIDError> for CliError {
    fn from(err: btleplug::api::ParseUUIDError) -> CliError {
        CliError::BleUuid(err)
    }
}