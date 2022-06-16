use crate::{SocketError, ThermometerError};
use std::{error::Error, fmt::Debug, fmt::Display};

#[derive(Debug)]
pub enum CustomError {
    DeviceFailure(Box<dyn DeviceError>),
    AddRoomError,
    AddDeviceError,
    DeviceNotFound,
    RoomNotFound,
}

impl Error for CustomError {}
impl Display for CustomError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CustomError::*;
        match self {
            DeviceFailure(err) => write!(f, "error: {:?}, message: {}", err, err.message()),
            AddRoomError => write!(f, "add room error"),
            AddDeviceError => write!(f, "add device error"),
            DeviceNotFound => write!(f, "device not found"),
            RoomNotFound => write!(f, "room not found"),
        }
    }
}

pub trait DeviceError: Debug {
    fn message(&self) -> String;
}

impl DeviceError for SocketError {
    fn message(&self) -> String {
        self.get_message()
    }
}

impl DeviceError for ThermometerError {
    fn message(&self) -> String {
        self.get_message()
    }
}
