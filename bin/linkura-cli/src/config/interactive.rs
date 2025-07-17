use anyhow::Result;
use inquire::{Password, Text};

use linkura_api::{self, ApiClient, Credential};
use crate::cli::spinner::SpinnerManager;

pub fn get_credential_with_simple_prompt(client: &mut ApiClient, spinner_manager: &SpinnerManager, player_id: Option<String>, password: Option<String>) -> Result<Credential> {
    let player_id = match player_id {
        Some(id) => id,
        None => Text::new("请输入你的账号id（app登陆界面左上角）").prompt()?,
    };
    let id_token = match password {
        Some(pwd) => pwd,
        None => Password::new("请输入你的账户密码")
            .without_confirmation()
            .prompt()?,
    };
    let sp = spinner_manager.create_spinner("正在获取登录信息...");
    let (res_version, client_version) = client.high_level().get_app_version()?;
    sp.set_message("获取app版本信息成功！");
    let res_version = res_version.unwrap_or(linkura_api::BASE_RES_VERSION.to_string());
    let client_version = client_version.unwrap_or(linkura_api::BASE_CLIENT_VERSION.to_string());
    println!(
        "App version: {}, Client version: {}",
        res_version, client_version
    );
    client.update_version(&res_version, &client_version);
    let device_specific_id = client.high_level().password_login(&player_id, &id_token)?;
    sp.finish_with_message("获取登录信息成功！");
    Ok(Credential {
        res_version,
        client_version,
        device_specific_id,
        player_id,
        session_token: None,
    })
}
