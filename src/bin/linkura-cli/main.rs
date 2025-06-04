use config::init;

use linkura_client::log;

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
            );
        }
        None => {
            command::default::run(&global);
        }
    }

    return;
}
