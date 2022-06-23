use std::any::Any;

use smart_house::*;

fn create_house() -> SmartHouse {
    let mut house = SmartHouse::new();
    let livingrooom = Room::with_name("livingroom");
    let bedroom = Room::with_name("bedroom");
    let hall = Room::with_name("hall");
    house.try_add_room(livingrooom).unwrap();
    house.try_add_room(bedroom).unwrap();
    house.try_add_room(hall).unwrap();
    house
}

fn create_room(name: &str) -> Room {
    Room::with_name(name)
}

fn create_device(device: Box<dyn Any>) -> SmartDevice {
    SmartDevice::from(device)
}
fn create_thermometer(name: &str) -> Thermometer {
    Thermometer {
        name: name.to_string(),
        state: Temperature::Celsius(0.),
    }
}

fn create_powersocket(name: &str) -> PowerSocket {
    PowerSocket {
        name: name.to_string(),
        state: PowerSocketState::NotPowered,
        description: "no desc".into(),
        power_consumption: 0,
    }
}
fn create_devices_storage() -> impl DeviceInfoProvider {
    let mut storage = SmartDeviceList::new();

    let socket1 = create_powersocket("socket1");
    let socket2 = create_powersocket("socket2");

    let therm1 = create_thermometer("therm1");
    let therm2 = create_thermometer("therm2");

    let collection: Vec<Box<dyn Any>> = vec![
        Box::new(socket1),
        Box::new(socket2),
        Box::new(therm1),
        Box::new(therm2),
    ];

    let mut vec: Vec<SmartDevice> = collection.into_iter().map(|d| create_device(d)).collect();
    vec.drain(..).for_each(|dev| {
        storage.add_device("hall", dev).ok();
    });
    storage
}

#[test]
#[should_panic]
fn devices_are_unique() {
    let mut house = create_house();
    house.try_add_device("livingroom", "thermo1").ok();
    house
        .try_add_device("livingroom", "thermo1")
        .expect("room can contain unique devices only");
}

#[test]
fn house_contains_rooms() {
    let house = create_house();
    let rooms = house.get_rooms();
    assert!(rooms.contains(&"livingroom") && rooms.contains(&"hall") && rooms.contains(&"bedroom"));
    assert!(rooms.len() == 3);
}
#[test]
fn rooms_are_unique() {
    let mut house = create_house();
    assert!(house.get_rooms().contains(&"hall"));
    let room = create_room("hall");
    assert!(house.try_add_room(room).is_err());
}

#[test]
fn room_contains_devices() {
    let mut house = create_house();
    house.try_add_device("hall", "therm1").unwrap();
    house.try_add_device("hall", "therm2").unwrap();
    let hall_devices = house.get_devices("hall");
    assert!(hall_devices.contains(&"therm1") && hall_devices.contains(&"therm2"));
}

#[test]
fn create_report() {
    let mut house = create_house();
    house.try_add_device("hall", "therm1").unwrap();
    house.try_add_device("hall", "therm2").unwrap();
    house.try_add_device("hall", "socket1").unwrap();
    house.try_add_device("hall", "socket2").unwrap();

    let storage = create_devices_storage();
    let report = house.get_report(storage);

    assert!(report.contains("therm1"));
    assert!(report.contains("therm2"));
    assert!(report.contains("socket1"));
    assert!(report.contains("socket2"));
}

#[test]
fn report_contains_error_if_no_device() {
    let mut house = create_house();
    house.try_add_device("hall", "therm1").unwrap();
    house.try_add_device("hall", "therm2").unwrap();
    house.try_add_device("hall", "socket").unwrap();

    let storage = create_devices_storage();
    let report = house.get_report(storage);
    println!("{}", report);
    assert!(report.contains("device not found"));
}
