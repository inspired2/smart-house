use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum CustomError {
    #[error("Error in device: {0}")]
    DeviceFailure(String),
    #[error("Cannot add room")]
    AddRoomError,
    #[error("Cannot add device")]
    AddDeviceError,
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Room not found")]
    RoomNotFound,
    #[error("Unknown error")]
    Unknown,
    #[error("Failed to execute command. Message: {0}")]
    CommandExecutionFailure(String),
}
