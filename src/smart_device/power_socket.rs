use crate::Executable;
use crate::{DeviceCommand, PowerSocketCommand};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::command::ExecutionResult;

#[derive(Clone, Debug)]
pub struct PowerSocket {
    pub name: String,
    pub state: PowerSocketState,
    pub description: String,
    pub power_consumption: u8,
}
impl Executable for PowerSocket {
    fn execute(
        &mut self,
        command: DeviceCommand,
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        match command {
            DeviceCommand::PowerSocket(cmd) => {
                match cmd {
                    PowerSocketCommand::TurnOff => self.turn_off(),
                    PowerSocketCommand::TurnOn => self.turn_on(),
                };
                Ok(ExecutionResult::PowerSocket(self.get_state()))
            }
        }
    }
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
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PowerSocketState {
    Powered,
    NotPowered,
}

#[derive(Debug, Display, Default)]
pub struct SocketError {}
impl Error for SocketError {}
