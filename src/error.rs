use crate::{SocketError, ThermometerError};
use std::{error::Error, fmt::Debug, fmt::Display};
#[derive(Debug)]
pub enum CustomError<D: DeviceError + Sized> {
    DeviceFailure(D),
    AddRoomError,
    AddDeviceError,
}

impl<D> Error for CustomError<D> where D: DeviceError + Sized {}
impl<D> Display for CustomError<D>
where
    D: DeviceError,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CustomError::*;
        match self {
            DeviceFailure(err) => write!(f, "error: {:?}, message: {}", err, err.message()),
            AddRoomError => write!(f, "add room error"),
            AddDeviceError => write!(f, "add device error"),
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
