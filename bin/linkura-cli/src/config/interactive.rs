use anyhow::Result;
use inquire::{Password, Text};
use spinoff::spinners;

use linkura_api::{self, ApiClient, Credential};

pub fn get_credential_with_simple_prompt(client: &mut ApiClient) -> Result<Credential> {
    let player_id = Text::new("请输入你的账号id（app登陆界面左上角）").prompt()?;
    let id_token = Password::new("请输入你的账户密码")
        .without_confirmation()
        .prompt()?;
    let mut sp =
        spinoff::Spinner::new(spinners::Dots, "正在获取登录信息...", spinoff::Color::Green);
    let (res_version, client_version) = client.get_app_version()?;
    sp.update_text("获取app版本信息成功！");
    let res_version = res_version.unwrap_or(linkura_api::BASE_CLIENT_VERSION.to_string());
    let client_version = client_version.unwrap_or(linkura_api::BASE_CLIENT_VERSION.to_string());
    client.update_version(&res_version, &client_version);
    let device_specific_id = client.password_login(&player_id, &id_token)?;
    sp.success("获取登录信息成功！");
    Ok(Credential {
        res_version,
        client_version,
        device_specific_id,
        player_id,
        session_token: None,
    })
}
