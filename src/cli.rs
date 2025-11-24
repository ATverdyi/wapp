use crate::config::{save_config, AppConfig};
use clap::{Parser, Subcommand};

/// List of supported weather API providers.
///
/// The CLI validates the provider name during the `configure` command.
/// Add new providers here when extending the application.
const SUPPORTED_PROVIDERS: &[&str] = &["weatherapi", "openweather"];

/// Main CLI entry point for the application.
///
/// This CLI supports two main commands:
/// - `configure`: Selects and saves the weather provider.
/// - `get`: Fetches weather data from the configured provider.
///
/// Example:
/// ```bash
/// wapp configure weatherapi
/// wapp get --city "New York" --data now
/// ```
#[derive(Parser)]
#[command(name = "wapp")]
pub struct Cli {
    /// Subcommands available in the CLI.
    #[command(subcommand)]
    pub cmd: Commands,
}

/// Defines all possible subcommands for the CLI.
///
/// # Commands
///
/// ## Configure
/// Saves the chosen weather provider into the application config.
/// Only providers from `SUPPORTED_PROVIDERS` are accepted.
///
/// ## Get
/// Fetches weather data from the configured provider.
/// Requires:
/// - `--city` — name of the city (mandatory)
/// - `--data` — type of weather data (default: "now")
#[derive(Subcommand)]
pub enum Commands {
    /// Configure the weather provider.
    ///
    /// Example:
    /// ```bash
    /// wapp configure openweather
    /// ```
    Configure {
        /// Provider name (must match SUPPORTED_PROVIDERS)
        provider: String,
    },

    /// Get weather data from the configured provider.
    ///
    /// Example:
    /// ```bash
    /// wapp get --city "Los Angeles" --data forecast
    /// ```
    Get {
        /// City name (required).
        /// If missing, the CLI prints an error and exits.
        #[arg(long)]
        city: Option<String>,

        /// Type of weather data.
        /// Supported values depend on the provider (but usually "now", "forecast", "tomorrow").
        /// Defaults to `"now"`.
        #[arg(long, default_value = "now")]
        data: String,
    },
}

/// Handles CLI execution logic.
///
/// This function executes the appropriate action based on the given subcommand:
///
/// - `configure`: Saves the chosen provider to the config file.
/// - `get`: Loads config, resolves provider implementation, fetches weather data.
///
/// Returns `anyhow::Result<()>` to allow flexible error handling.
///
/// # Errors
///
/// This function may exit the process if:
/// - The provider is not supported
/// - The city parameter is missing
/// - Config loading fails
/// - Provider initialization fails
/// - API request fails
pub async fn handle_cli(cli: Cli) -> anyhow::Result<()> {
    match cli.cmd {
        Commands::Configure { provider } => {
            // Validate provider name.
            if !SUPPORTED_PROVIDERS.contains(&provider.as_str()) {
                eprintln!("Error: provider '{}' is not supported.", provider);
                eprintln!("Supported providers: {}", SUPPORTED_PROVIDERS.join(", "));
                std::process::exit(1);
            }

            // Save provider into configuration.
            let cfg = AppConfig { provider };
            let _ = save_config(&cfg);
            println!("Provider saved");
        }

        Commands::Get { city, data } => {
            // City must be provided.
            let city = match city {
                Some(c) => c,
                None => {
                    eprintln!("Error: city is required. Use --city <NAME>");
                    std::process::exit(1);
                }
            };

            // Load configuration file.
            let cfg = crate::config::load_config()?;

            // Create provider instance (strategy pattern).
            let provider = crate::providers::provider_factory(&cfg)?;

            // Perform API request.
            let response = provider.get_data(city, data).await?;

            // Print raw provider response.
            println!("{}", response)
        }
    }

    Ok(())
}
