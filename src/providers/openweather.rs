use anyhow::Result;
use async_trait::async_trait;
use std::env;
use urlencoding::encode;

use super::ApiProvider;

/// Provider for working with the OpenWeatherMap API.
///
/// This provider offers access to weather data through the OpenWeatherMap API,
/// supporting various types of queries (current weather, forecast).
///
/// # Fields
///
/// * `api_key` - API key for authentication with OpenWeatherMap
/// * `base_url` - Base URL of the API
/// * `units` - Units of measurement (metric, imperial, standard)
/// * `lang` - Language of API response
pub struct OpenWeatherProvider {
    pub api_key: String,
    pub base_url: String,
    pub units: Option<String>,
    pub lang: Option<String>,
}

impl OpenWeatherProvider {
    /// Creates a new instance of `OpenWeatherProvider` from environment variables.
    ///
    /// # Environment Variables
    ///
    /// * `OPENWEATHER_KEY` (required) - OpenWeatherMap API key
    /// * `OPENWEATHER_BASE_URL` (optional) - API base URL (default: "https://api.openweathermap.org/data/3.0")
    /// * `OPENWEATHER_UNITS` (optional) - Units of measurement (metric/imperial/standard)
    /// * `OPENWEATHER_LANG` (optional) - Response language code (e.g., "en", "uk", "es")
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - A new provider instance or an error if required variables are missing
    ///
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            api_key: env::var("OPENWEATHER_KEY")?,
            base_url: env::var("OPENWEATHER_BASE_URL")
                .unwrap_or("https://api.openweathermap.org/data/3.0".into()),
            units: env::var("OPENWEATHER_UNITS").ok(),
            lang: env::var("OPENWEATHER_LANG").ok(),
        })
    }
}

#[async_trait]
impl ApiProvider for OpenWeatherProvider {
    /// Retrieves weather data for the specified city.
    ///
    /// # Arguments
    ///
    /// * `city` - Name of the city to fetch data for
    /// * `kind` - Type of request:
    ///   - "now" - current weather
    ///   - "forecast" - weather forecast
    ///   - "tomorrow" - tomorrow's forecast (uses the same endpoint as "forecast")
    ///
    /// # Returns
    ///
    /// * `Result<String>` - JSON response from the API as a string, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * An unknown request type (`kind`) is specified
    /// * The HTTP request fails
    /// * Reading the response text fails
    ///
    /// # Example
    async fn get_data(&self, city: String, kind: String) -> Result<String> {
        // Encode city name for safe use in URL
        let city = encode(&city);

        // Build URL based on request type
        let url = match kind.as_str() {
            "now" => {
                let mut url = format!(
                    "{}/weather?q={}&appid={}",
                    self.base_url, city, self.api_key
                );

                // Add units of measurement if specified
                if let Some(units) = &self.units {
                    url.push_str("&units=");
                    url.push_str(units);
                }

                // Add response language if specified
                if let Some(lang) = &self.lang {
                    url.push_str("&lang=");
                    url.push_str(lang);
                }

                url
            }

            "forecast" | "tomorrow" => {
                let mut url = format!(
                    "{}/forecast?q={}&appid={}",
                    self.base_url, city, self.api_key
                );

                // Add units of measurement if specified
                if let Some(units) = &self.units {
                    url.push_str("&units=");
                    url.push_str(units);
                }

                // Add response language if specified
                if let Some(lang) = &self.lang {
                    url.push_str("&lang=");
                    url.push_str(lang);
                }

                url
            }

            // Return error for unknown request types
            _ => return Err(anyhow::anyhow!("Unknown data type: {}", kind)),
        };

        // Execute HTTP request and return response text
        Ok(reqwest::get(url).await?.text().await?)
    }
}
