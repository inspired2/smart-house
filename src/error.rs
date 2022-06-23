use std::{error::Error, fmt::Debug, fmt::Display};

#[derive(Debug)]
pub enum CustomError {
    DeviceFailure(Box<dyn Error + 'static>),
    AddRoomError,
    AddDeviceError,
    DeviceNotFound,
    RoomNotFound,
    Unknown,
}

impl Error for CustomError {}
impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CustomError::*;
        match self {
            DeviceFailure(err) => {
                write!(
                    f,
                    "error: {:?}, message: {:?}",
                    err,
                    err.source().unwrap().to_string()
                )
            }
            AddRoomError => write!(f, "add room error"),
            AddDeviceError => write!(f, "add device error"),
            DeviceNotFound => write!(f, "device not found"),
            RoomNotFound => write!(f, "room not found"),
            Unknown => write!(f, "unknown error"),
        }
    }
}
