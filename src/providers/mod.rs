use async_trait::async_trait;

/// A common interface for all weather API providers.
///
/// This trait defines the unified method used by the application to fetch
/// weather information, regardless of the underlying provider.
///
/// Implementations of this trait must:
/// - perform an asynchronous HTTP request,
/// - return weather data as a `String` (raw JSON or formatted text),
/// - handle provider-specific errors internally and convert them into `anyhow::Error`.
///
/// # Method
///
/// ## `get_data`
/// Fetches weather data from the provider.
///
/// * `city` — city name provided by the user.
/// * `when` — time/data type such as `"now"`, `"forecast"`, `"tomorrow"`.
///
/// # Example
/// ```ignore
/// let provider = WeatherApiProvider::from_env()?;
/// let result = provider.get_data("London".into(), "now".into()).await?;
/// println!("{}", result);
/// ```
#[async_trait]
pub trait ApiProvider {
    /// Fetches weather data from the provider asynchronously.
    ///
    /// Returns raw response data as a `String`.
    async fn get_data(&self, city: String, when: String) -> anyhow::Result<String>;
}

/// OpenWeatherMap provider implementation.
/// Located in `providers/openweather.rs`.
pub mod openweather;

/// WeatherAPI provider implementation.
/// Located in `providers/weatherapi.rs`.
pub mod weatherapi;

/// Re-export for easier access to provider types.
pub use openweather::OpenWeatherProvider;
pub use weatherapi::WeatherApiProvider;

use crate::config::AppConfig;

/// Factory function that constructs the appropriate API provider
/// based on the application's configuration.
///
/// This is the central point of the Strategy Pattern implementation.
/// It reads the `provider` field from `AppConfig` and returns a boxed
/// instance of the correct provider implementation.
///
/// Each provider must expose a `from_env()` constructor, which loads
/// required environment variables (API key, base URL, etc.).
///
/// # Errors
/// Returns an error if:
/// - the provider name is unknown,
/// - environment variables required by the provider are missing,
/// - provider initialization fails for any other reason.
///
/// # Example
/// ```ignore
/// let cfg = load_config()?;
/// let provider = provider_factory(&cfg)?;
/// let result = provider.get_data("Tokyo".into(), "forecast".into()).await?;
/// println!("{}", result);
/// ```
pub fn provider_factory(cfg: &AppConfig) -> anyhow::Result<Box<dyn ApiProvider>> {
    match cfg.provider.as_str() {
        "weatherapi" => Ok(Box::new(WeatherApiProvider::from_env()?)),
        "openweather" => Ok(Box::new(OpenWeatherProvider::from_env()?)),
        other => Err(anyhow::anyhow!("Unsupported provider: {}", other)),
    }
}
