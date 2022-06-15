mod power_socket;
mod thermometer;

pub use power_socket::{PowerSocket, PowerSocketState, SocketError};
pub use thermometer::{Temperature, Thermometer, ThermometerError};

#[derive(Debug)]
pub enum SmartDevice {
    Thermo(Thermometer),
    Socket(PowerSocket),
}
impl SmartDevice {
    pub fn get_name(&self) -> String {
        match self {
            SmartDevice::Socket(s) => s.name.to_owned(),
            SmartDevice::Thermo(t) => t.name.to_owned(),
        }
    }
    pub fn get_state(&self) -> String {
        match self {
            SmartDevice::Socket(s) => format!("{:?}", s.get_state()),
            SmartDevice::Thermo(t) => format!("{:?}", t.get_temperature()),
        }
    }
    pub fn get_type(&self) -> String {
        match self {
            SmartDevice::Socket(_) => "SmartSocket".to_owned(),
            SmartDevice::Thermo(_) => "SmartThermometer".to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_therm() {
        let thermometer = Thermometer {
            name: "thermometer".to_owned(),
            state: Temperature::Celsius(11.),
        };
        let device = SmartDevice::Thermo(thermometer);
        assert_eq!(device.get_name(), "thermometer");
        assert_eq!(device.get_type(), "SmartThermometer");
        assert_eq!(
            device.get_state(),
            format!("{:?}", Temperature::Celsius(11.))
        );
    }

    #[test]
    fn create_socket() {
        let socket = PowerSocket {
            name: "socket".to_owned(),
            state: PowerSocketState::NotPowered,
            power_consumption: 0,
            description: "smart power socket".to_owned(),
        };
        let device = SmartDevice::Socket(socket);
        assert_eq!(device.get_name(), "socket");
    }
}
