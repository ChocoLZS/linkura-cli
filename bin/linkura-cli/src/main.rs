use config::init;

use linkura_common::log;

rust_i18n::i18n!("locales", fallback = "en");

mod command;
mod config;

use rust_i18n::t;

use crate::config::Commands;
fn main() {
    log::init();

    let global = init().expect(&t!("config.initialize.failed"));
    let args = &global.args;

    match &args.command {
        Some(Commands::MRS { .. }) => {
            todo!("Implement MRS client command handling");
        }
        Some(Commands::ALS {
            addr,
            port,
            room_id,
            token,
            watch,
        }) => {
            let _ = command::als::run(
                &global,
                command::als::AlsConnectionInfo {
                    address: addr.clone(),
                    port: *port,
                    room_id: room_id.clone(),
                    token: token.clone(),
                },
                *watch,
            )
            .map_err(|e| {
                tracing::error!("Error running ALS command: {}", e);
                std::process::exit(1);
            });
        }
        None => {
            command::default::run(&global);
        }
    }

    return;
}
