use smart_house::{PowerSocket, PowerSocketState};
use smart_house::{Room, SmartDevice, SmartDeviceList, SmartHouse};
use smart_house::{Temperature, Thermometer};

fn main() -> Result<(), &'static str> {
    //create house:
    let mut house = SmartHouse::new();

    //create thermometer
    let device1 = SmartDevice::Thermo(Thermometer {
        name: "Therm1".to_owned(),
        state: Temperature::Celsius(18.0),
    });

    //create power socket
    let device2 = SmartDevice::Socket(PowerSocket {
        name: "Socket1".to_owned(),
        description: "Power Socket".to_owned(),
        state: PowerSocketState::NotPowered,
        power_consumption: 0,
    });

    //create rooms:
    //
    let mut room1 = Room::with_name("room1");
    let room2 = Room::with_name("room2");

    //we can add devices to rooms directly:
    //note that devices are stored as text ids
    //devices as entities are stored separately - in some device list
    room1.try_add_device("Therm1")?;

    //add rooms to the house:
    //note that room names are case insensitive - rooms "Room1", "ROOM1", "room1" will be treated as same room names on insertion;
    house.try_add_room(room1)?;
    house.try_add_room(room2)?;

    //add device to the house:
    house.try_add_device("room2", "Socket1")?;

    let mut device_list = SmartDeviceList::new();
    device_list.add_device("room1", device1);
    device_list.add_device("room2", device2);

    let report = house.get_report(device_list);
    println!("{}", report);

    Ok(())
}
