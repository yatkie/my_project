use std::error::Error;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() -> Result<WorkerGuard, Box<dyn Error>> {
    std::fs::create_dir_all("logs")?;

    let file_appender = tracing_appender::rolling::never("logs", "application.log");

    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let console_layer = fmt::layer().with_target(false).with_writer(std::io::stdout);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_target(true)
        .with_writer(file_writer);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .try_init()?;

    tracing::info!("Logging system initialized");

    Ok(guard)
}
