use tracing_subscriber::filter::FromEnvError;

use crate::config::ServiceConfig;

pub fn init_logger(config: &ServiceConfig) -> Result<(), FromEnvError> {
    init_rust_log_env(config);

    let env_filter = tracing_subscriber::EnvFilter::builder().from_env()?;
    tracing_subscriber::FmtSubscriber::builder()
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(env_filter)
        .init();

    Ok(())
}

fn init_rust_log_env(config: &ServiceConfig) {
    let level = config.logger();
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", level);
        }
    }
}
