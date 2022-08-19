use crate::{CommandData, CustomError, CustomResult, ExecutionResult, SmartDevice};
use dashmap::DashMap;
use std::sync::Arc;

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> CustomResult<DeviceInfo>;
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub kind: String,
    pub name: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct SmartDeviceList(Arc<DashMap<String, Vec<SmartDevice>>>);
impl Default for SmartDeviceList {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartDeviceList {
    pub fn new() -> Self {
        Self(Arc::new(DashMap::new()))
    }
    pub fn add_device(&mut self, room: &str, device: SmartDevice) -> CustomResult<()> {
        let mut mut_vec = self.0.entry(room.to_lowercase()).or_insert(Vec::new());

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
    pub fn execute_command(&self, cmd: CommandData) -> CustomResult<ExecutionResult> {
        let CommandData { device_name, data } = cmd;
        for mut room in self.0.iter_mut() {
            for device in room.iter_mut() {
                if device.get_name() == device_name {
                    return device
                        .execute_command(data)
                        .map_err(|e| CustomError::DeviceFailure(e.to_string()));
                }
            }
        }
        Err(CustomError::DeviceNotFound)
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
