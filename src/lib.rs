mod device_info_provider;
mod error;
mod house;
mod smart_device;

pub use device_info_provider::{DeviceInfoProvider, SmartDeviceList};
pub use house::{Room, SmartHouse};
pub use smart_device::{
    Device, PowerSocket, PowerSocketState, SmartDevice, Temperature, Thermometer,
};

use error::CustomError;
use house::CustomResult;
