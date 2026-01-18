# nurv_config

A Rust wrpper library for config-rs to manage YAML configuration files in a structured way. It provides a simple API to load and merge configuration from multiple YAML files based on environment variables.

## Features

- Load configuration from YAML files
- Environment-specific overrides (e.g., `local.yaml`, `production.yaml`)
- Type-safe deserialization using Serde
- Convenient macro for quick loading with panic-on-error
- Flexible API for custom error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nurv_config = { git = "https://github.com/bugyboo/nurv_config.git" }
serde = { version = "1.0", features = ["derive"] }
```

## Usage

### Basic Setup

1. Create a `config/` directory in your project root.
2. Add a `base.yaml` file with your default configuration.
3. Optionally, add environment-specific files like `local.yaml`, `production.yaml`, etc.

Example `config/base.yaml`:

```yaml
app:
  name: "My App"
  port: 8080
```

Example `config/local.yaml`:

```yaml
app:
  port: 3000
database:
  url: "localhost:5432"
```

### Loading Configuration

#### Using the Macro (Recommended for Simple Cases)

The `load_config!` macro provides a convenient way to load configuration. It panics on failure, so use it when you're confident the config will load.

```rust
use nurv_config::load_config;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    app: AppConfig,
    database: DatabaseConfig,
}

#[derive(Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    url: String,
}

fn main() {
    // Load config, inferring the type
    let config: Config = load_config!();

    // Or specify the type explicitly
    let config = load_config!(Config);

    // Or use the function
    let config: Config = get_configuration()?;

    println!("App name: {}", config.app.name);
    println!("Port: {}", config.app.port);
    println!("Database URL: {}", config.database.url);
}
```

### Environment-Specific Loading

Set the `APP_ENV` environment variable to load additional config files:

```bash
# Loads base.yaml + local.yaml
APP_ENV=local cargo run

# Loads base.yaml + production.yaml
APP_ENV=production cargo run

# Loads only base.yaml
cargo run
```

## API Reference

### Macros

- `load_config!()` - Load configuration, panicking on error. Type inferred.
- `load_config!(Type)` - Load configuration of specified type, panicking on error.

### Functions

- `get_configuration<T>() -> Result<T, ConfigError>` - Load configuration with error handling.

### Types

- `ConfigError` - Enum representing possible configuration errors (IO or Config parsing).

## Testing

Run the tests with:

```bash
cargo test
```

The tests use the example config files in `config/`.

## License

This project is licensed under MIT OR Apache-2.0.