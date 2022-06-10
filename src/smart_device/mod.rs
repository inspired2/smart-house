mod power_socket;
mod thermometer;

use crate::device_info_provider::{DeviceInfo, DeviceInfoProvider};

pub use power_socket::{PowerSocket, PowerSocketState};
pub use thermometer::{Temperature, Thermometer};

use std::collections::HashMap;

#[derive(Debug)]
pub enum SmartDevice {
    Thermo(Thermometer),
    Socket(PowerSocket),
}
impl SmartDevice {
    fn get_name(&self) -> String {
        match self {
            SmartDevice::Socket(s) => s.name.to_owned(),
            SmartDevice::Thermo(t) => t.name.to_owned(),
        }
    }
    fn get_state(&self) -> String {
        match self {
            SmartDevice::Socket(s) => format!("{:?}", s.get_state()),
            SmartDevice::Thermo(t) => format!("{:?}", t.get_temperature()),
        }
    }
    fn get_type(&self) -> String {
        match self {
            SmartDevice::Socket(_) => "SmartSocket".to_owned(),
            SmartDevice::Thermo(_) => "SmartThermometer".to_owned(),
        }
    }
}

#[derive(Default)]
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
        let device = room_devices.iter().find(|&d| d.get_name() == device)?;
        Some(DeviceInfo {
            kind: device.get_type(),
            name: device.get_name(),
            state: device.get_state(),
        })
    }
}
