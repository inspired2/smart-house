use crate::SmartDevice;
use std::collections::HashMap;

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Option<DeviceInfo>;
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
    pub fn add_device(&mut self, room: &str, device: SmartDevice) {
        self.0
            .entry(room.to_owned())
            .or_insert(Vec::new())
            .push(device);
    }
}
impl DeviceInfoProvider for SmartDeviceList {
    fn get_device_info(&self, room: &str, device: &str) -> Option<DeviceInfo> {
        let room_devices = self.0.get(room).unwrap();
        let device = room_devices
            .iter()
            .find(|&d| d.get_name().to_lowercase() == device)?;
        Some(DeviceInfo {
            kind: device.get_type(),
            name: device.get_name(),
            state: device.get_state(),
        })
    }
}
