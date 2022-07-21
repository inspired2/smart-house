use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub enum Command {
    Exit,
    Execute(CommandData),
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceCommand {
    PowerSocket(PowerSocketCommand)
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PowerSocketCommand {
    TurnOn,
    TurnOff
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandData {
    pub device_name: String,
    pub data: DeviceCommand,
}

// impl From<String> for Command {
//     fn from(s: String) -> Self {
//         serde_json::from_str(&s).unwrap()
//     }
// }
// impl From<String> for CommandData {
//     fn from(s: String) -> Self {
//         let v: Vec<&str> = s.split("|").collect();
//         let device_name = v[0].trim().to_owned();
//         let data = v[1].trim().to_owned();
//         CommandData { device_name, data }
//     }
// }
pub trait Executable {
    fn execute(&mut self, command: DeviceCommand) -> Result<(), Box<dyn std::error::Error>>;
}