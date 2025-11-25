use anyhow::Result;
use async_trait::async_trait;
use std::env;
use urlencoding::encode;

use super::ApiProvider;

/// Provider for working with the WeatherAPI service.
///
/// This provider offers access to weather data through the WeatherAPI,
/// supporting current weather and forecast queries.
///
/// # Fields
///
/// * `api_key` - API key for authentication with WeatherAPI
/// * `base_url` - Base URL of the API
/// * `lang` - Language of API response
pub struct WeatherApiProvider {
    pub api_key: String,
    pub base_url: String,
    pub lang: Option<String>,
}

impl WeatherApiProvider {
    /// Creates a new instance of `WeatherApiProvider` from environment variables.
    ///
    /// # Environment Variables
    ///
    /// * `WEATHERAPI_KEY` (required) - WeatherAPI API key
    /// * `WEATHERAPI_BASE_URL` (optional) - API base URL (default: "https://api.weatherapi.com/v1")
    /// * `WEATHERAPI_LANG` (optional) - Response language code (e.g., "en", "uk", "es")
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - A new provider instance or an error if required variables are missing
    ///
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            api_key: env::var("WEATHERAPI_KEY")?,
            base_url: env::var("WEATHERAPI_BASE_URL")
                .unwrap_or("https://api.weatherapi.com/v1".into()),
            lang: env::var("WEATHERAPI_LANG").ok(),
        })
    }
}

#[async_trait]
impl ApiProvider for WeatherApiProvider {
    /// Retrieves weather data for the specified city.
    ///
    /// # Arguments
    ///
    /// * `city` - Name of the city to fetch data for
    /// * `kind` - Type of request:
    ///   - "now" - current weather
    ///   - "forecast" - weather forecast for 3 days
    ///   - "tomorrow" - tomorrow's forecast (1 day)
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
    async fn get_data(&self, city: String, kind: String) -> Result<String> {
        // Encode city name for safe use in URL
        let city = encode(&city);

        // Build URL based on request type
        let url = match kind.as_str() {
            "now" => {
                let mut url = format!(
                    "{}/current.json?key={}&q={}",
                    self.base_url, self.api_key, city
                );

                // Add response language if specified
                if let Some(lang) = &self.lang {
                    url.push_str("&lang=");
                    url.push_str(lang);
                }

                url
            }

            "forecast" | "tomorrow" => {
                // Set forecast days: 1 for tomorrow, 3 for general forecast
                let days = if kind == "tomorrow" { 1 } else { 3 };

                let mut url = format!(
                    "{}/forecast.json?key={}&q={}&days={}",
                    self.base_url, self.api_key, city, days
                );

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
