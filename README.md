# WApp – Weather CLI Tool

A cross-platform command-line application written in Rust that fetches weather data from WeatherAPI or OpenWeatherMap, using a Strategy pattern to unify requests.

MyApp works on:
- Linux
- macOS (Intel & Apple Silicon)
- Windows

---

## Features

- Unified get command for all providers
- Multiple providers (WeatherAPI, OpenWeatherMap)
- Supported data types: now, forecast, tomorrow
- Async HTTP client (reqwest + tokio)
- Provider selection via configure command
- API keys stored in environment variables (no secrets in configs)
- Cross-platform builds via GitHub Actions

---

## Installation

Download the latest binaries from the Releases:

https://github.com/<yourname>/<yourrepo>/releases

### Linux

    chmod +x myapp-linux
    ./wapp-linux --help

### macOS

    chmod +x myapp-macos-x86_64
    ./wapp-macos-x86_64 --help

### Windows

    wapp-windows.exe --help

---

## Environment Variables

### WeatherAPI

    export WEATHERAPI_KEY="your_key"
    export WEATHERAPI_BASE_URL="https://api.weatherapi.com/v1"
    export WEATHERAPI_LANG="en"

### OpenWeatherMap

    export OPENWEATHER_KEY="your_key"
    export OPENWEATHER_BASE_URL="https://api.openweathermap.org/data/2.5"
    export OPENWEATHER_LANG="en"
    export OPENWEATHER_UNITS="metric"

Windows users: set these through System Environment Variables.

---

## Configure Provider

    wapp configure weatherapi

or

    wapp configure openweather

This generates a minimal config:

    { "provider": "weatherapi" }

---

## Usage

### Current weather

    wapp get --city "New York"

### Forecast

    wapp get --city London --data forecast

### Tomorrow

    wapp get --city Paris --data tomorrow

### Help

    wapp --help
    wapp get --help

---

## Build from Source

### Linux/macOS

    cargo build --release

### Windows

    cargo build --release

---

## Cross-Compilation

### Linux → Windows

    rustup target add x86_64-pc-windows-gnu
    sudo apt install mingw-w64
    cargo build --target x86_64-pc-windows-gnu --release

### macOS → Windows

    rustup target add x86_64-pc-windows-gnu
    brew install mingw-w64
    cargo build --target x86_64-pc-windows-gnu --release

### Windows → Linux

    rustup target add x86_64-unknown-linux-musl
    cargo build --target x86_64-unknown-linux-musl --release

---

## Project Structure

    src/
      main.rs
      cli.rs
      config.rs
      providers/
          mod.rs
          weatherapi.rs
          openweather.rs

---

## License

MIT License

---

## Contributing

Pull requests are welcome. Please open an issue first to discuss changes.
