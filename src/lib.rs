pub mod configuration;

pub use configuration::{get_configuration, ConfigError};

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize, Debug)]
    struct MySettings {
        app: AppConfig,
        database: DatabaseConfig,
    }

    #[derive(Deserialize, Debug)]
    struct AppConfig {
        api_key: String,
        port: u16,
    }

    #[derive(Deserialize, Debug)]
    struct DatabaseConfig {
        pub database_name: String,
        pub username: String,
        pub password: String,
    }        

    #[test]
    fn it_works() {

        let settings: MySettings = get_configuration().expect("Failed to load config");
        assert_eq!(settings.app.api_key, "my_api_key");
        assert_eq!(settings.app.port, 8080);

        assert_eq!(settings.database.database_name, "my_database");
        assert_eq!(settings.database.username, "my_username");
        assert_eq!(settings.database.password, "my_password");
    }
}
