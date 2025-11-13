//! # Units Definition Module
//!
//! This module defines all the supported units, their properties, and related utilities.
//! It includes enums for `Unit`, `UnitDimension`, and helpers for parsing and displaying them.

use anyhow::{Result, bail};
use std::fmt::{Display, Formatter};

/// Differentiates between a source unit and a target unit
pub enum UnitType {
    Source,
    Target,
}

impl Display for UnitType {
    /// Formats the enum into a capitalized string representation ("Source" or "Target").
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitType::Source => write!(f, "Source"),
            UnitType::Target => write!(f, "Target"),
        }
    }
}

/// Dimension category of a unit.
#[derive(PartialEq)]
pub enum UnitDimension {
    Temperature,
    Length,
}

impl Display for UnitDimension {
    /// Formats the enum into a lowercase string representation ("temperature" or "length").
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitDimension::Temperature => write!(f, "temperature"),
            UnitDimension::Length => write!(f, "length"),
        }
    }
}

/// All supported conversion units.
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
    /// Formats the unit enum into its lowercase string representation (e.g., "celsius", "km").
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
    /// Gets the physical dimension of the unit.
    ///
    /// This method returns the `UnitDimension` (e.g., `Temperature` or `Length`) for the
    /// given `Unit` instance.
    ///
    /// ## Returns
    ///
    /// A `UnitDimension` enum variant corresponding to the unit's category.
    ///
    pub fn dimension(&self) -> UnitDimension {
        match self {
            Unit::Celsius | Unit::Fahrenheit | Unit::Kelvin => UnitDimension::Temperature,
            Unit::Centimeter | Unit::Inch | Unit::Kilometer | Unit::Mile => UnitDimension::Length,
        }
    }

    /// Gets the common symbol for the unit.
    ///
    /// This method returns a string containing the standard symbol for the unit,
    /// such as "°C" for Celsius or "km" for Kilometer.
    ///
    /// ## Returns
    ///
    /// A `String` representing the unit's symbol.
    ///
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

    /// Provides a list of all supported `Unit` variants.
    ///
    /// This static method returns a fixed-size array containing one instance of every
    /// unit defined in the `Unit` enum.
    ///
    /// ## Returns
    ///
    /// An array of all `Unit` variants.
    ///
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

    /// Prints a formatted list of all supported units to the console.
    ///
    /// This static method iterates over all available units, printing each one's name
    /// and dimension in a human-readable, numbered list to standard output.
    ///
    pub fn print() {
        println!("Supported units:");
        for (i, unit) in Self::all_units().iter().enumerate() {
            println!("{}. [{}] {}", i + 1, unit.dimension(), unit.to_string());
        }
    }
}

/// Parses a string into a `Unit` enum.
///
/// The matching is case-insensitive. If the string does not match any known unit,
/// an error is returned.
///
/// ## Arguments
///
/// * `unit` - The `String` to parse.
/// * `unit_type` - The `UnitType` (Source/Target) for creating a specific error message.
///
/// ## Returns
///
/// An `anyhow::Result<Unit>` containing the corresponding `Unit` variant on success,
/// or an error if the unit is not recognized.
///
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
