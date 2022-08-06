use serde::{Deserialize, Serialize};

use crate::PowerSocketState;

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Exit,
    Execute(CommandData),
    Unknown,
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum DeviceCommand {
    PowerSocket(PowerSocketCommand),
}
impl DeviceCommand {
    fn from_u8(n: u8) -> Result<Self, Box<dyn std::error::Error>> {
        let s = n.to_string();
        let mut iter = s.chars().rev();
        let cmd_code = iter.next().unwrap().to_digit(10).unwrap();
        let device_code = iter.next().unwrap().to_digit(10).unwrap();
        match (device_code, cmd_code) {
            (1, _) => Ok(DeviceCommand::PowerSocket(PowerSocketCommand::from_u8(
                cmd_code as u8,
            )?)),
            _ => Err("Unknown device code in command code".into()),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum PowerSocketCommand {
    TurnOn,
    TurnOff,
    GetState
}
impl PowerSocketCommand {
    fn from_u8(n: u8) -> Result<Self, Box<dyn std::error::Error>> {
        let cmd: Self = 
        match n {
            2 => Self::GetState,
            1 => Self::TurnOn,
            0 => Self::TurnOff,
            _ => return Err("Unknown PowerSocketCommand code".into()),
        };
        Ok(cmd)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandData {
    pub device_name: String,
    pub data: DeviceCommand,
}

impl From<(String, u8)> for Command {
    fn from(key_code: (String, u8)) -> Self {
        match key_code.0 {
            s if s.is_empty() || &s.to_lowercase() == "exit" => Command::Exit,
            s => {
                if let Ok(data) = DeviceCommand::from_u8(key_code.1) {
                    Command::Execute(CommandData { device_name: s, data })
                } else {
                    Command::Unknown
                }
            }
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ExecutionResult {
    PowerSocket(PowerSocketState),
    Error(crate::error::CustomError),
}
pub trait Executable {
    fn execute(
        &mut self,
        command: DeviceCommand,
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>>;
}
