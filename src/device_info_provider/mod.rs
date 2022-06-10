
pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Option<DeviceInfo>;
}
#[derive(Debug)]
pub struct DeviceInfo {
    pub kind: String,
    pub name: String,
    pub state: String,
}
