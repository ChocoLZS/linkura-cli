use anyhow::Result;
use inquire::{Password, Text};
use linkura_i18n::t;

use crate::cli::spinner::SpinnerManager;
use linkura_api::{self, ApiClient, Credential};

pub fn get_credential_with_simple_prompt(
    client: &mut ApiClient,
    spinner_manager: &SpinnerManager,
    player_id: Option<String>,
    password: Option<String>,
) -> Result<Credential> {
    let player_id = match player_id {
        Some(id) => id,
        None => Text::new(&t!("linkura.interactive.prompt.account")).prompt()?,
    };
    let id_token = match password {
        Some(pwd) => pwd,
        None => Password::new(&t!("linkura.interactive.prompt.password"))
            .without_confirmation()
            .prompt()?,
    };
    let sp = spinner_manager.create_spinner(&t!("linkura.interactive.fetching.login.info"));
    let (res_version, client_version) = client.high_level().get_app_version()?;
    sp.set_message(t!("linkura.interactive.fetch.app.version.success"));
    let res_version = res_version.unwrap_or(linkura_api::BASE_RES_VERSION.to_string());
    let client_version = client_version.unwrap_or(linkura_api::BASE_CLIENT_VERSION.to_string());
    println!(
        "App version: {}, Client version: {}",
        res_version, client_version
    );
    client.update_version(&res_version, &client_version);
    let device_specific_id = client.high_level().password_login(&player_id, &id_token)?;
    sp.finish_with_message(t!("linkura.interactive.fetch.login.info.success"));
    Ok(Credential {
        res_version,
        client_version,
        device_specific_id,
        player_id,
        session_token: None,
    })
}
