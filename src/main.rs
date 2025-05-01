use config::init;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod api;
mod config;

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    // do prepare
    let global = init().expect("Failed to initialize config");
    let api_client = &global.api_client;
    let res = api_client.get_with_meets_plan_list().unwrap();
    println!("res: {:?}", res);
    return;
}
