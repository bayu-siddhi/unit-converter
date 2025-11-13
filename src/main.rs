//! # Unit Conversion CLI
//!
//! This is the binary entry point for the `unitconv` application.
//! Its primary role is to execute the main application logic defined in the `unitconv` 
//! library and handle any potential errors by printing them to standard error.

use unitconv::run;

/// The main entry point of the application.
///
/// Executes the command-line interface logic from the `run` function. If any errors
/// occur during execution, they are caught and printed to `stderr`.
/// 
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}
