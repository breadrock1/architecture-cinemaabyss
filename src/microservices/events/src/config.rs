use config::{Config, ConfigError, Environment, File, FileFormat};
use dotenv::dotenv;
use getset::{CopyGetters, Getters};
use serde::Deserialize;

const DEV_FILE_CONFIG_PATH: &str = "./config/development.toml";
const SERVICE_RUN_MODE: &str = "RUN_MODE";
const SERVICE_PREFIX: &str = "";

#[derive(Clone, Deserialize, Getters, CopyGetters)]
#[getset(get = "pub")]
pub struct ServiceConfig {
    logger: String,
    address: String,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    port: u16,
    monolith_url: String,
    movies_service_url: String,
    events_service_url: String,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    gradual_migration: bool,
    #[getset(skip)]
    #[getset(get_copy = "pub")]
    movies_migration_percent: u32,
}

impl ServiceConfig {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let dev_file_config = File::with_name(DEV_FILE_CONFIG_PATH);

        let run_mode = std::env::var(SERVICE_RUN_MODE).unwrap_or("development".into());
        let run_mode_file_path = format!("./config/{}", run_mode);
        let file_config = File::with_name(&run_mode_file_path)
            .format(FileFormat::Toml)
            .required(false);

        let env_config = Environment::with_prefix(SERVICE_PREFIX);

        let settings = Config::builder()
            .add_source(dev_file_config)
            .add_source(file_config)
            .add_source(env_config)
            .build()?;

        settings.try_deserialize()
    }
}
