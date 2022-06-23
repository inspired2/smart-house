use crate::{CustomError, CustomResult, SmartDevice};
use std::collections::HashMap;

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> CustomResult<DeviceInfo>;
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub kind: String,
    pub name: String,
    pub state: String,
}

#[derive(Default, Debug)]
pub struct SmartDeviceList(HashMap<String, Vec<SmartDevice>>);

impl SmartDeviceList {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn add_device(&mut self, room: &str, device: SmartDevice) -> CustomResult<()> {
        let mut_vec = self.0.entry(room.to_lowercase()).or_insert(Vec::new());

        match mut_vec
            .iter()
            .any(|d| d.get_name().to_lowercase() == device.get_name().to_lowercase())
        {
            false => {
                mut_vec.push(device);
                Ok(())
            }
            true => Err(CustomError::AddDeviceError),
        }
    }
}
impl DeviceInfoProvider for SmartDeviceList {
    fn get_device_info(&self, room: &str, device: &str) -> CustomResult<DeviceInfo> {
        let room_devices = self
            .0
            .get(&room.to_lowercase())
            .ok_or(CustomError::RoomNotFound)?;
        let device = room_devices
            .iter()
            .find(|&d| d.get_name().to_lowercase() == device)
            .ok_or(CustomError::DeviceNotFound)?;

        Ok(DeviceInfo {
            kind: device.get_type(),
            name: device.get_name(),
            state: device.get_state(),
        })
    }
}
