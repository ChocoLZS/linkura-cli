use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init(log_level: Option<String>) {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(log_level.unwrap_or_else(|| "info".into()))),
        )
        .init();
}
