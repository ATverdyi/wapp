use std::fs;
use wapp::config::{load_config, save_config, AppConfig};

#[test]
fn test_save_and_load_config() {
    let cfg = AppConfig {
        provider: "weatherapi".into(),
    };

    save_config(&cfg).unwrap();
    let loaded = load_config().unwrap();

    assert_eq!(loaded.provider, "weatherapi");

    fs::remove_file("config.json").unwrap();
}

#[test]
fn test_missing_config() {
    let _ = fs::remove_file("config.json");
    let result = load_config();
    assert!(result.is_err());
}
