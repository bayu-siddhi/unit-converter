//! # `unitconv` Library
//!
//! This crate contains the core logic for the unit conversion application.
//! It handles command-line argument parsing, dispatches commands, performs
//! conversions, and manages conversion history.

mod cli;
mod converter;
mod history;
mod units;

use crate::cli::{Cli, Commands};
use crate::converter::convert;
use crate::history::History;
use crate::units::{Unit, UnitType, get_enum};
use anyhow::{Context, Result};
use clap::Parser;

/// Runs the main application logic.
///
/// This function parses command-line arguments, loads the conversion history,
/// and executes the appropriate command (convert, list, or history).
/// If a conversion is performed, it saves the updated history to a file.
/// 
pub fn run() -> Result<()> {
    let cli: Cli = Cli::parse();
    let mut history: History = History::load().unwrap_or_default();
    let mut updated: bool = false;

    match cli.command {
        Commands::Convert { from, to, value } => {
            handle_convert(from, to, value, &mut history, &mut updated)?;
        }
        Commands::List => Unit::print(),
        Commands::History => history.print()?,
    }

    if updated {
        history
            .save()
            .context("Failed to save conversion history.")?;
    }

    return Ok(());
}

/// Formats a floating-point value into a cleaned-up string.
///
/// - Target values are formatted to a maximum of 4 decimal places.
/// - Trailing zeros and unnecessary decimal points are removed.
/// - Ensures that whole numbers are formatted with `.0`.
///
/// ## Arguments
///
/// * `value` - The `f64` value to format.
/// * `unit_type` - The type of unit (`Source` or `Target`) to determine 
///    formatting precision.
///
/// ## Returns
///
/// A formatted `String`.
/// 
fn format_value(value: f64, unit_type: UnitType) -> String {
    let mut str_value: String = match unit_type {
        UnitType::Source => value.to_string(),
        UnitType::Target => format!("{:.4}", value).to_string(),
    };

    if str_value.contains('.') {
        str_value = str_value.trim_end_matches('0').to_string();
        if str_value.ends_with('.') {
            str_value.push('0');
        }
    } else {
        str_value.push_str(".0");
    }

    return str_value;
}

/// Handles the 'convert' command logic.
///
/// It parses the source and target units, performs the conversion,
/// prints the result to the console, and adds the result to the history.
///
/// ## Arguments
///
/// * `from` - The string representation of the source unit.
/// * `to` - The string representation of the target unit.
/// * `value` - The numerical value to be converted.
/// * `history` - A mutable reference to the `History` struct.
/// * `updated` - A mutable boolean flag to indicate if the history was modified.
///
/// ## Returns
///
/// An `anyhow::Result` indicating success or failure.
/// 
fn handle_convert(
    from: String,
    to: String,
    value: f64,
    history: &mut History,
    updated: &mut bool,
) -> Result<()> {
    let from: Unit = get_enum(from, UnitType::Source)?;
    let to: Unit = get_enum(to, UnitType::Target)?;
    let conv_value: f64 = convert(&from, &to, &value)?;

    let str_result: String = format!(
        "{} {} = {} {}",
        format_value(value, UnitType::Source),
        from.symbol(),
        format_value(conv_value, UnitType::Target),
        to.symbol()
    )
    .to_string();

    println!("{}", &str_result);
    (*history).add(str_result);
    *updated = true;

    return Ok(());
}
