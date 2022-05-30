#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

trait Info {
    fn get_device_info(&self, device_name: &str, room_name: &str) -> String;
    fn get_devices(&self, room_name: &str) -> Vec<Device>;
}
impl House {
    fn get_report<T: Info>(&self, provider: T) -> Vec<String> {
        let mut output = Vec::new();
        for room in self.rooms.iter() {
            let devices = provider.get_devices(room);
            for device in devices {
                output.push(device.get_state())
            }
        }
        output
    }
}
struct House {
    rooms: HashSet<String>,
}
struct Room {
    name: String,
    devices: Vec<String>
}
struct Devices {
    //room => devices
    list: HashMap<String, Vec<Device>>,
}

impl Info for Devices {
    fn get_devices(&self, room_name: &str) -> Vec<Device> {
        self.list.get(room_name).unwrap().to_owned()
    }
    fn get_device_info(&self, device_name: &str, room_name: &str) -> String {
        let ents = self.list.iter();
        for (room, devices) in ents {
            if room == room_name {
                return devices
                .iter()
                .find(|&dev| dev.get_name() == device_name)
                .unwrap()
                .get_state();
            }
        }
        "Error: no device with such credentials".into()
        //iter self
        //match each device and f
    }
}
trait DeviceState {
    fn get_state(&self) -> String;
}
#[derive(Clone)]
enum Device {
    Thermo(Thermometer),
    PowerSock(PowerSocket),
}
impl Device {
    fn get_name(&self) -> String {
        match self {
            Device::Thermo(t) => t.name.to_string(),
            Device::PowerSock(p) => p.name.to_string(),
        }
    }
}
impl DeviceState for Device {
    fn get_state(&self) -> String {
        match self {
            Device::Thermo(t) => t.get_celsius().to_string(),
            Device::PowerSock(s) => format!("{:?}", s.get_state()),
        }
    }
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
