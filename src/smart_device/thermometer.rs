#[derive(Clone, Copy, Debug)]
pub enum Temperature {
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

#[derive(Clone, Debug)]
pub struct Thermometer {
    pub name: String,
    pub state: Temperature,
}

impl Thermometer {
    pub fn get_celsius(&self) -> i16 {
        self.get_temperature().as_celsius()
    }

    pub fn get_fahrenheit(&self) -> i16 {
        self.get_temperature().as_fahrenheit()
    }

    pub fn get_temperature(&self) -> Temperature {
        self.state
    }
}

#[derive(Debug, Default)]
pub struct ThermometerError {}

impl ThermometerError {
    pub fn get_message(&self) -> String {
        "thermometer error".into()
    }
}

#[cfg(test)]
mod test {
    use super::Temperature;
    #[test]
    fn temperature_conversion() {
        assert_eq!(Temperature::Celsius(-10.).as_fahrenheit(), 14);
        assert_eq!(Temperature::Fahrenheit(35.).as_fahrenheit(), 35);
        assert_eq!(Temperature::Celsius(31.).as_fahrenheit(), 88);

        assert_eq!(Temperature::Fahrenheit(-40.).as_celsius(), -40);
        assert_eq!(Temperature::Fahrenheit(0.).as_celsius(), -18);
        assert_eq!(Temperature::Fahrenheit(32.).as_celsius(), 0);
    }
}
