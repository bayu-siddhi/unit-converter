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
