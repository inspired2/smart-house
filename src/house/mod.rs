use crate::{device_info_provider::DeviceInfoProvider, CustomError};
use std::collections::HashSet;

pub type CustomResult<T> = Result<T, CustomError>;

#[derive(Debug)]
pub struct Room {
    name: String,
    devices: HashSet<String>,
}
impl Room {
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            devices: HashSet::new(),
        }
    }
    pub fn try_add_device(&mut self, name: &str) -> CustomResult<()> {
        if let false = self.devices.insert(name.to_lowercase()) {
            return Err(CustomError::AddDeviceError);
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct SmartHouse {
    rooms: Vec<Room>,
}

impl SmartHouse {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }
    pub fn get_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(|r| r.name.as_str()).collect()
    }

    pub fn try_add_room(&mut self, room: Room) -> CustomResult<()> {
        if self
            .rooms
            .iter()
            .any(|r| r.name.to_lowercase() == room.name.to_lowercase())
        {
            return Err(CustomError::AddRoomError);
        }
        self.rooms.push(room);
        Ok(())
    }
    fn get_room_mut(&mut self, room_name: &str) -> Option<&mut Room> {
        self.rooms
            .iter_mut()
            .find(|r| r.name.to_lowercase() == room_name.to_lowercase())
    }
    pub fn get_devices(&self, room: &str) -> Vec<&str> {
        let devices = Vec::new();
        let room = self.rooms.iter().find(|&r| r.name == room);
        if room.is_none() {
            return devices;
        };
        room.unwrap().devices.iter().map(|d| d.as_str()).collect()
    }
    pub fn get_report<T: DeviceInfoProvider>(&self, provider: T) -> String {
        let mut report = String::new();
        for &room in self.get_rooms().iter() {
            for device in self.get_devices(room) {
                let device_info: String = provider
                    .get_device_info(room, device)
                    .map(|i| format!("{:?}", i))
                    .unwrap_or_else(|err| err.to_string());
                report += &format!("room: {}, device: {}\n", room, device_info);
            }
        }
        report
    }
    pub fn try_add_device<'a, 'b>(
        &'b mut self,
        room: &'a str,
        device: &'a str,
    ) -> CustomResult<()> {
        if let Some(room) = self.get_room_mut(room) {
            return room.try_add_device(device);
        }
        Err(CustomError::AddDeviceError)
    }
}