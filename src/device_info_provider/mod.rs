use crate::{CustomError, CustomResult, SmartDevice, Command, CommandData};
use dashmap::{DashMap, mapref::one::RefMut};
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

#[derive(Debug)]
pub struct SmartDeviceList(Arc<DashMap<String, Vec<SmartDevice>>>);

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
    pub fn room_with_device_ref_mut(&mut self, device_name: &str) -> Option<(RefMut<'_,String, Vec<SmartDevice>>, usize)> {
        for room in self.0.iter() {
            for (idx, device) in room.iter().enumerate() {
                if device.get_name() == device_name {
                    return Some((self.0.get_mut(room.key()).unwrap(), idx))
                }
            }
        }
        None
    }
    pub fn execute_command(&mut self, cmd: CommandData) -> CustomResult<()> {
        match cmd {
            CommandData {device_name, data } => {
                if let Some((mut room, idx)) = self.room_with_device_ref_mut(&device_name) {
                    //SAFETY: already checked that device is present at idx
                    let device = unsafe { room.get_unchecked_mut(idx)};
                    //device.change_state(data)
                }
            }
        }
        Ok(())
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
