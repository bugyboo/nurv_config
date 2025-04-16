use serde::de::DeserializeOwned;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Config(config::ConfigError),
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<config::ConfigError> for ConfigError {
    fn from(err: config::ConfigError) -> Self {
        ConfigError::Config(err)
    }
}

impl StdError for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "IO error: {}", err),
            ConfigError::Config(err) => write!(f, "Config error: {}", err),
        }
    }
}

pub fn get_configuration<T: DeserializeOwned>() -> Result<T, ConfigError> {
    let base_path = std::env::current_dir()?;
    let configuration_directory = base_path.join("config");

    let mut builder = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yaml")));

    let env = std::env::var("APP_ENV").ok();

    if let Some(env) = env {
        builder = builder.add_source(config::File::from(configuration_directory.join(format!("{}.yaml", env))));
    }

    let settings = builder.build()?;

    Ok(settings.try_deserialize::<T>()?)
}

#[macro_export]
macro_rules! load_config {
    () => {
        $crate::get_configuration().expect("Failed to load configuration")
    };
    ($t:ty) => {
        $crate::get_configuration::<$t>().expect("Failed to load configuration")
    };
}