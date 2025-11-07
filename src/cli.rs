use clap::{Parser, Subcommand, arg, command};

#[derive(Parser)]
#[command(
    about = "\
        Unit Converter\n\n\
        A terminal-based application for converting temperature and length units.\\
    ",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Converts temperature or length units to other supported units
    Convert {
        /// Source unit
        #[arg(long)]
        from: String,
        /// Target unit
        #[arg(long)]
        to: String,
        /// Value to convert
        #[arg(long)]
        value: f64,
    },
    /// Displays the list of supported temperature and length units
    List,
    /// Displays the history of previous unit conversions
    History,
}
