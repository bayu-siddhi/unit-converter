use anyhow::{Result, bail};
use std::fmt::{Display, Formatter};

pub enum UnitType {
    Source,
    Target,
}

impl Display for UnitType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitType::Source => write!(f, "Source"),
            UnitType::Target => write!(f, "Target"),
        }
    }
}

#[derive(PartialEq)]
pub enum UnitDimension {
    Temperature,
    Length,
}

impl Display for UnitDimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitDimension::Temperature => write!(f, "temperature"),
            UnitDimension::Length => write!(f, "length"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Centimeter,
    Inch,
    Kilometer,
    Mile,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Celsius => write!(f, "celsius"),
            Unit::Fahrenheit => write!(f, "fahrenheit"),
            Unit::Kelvin => write!(f, "kelvin"),
            Unit::Centimeter => write!(f, "cm"),
            Unit::Inch => write!(f, "inch"),
            Unit::Kilometer => write!(f, "km"),
            Unit::Mile => write!(f, "miles"),
        }
    }
}

impl Unit {
    pub fn dimension(&self) -> UnitDimension {
        match self {
            Unit::Celsius | Unit::Fahrenheit | Unit::Kelvin => UnitDimension::Temperature,
            Unit::Centimeter | Unit::Inch | Unit::Kilometer | Unit::Mile => UnitDimension::Length,
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Unit::Celsius => return String::from("°C"),
            Unit::Fahrenheit => return String::from("°F"),
            Unit::Kelvin => return String::from("K"),
            Unit::Centimeter => return String::from("cm"),
            Unit::Inch => return String::from("inch"),
            Unit::Kilometer => return String::from("km"),
            Unit::Mile => return String::from("miles"),
        }
    }

    pub fn all_units() -> [Unit; 7] {
        [
            Unit::Celsius,
            Unit::Fahrenheit,
            Unit::Kelvin,
            Unit::Centimeter,
            Unit::Inch,
            Unit::Kilometer,
            Unit::Mile,
        ]
    }

    pub fn print() {
        println!("Supported units:");
        for (i, unit) in Self::all_units().iter().enumerate() {
            println!("{}. [{}] {}", i + 1, unit.dimension(), unit.to_string());
        }
    }
}

pub fn get_enum(unit: String, unit_type: UnitType) -> Result<Unit> {
    match unit.to_lowercase().as_str() {
        "celsius" => return Ok(Unit::Celsius),
        "fahrenheit" => return Ok(Unit::Fahrenheit),
        "kelvin" => return Ok(Unit::Kelvin),
        "cm" => return Ok(Unit::Centimeter),
        "inch" => return Ok(Unit::Inch),
        "km" => return Ok(Unit::Kilometer),
        "miles" => return Ok(Unit::Mile),
        _ => bail!(format!(
            "Error: [ERROR] {} unit '{}' not recognized.",
            unit_type.to_string(),
            unit
        )),
    }
}
