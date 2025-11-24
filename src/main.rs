//! Weather Application
//!
//! A command-line application for fetching weather data from various providers.
//! Supports multiple weather API services and provides flexible configuration options.
//!
//! # Modules
//!
//! * `cli` - Command-line interface handling and argument parsing
//! * `config` - Configuration management and settings
//! * `providers` - Weather API provider implementations
//!
//! # Environment Variables
//!
//! The application loads environment variables from a `.env` file if present.
//! See individual provider documentation for required environment variables.
//!
//! # Example
//!
//! ```bash
//! # Run the application
//! cargo run -- --city "London"
//! ```

mod cli;
mod config;
mod providers;

use clap::Parser;

/// Main entry point of the weather application.
///
/// This function performs the following steps:
/// 1. Loads environment variables from a `.env` file (if present)
/// 2. Parses command-line arguments
/// 3. Delegates execution to the CLI handler
///
/// # Returns
///
/// * `anyhow::Result<()>` - Ok if the application runs successfully, or an error
///
/// # Errors
///
/// Returns an error if:
/// * Command-line arguments are invalid
/// * Required environment variables are missing
/// * API requests fail
/// * Any other runtime error occurs
///
/// # Example
///
/// The application is typically run from the command line:
///
/// ```bash
/// # Get current weather
/// wapp --city "New York" --data now
///
/// # Get forecast
/// wapp --city "Paris" --data forecast
/// ```
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file if it exists
    dotenvy::dotenv().ok();

    // Parse command-line arguments
    let cli = cli::Cli::parse();

    // Handle the CLI command and execute the requested operation
    cli::handle_cli(cli).await
}
