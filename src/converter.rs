//! # Conversion Logic Module
//!
//! This module contains the core logic for converting values between different units.
//! It validates that conversions are only attempted between units of the same dimension
//! (e.g., length to length) and then applies the appropriate mathematical formula.

use crate::units::Unit;
use anyhow::{Result, bail};

/// Validates if two units can be converted between each other.
///
/// A conversion is valid only if both units belong to the same dimension
/// (e.g., both are temperature units or both are length units).
///
/// ## Arguments
///
/// * `from` - A reference to the source `Unit`.
/// * `to` - A reference to the target `Unit`.
///
/// ## Returns
///
/// An `anyhow::Result` which is `Ok(())` if the units are compatible, or an 
/// `Err` with a descriptive message if they are not.
///
fn validate(from: &Unit, to: &Unit) -> Result<()> {
    if from.dimension() != to.dimension() {
        bail!(format!(
            "Error: [ERROR] Cannot convert between different unit categories: [{}] {} → [{}] {}",
            from.dimension(),
            from.to_string(),
            to.dimension(),
            to.to_string()
        ));
    }

    return Ok(());
}

/// Converts a value from a source unit to a target unit.
///
/// The conversion is performed in two steps:
/// 1. The source value is converted to a base unit for its dimension (Celsius 
///    for temperature, Centimeter for length).
/// 2. The value in the base unit is then converted to the target unit.
///
/// ## Arguments
///
/// * `from` - The source `Unit`.
/// * `to` - The target `Unit`.
/// * `value` - The `f64` value to convert.
///
/// ## Returns
///
/// An `anyhow::Result<f64>` containing the converted value on success,
/// or an error if the units are incompatible.
///
pub fn convert(from: &Unit, to: &Unit, value: &f64) -> Result<f64> {
    validate(&from, &to)?;
    if *from == *to {
        return Ok(*value);
    }

    let base_val: f64 = match from {
        // Temperature
        Unit::Celsius => *value, // Base
        Unit::Fahrenheit => 5.0 / 9.0 * (*value - 32.0),
        Unit::Kelvin => *value - 273.15,
        // Length
        Unit::Centimeter => *value, // Base
        Unit::Inch => *value * 2.54,
        Unit::Kilometer => *value * 100000.0,
        Unit::Mile => *value * 160930.0,
    };

    let final_val: f64 = match to {
        // Temperature
        Unit::Celsius => base_val,
        Unit::Fahrenheit => (9.0 / 5.0 * base_val) + 32.0,
        Unit::Kelvin => base_val + 273.15,
        // Length
        Unit::Centimeter => base_val,
        Unit::Inch => base_val / 2.54,
        Unit::Kilometer => base_val / 100000.0,
        Unit::Mile => base_val / 160930.0,
    };

    return Ok(final_val);
}
