use std::{error::Error, fmt::Debug, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CustomError {
    DeviceFailure(String),
    AddRoomError,
    AddDeviceError,
    DeviceNotFound,
    RoomNotFound,
    Unknown,
    CommandExecutionFailure,
}

impl Error for CustomError {}
impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CustomError::*;
        match self {
            DeviceFailure(err) => {
                write!(f, "error: {:?}", err)
            }
            AddRoomError => write!(f, "add room error"),
            AddDeviceError => write!(f, "add device error"),
            DeviceNotFound => write!(f, "device not found"),
            RoomNotFound => write!(f, "room not found"),
            CommandExecutionFailure => write!(f, "failed to execute command"),
            Unknown => write!(f, "unknown error"),
        }
    }
}
