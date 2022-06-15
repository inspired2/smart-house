#[derive(Clone, Debug)]
pub struct PowerSocket {
    pub name: String,
    pub state: PowerSocketState,
    pub description: String,
    pub power_consumption: u8,
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

    pub fn get_state(&self) -> PowerSocketState {
        self.state
    }

    fn is_turned_on(&self) -> bool {
        matches!(self.state, PowerSocketState::Powered)
    }
}
#[derive(Clone, Copy, Debug)]
pub enum PowerSocketState {
    Powered,
    NotPowered,
}

#[derive(Debug, Default)]
pub struct SocketError {}

impl SocketError {
    pub fn get_message(&self) -> String {
        "socket error".into()
    }
}
