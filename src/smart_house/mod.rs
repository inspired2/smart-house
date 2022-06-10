use crate::device_info_provider::DeviceInfoProvider;
use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct SmartHouse {
    rooms: Vec<Room>,
}

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
    pub fn try_add_device(&mut self, name: &str) -> Result<(), &'static str> {
        if let false = self.devices.insert(name.to_lowercase()) {
            return Err("device with the same name already present");
        }
        Ok(())
    }
}

impl SmartHouse {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }
    fn get_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(|r| r.name.as_str()).collect()
    }

    pub fn try_add_room(&mut self, room: Room) -> Result<(), &'static str> {
        if self
            .rooms
            .iter()
            .any(|r| r.name.to_lowercase() == room.name.to_lowercase())
        {
            return Err("room with similiar name already in the house");
        }
        self.rooms.push(room);
        Ok(())
    }
    fn get_room_mut(&mut self, room_name: &str) -> Option<&mut Room> {
        self.rooms
            .iter_mut()
            .find(|r| r.name.to_lowercase() == room_name.to_lowercase())
    }
    fn get_devices(&self, room: &str) -> Vec<String> {
        let devices = Vec::new();
        let room = self.rooms.iter().find(|&r| r.name == room);
        if room.is_none() {
            return devices;
        };
        room.unwrap().devices.iter().map(|d| d.to_owned()).collect()
    }
    pub fn get_report<T: DeviceInfoProvider>(&self, provider: T) -> String {
        let mut report = String::new();
        for &room in self.get_rooms().iter() {
            for device in self.get_devices(room) {
                let device_info: String = provider
                    .get_device_info(room, &device)
                    .map(|i| format!("{:?}", i))
                    .unwrap_or_else(|| "device not found".to_string());
                report += &format!("room: {}, device: {}\n", room, device_info);
            }
        }
        report
    }
    pub fn try_add_device<'a, 'b>(
        &'b mut self,
        room: &'a str,
        device: &'a str,
    ) -> Result<(), &'static str> {
        if let Some(room) = self.get_room_mut(room) {
            return room.try_add_device(device);
        }
        Err("no such room")
    }
}
