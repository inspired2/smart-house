mod device_info_provider;
mod error;
mod smart_device;
mod house;

pub use device_info_provider::{DeviceInfo, DeviceInfoProvider, SmartDeviceList};
pub use error::CustomError;
pub use smart_device::{
    PowerSocket, PowerSocketState, SmartDevice, SocketError, Temperature, Thermometer,
    ThermometerError,
};
pub use house::{Room, SmartHouse, CustomResult};
