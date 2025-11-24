use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Application configuration structure.
///
/// This config controls which weather provider is currently selected.
/// It is saved to and loaded from `config.json` in the application root.
///
/// # Fields
/// - `provider`: Name of the active weather provider (e.g., `"weatherapi"`, `"openweather"`).
///
/// This struct is serializable and deserializable using Serde.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// Name of the selected weather provider.
    pub provider: String,
}

/// Path to the configuration file.
///
/// The file is stored next to the application binary.
/// No directories are created automatically.
///
/// Example: `config.json`
const CONFIG_PATH: &str = "config.json";

/// Saves the given configuration to `config.json`.
///
/// The file is written using pretty JSON formatting for readability.
///
/// # Errors
/// Returns an error if:
/// - the file cannot be written,
/// - serialization fails.
///
/// # Example
/// ```ignore
/// let cfg = AppConfig { provider: "weatherapi".into() };
/// save_config(&cfg)?;
/// ```
pub fn save_config(cfg: &AppConfig) -> anyhow::Result<()> {
    fs::write(CONFIG_PATH, serde_json::to_string_pretty(cfg)?)?;
    Ok(())
}

/// Loads the application configuration from `config.json`.
///
/// If the file does not exist, this function returns an instructional error
/// telling the user to run the `configure` command first.
///
/// # Errors
/// Returns an error if:
/// - the config file is missing,
/// - the JSON is malformed,
/// - the file cannot be read.
///
/// # Example
/// ```ignore
/// let cfg = load_config()?;
/// println!("Current provider: {}", cfg.provider);
/// ```
pub fn load_config() -> anyhow::Result<AppConfig> {
    if !Path::new(CONFIG_PATH).exists() {
        return Err(anyhow::anyhow!(
            "config.json not found. Run: wapp configure <provider>"
        ));
    }

    Ok(serde_json::from_str(&fs::read_to_string(CONFIG_PATH)?)?)
}
