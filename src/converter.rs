use crate::units::Unit;
use anyhow::{bail, Result};

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
