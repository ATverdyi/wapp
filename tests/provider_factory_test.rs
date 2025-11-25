use wapp::config::AppConfig;
use wapp::providers::provider_factory;

#[test]
fn test_weatherapi_provider_exists() {
    std::env::set_var("WEATHERAPI_KEY", "dummy");

    let cfg = AppConfig {
        provider: "weatherapi".into(),
    };

    assert!(provider_factory(&cfg).is_ok());
}

#[test]
fn test_openweather_provider_exists() {
    std::env::set_var("OPENWEATHER_KEY", "dummy");

    let cfg = AppConfig {
        provider: "openweather".into(),
    };

    assert!(provider_factory(&cfg).is_ok());
}

#[test]
fn test_invalid_provider() {
    let cfg = AppConfig {
        provider: "unknown".into(),
    };

    assert!(provider_factory(&cfg).is_err());
}
