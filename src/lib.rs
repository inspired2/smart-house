#![allow(unused)]

use std::{
    hash::Hash,
    collections::{HashMap, HashSet},
    io::Error,
};

struct SmartHouse {
    rooms: HashSet<Room>,
}

struct Room {
    name: String,
    devices: Vec<String>
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Room {}
impl Hash for Room {

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}


impl SmartHouse {
    fn new() -> Self {
        Self {
            rooms: HashSet::new(),
        }
    }
    fn get_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(|r| r.name.as_str()).collect()
    }
    fn add_room(&mut self, room: Room) -> bool {
        self.rooms.insert(room)
    }
    fn get_devices(&self, room: &str) -> Vec<SmartDevice> {
        todo!()
    }
    fn get_report<T: DeviceInfoProvider>(&self, provider: T) -> SmartHouseReport {
        todo!()
    }
}
enum SmartDevice {
    Thermo(Thermometer),
    Socket(PowerSocket)
}
impl SmartDevice {
    fn get_name(&self) -> String {
        match self {
            SmartDevice::Socket(s) => s.name.to_owned(),
            SmartDevice::Thermo(t) => t.name.to_owned()
        }
    }
    fn get_state(&self) -> String {
                match self {
            SmartDevice::Socket(s) => format!("{:?}", s.get_state()),
            SmartDevice::Thermo(t) => format!("{:?}", t.get_temperature())}
        }
    }

struct DeviceInfo {
    name: String,
    state: String,
}
struct SmartDeviceList(HashMap<String, Vec<SmartDevice>>);
impl DeviceInfoProvider for SmartDeviceList {
    fn get_device_info(&self, room: &str, device: &str) -> Option<DeviceInfo> {
        let room_devices = self.0.get(room).unwrap();
        let device = room_devices.iter().find(|&d| d.get_name() == device);
        if device.is_none() { return None };
        Some(DeviceInfo {
            name: device.unwrap().get_name(),
            state: device.unwrap().get_state()
        })
    }
}
trait DeviceInfoProvider {
    fn get_device_info(&self, device_name: &str, room_name: &str) -> Option<DeviceInfo>;
}
struct SmartHouseReport {
    inner: HashMap<String, Vec<HashMap<String, String>>>
}


#[derive(Clone)]
struct Thermometer {
    name: String,
    state: Temperature,
}

impl Thermometer {
    pub fn get_celsius(&self) -> i16 {
        self.get_temperature().as_celsius()
    }

    pub fn get_fahrenheit(&self) -> i16 {
        self.get_temperature().as_fahrenheit()
    }

    fn get_temperature(&self) -> Temperature {
        self.state
    }
}
#[derive(Clone)]
struct PowerSocket {
    name: String,
    state: PowerSocketState,
    description: String,
    power_consumption: u8,
}
impl PowerSocket {
    pub fn get_power_consumption(&self) -> u8 {
        self.power_consumption
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn turn_on(&mut self) {
        if self.is_turned_on() {
            return;
        }
        self.state = PowerSocketState::Powered
    }

    pub fn turn_off(&mut self) {
        if self.is_turned_on() {
            self.state = PowerSocketState::NotPowered
        }
    }

    fn get_state(&self) -> PowerSocketState {
        self.state
    }

    fn is_turned_on(&self) -> bool {
        matches!(self.state, PowerSocketState::Powered)
    }
}
#[derive(Clone, Copy, Debug)]
enum PowerSocketState {
    Powered,
    NotPowered,
}

/// # Temperature conversion test
/// ```
/// use smart_house::Temperature;
///
/// assert_eq!(Temperature::Celsius(-10.).as_fahrenheit(), 14);
/// assert_eq!(Temperature::Fahrenheit(35.).as_fahrenheit(), 35);
/// assert_eq!(Temperature::Celsius(31.).as_fahrenheit(), 88);
///
/// assert_eq!(Temperature::Fahrenheit(-40.).as_celsius(), -40);
/// assert_eq!(Temperature::Fahrenheit(0.).as_celsius(), -18);
/// assert_eq!(Temperature::Fahrenheit(32.).as_celsius(), 0);
/// ```
#[derive(Clone, Copy, Debug)]
enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}

impl Temperature {
    pub fn as_celsius(&self) -> i16 {
        match *self {
            Temperature::Celsius(c) => c.round() as i16,
            Temperature::Fahrenheit(f) => (((f - 32.0) * 5.0) / 9.0).round() as i16,
        }
    }

    pub fn as_fahrenheit(&self) -> i16 {
        match *self {
            Temperature::Fahrenheit(f) => f.round() as i16,
            Temperature::Celsius(c) => (c * 1.8 + 32.0).round() as i16,
        }
    }
}
