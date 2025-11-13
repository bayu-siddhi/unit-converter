//! # Command-Line Interface Module
//!
//! This module defines the structure of the command-line interface using the `clap` crate.
//! It specifies the main commands (`convert`, `list`, `history`) and their arguments.

use clap::{Parser, Subcommand, arg, command};

/// The main command-line interface structure.
#[derive(Parser)]
#[command(
    about = "\
        Unit Converter\n\n\
        A terminal-based application for converting temperature and length units.\
    ",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// The subcommands for the application.
#[derive(Subcommand)]
pub enum Commands {
    /// Converts temperature or length units to other supported units
    Convert {
        /// Source unit (e.g., celsius, km).
        #[arg(long)]
        from: String,
        /// Target unit (e.g., fahrenheit, miles).
        #[arg(long)]
        to: String,
        /// The numerical value to convert.
        #[arg(long)]
        value: f64,
    },
    /// Displays the list of supported temperature and length units
    List,
    /// Displays the history of previous unit conversions
    History,
}
