use anyhow::{Context, Result};
use clap::{Args as ClapArgs, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use spinoff::{Color, Spinner, spinners};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use linkura_api::{self, ApiClient, Credential};
use rust_i18n::t;

/** ARG PARSER **/
#[derive(Parser, Debug)]
#[command(
    name = "linkura-cli",
    version = "0.1.0",
    author = "ChocoLZS, chocoielzs@gmail.com",
    about = t!("linkura.cli.about").to_string(),
    long_about = None,
    bin_name = "linkura-cli",
)]
pub struct Args {
    #[clap(short('k'), long)]
    pub skip: bool,
    #[clap(short('i'), long = "id", value_name = "ID")]
    pub id: Option<String>,
    #[clap(short('c'), long = "config", value_name = "CONFIG_PATH")]
    pub config_path: Option<String>,
    #[clap(short('Q'), long = "quiet", action = clap::ArgAction::SetTrue)]
    pub quiet: bool,
    #[clap(short('l'), long = "loglevel", value_name = "LOG_LEVEL")]
    /// Sets the log level for the application.
    /// 
    /// Valid values are, in order of verbosity:
    /// 
    /// `off`, `error`, `warn`, `info`, `debug`, `trace`
    ///
    /// Default is "info".
    pub log_level: Option<String>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsMRS {
    #[clap(short('a'), long = "address", value_name = "ADDRESS")]
    pub addr: String,
    #[clap(
        short('p'),
        long = "port",
        value_name = "PORT",
        default_value_t = 21011
    )]
    pub port: u16,
    #[clap(short('r'), long = "room-id", value_name = "ROOM_ID")]
    pub room_id: u32,
    #[clap(short('i'), long = "player-id", value_name = "PLAYER_ID")]
    pub player_id: u16,
    #[clap(short('w'), long = "watch", value_name = "WATCH")]
    pub watch: bool,
}

#[derive(Debug, ClapArgs)]
pub struct ArgsALS {
    #[clap(short('a'), long = "address", value_name = "ADDRESS")]
    pub addr: Option<String>,
    #[clap(short('p'), long = "port", value_name = "PORT")]
    pub port: Option<u16>,
    #[clap(short('l'), long = "room-id", value_name = "ROOM_ID")]
    pub room_id: Option<String>,
    #[clap(short('t'), long = "token", value_name = "TOKEN")]
    pub token: Option<String>,
    #[clap(
        short('w'),
        long = "watch",
        value_name = "WATCH_MODE",
        default_value_t = false
    )]
    pub watch: bool,
}
#[derive(Debug, ClapArgs)]
pub struct ArgsArchive {
    #[clap(short('s'), long = "save-json", value_name = "SAVE_JSON")]
    /// if provided, will save the archive to the file
    /// with the given name, otherwise will just print the archive info
    /// to the console.
    pub save_json: Option<String>,
    #[clap(short('l'), long = "limit", value_name = "LIMIT")]
    /// limit the number of archives to fetch, default is 4
    pub limit: Option<u32>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get mrs data
    MRS(ArgsMRS),
    /// Get als data
    ALS(ArgsALS),
    Archive(ArgsArchive),
}

/** ARG PARSER END**/

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub credential: Credential,
}

#[derive(Debug)]
pub struct ConfigManager {
    args_config_path: Option<PathBuf>,
    current_dir_config_path: PathBuf,
    home_dir_config_path: PathBuf,

    runtime_config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(args_path: Option<String>) -> Self {
        let args_config_path = args_path.map(PathBuf::from);
        // 获取当前目录下的配置文件路径
        let current_dir_config_path = PathBuf::from("linkura-cli_config.json");

        #[cfg(unix)]
        let home = std::env::var("HOME").ok().map(PathBuf::from);
        #[cfg(windows)]
        let home = std::env::var("USERPROFILE").ok().map(PathBuf::from);

        // 获取home目录下的配置文件路径
        let mut home_dir_config_path = home.clone().unwrap();
        home_dir_config_path.push(".config");
        home_dir_config_path.push("linkura-cli");
        home_dir_config_path.push("config.json");
        let runtime_config_path = home_dir_config_path.clone();

        Self {
            args_config_path,
            current_dir_config_path,
            home_dir_config_path,
            runtime_config_path,
        }
    }

    pub fn load_config(&mut self) -> Result<Option<Config>> {
        // 1. 首先检查用户提供的args配置
        if let Some(config) = &self.args_config_path {
            if config.exists() {
                self.runtime_config_path = config.clone();
                return Ok(Some(self.read_config(config)?));
            }
        }

        // 2. 检查当前目录下的配置文件
        if self.current_dir_config_path.exists() {
            self.runtime_config_path = self.current_dir_config_path.clone();
            return Ok(Some(self.read_config(&self.current_dir_config_path)?));
        }

        // 3. 检查home目录下的配置文件
        if self.home_dir_config_path.exists() {
            self.runtime_config_path = self.home_dir_config_path.clone();
            return Ok(Some(self.read_config(&self.home_dir_config_path)?));
        }

        // 如果都没有，则创建home目录下的配置文件
        // mkdir -p
        if let Some(parent) = self.home_dir_config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(None)
    }

    pub fn get_config_path(&self) -> &PathBuf {
        &self.runtime_config_path
    }

    fn read_config(&self, path: &Path) -> Result<Config> {
        let content = fs::read_to_string(path).context(format!(
            "Failed to read config file at {:?}",
            path.display()
        ))?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save_config(&self, config: &Config) -> Result<()> {
        let path = self.get_config_path();
        tracing::debug!("Trying to save config to {:?}", path);
        // 确保目录存在
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create config directory at {}", parent.display())
                })?;
            }
        }

        let content = serde_json::to_string_pretty(config).context("Failed to serialize config")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file at {}", path.display()))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Global {
    pub config: Config,
    pub config_manager: ConfigManager,
    pub api_client: linkura_api::ApiClient,
    pub args: Args,
}

impl Global {
    pub fn new(args: Args) -> Self {
        let args = args;
        let mut api_client = linkura_api::ApiClient::new();
        let mut config_manager = ConfigManager::new(args.config_path.clone());

        let config_res = config_manager.load_config();

        let config = if config_res.is_err() {
            tracing::error!("Failed to load config: {:?}", config_res.err());
            Self::initialize_config(&config_manager, &mut api_client)
        } else {
            match config_res.unwrap() {
                Some(mut config) => {
                    if !args.skip {
                        let mut sp = Spinner::new(
                            spinners::Dots,
                            "Checking for linkura version...",
                            Color::Green,
                        );
                        // check if latest res_version and client_version
                        let (res_version, client_version) =
                            api_client.high_level().get_app_version().unwrap();
                        if let Some(res_version) = res_version {
                            if res_version != config.credential.res_version {
                                sp.update_text(format!(
                                    "New res version found, update from {} to {}",
                                    config.credential.res_version, res_version
                                ));

                                config.credential.res_version = res_version;
                            }
                        }

                        if let Some(client_version) = client_version {
                            if client_version != config.credential.client_version {
                                sp.update_text(format!(
                                    "New client version found, update from {} to {}",
                                    config.credential.client_version, client_version
                                ));
                                config.credential.client_version = client_version;
                            }
                        }

                        sp.success("Version check complete!");
                    }

                    config
                }
                None => Self::initialize_config(&config_manager, &mut api_client),
            }
        };

        api_client.update_with_credential(&config.credential);
        Self {
            config,
            config_manager,
            api_client,
            args,
        }
    }

    fn initialize_config(config_manager: &ConfigManager, api_client: &mut ApiClient) -> Config {
        tracing::warn!(
            "No config found, creating a new one to path: {}",
            config_manager.get_config_path().display()
        );
        // first time to init interactive
        let credential = interactive::get_credential_with_simple_prompt(api_client)
            .expect("Failed to get credential");
        Config { credential }
    }
}

/*  CONFIG END **/

pub fn init(args: Args) -> Result<Global> {
    tracing::info!("Initializing config...");
    let mut global = Global::new(args);
    tracing::info!("Config initialized!");

    let mut sp = Spinner::new(spinners::Dots, "登陆中...", Color::Blue);
    let session_token = if global.config.credential.session_token.is_none() {
        let session_token = global.api_client.high_level().device_id_login(
            &global.config.credential.player_id,
            &global.config.credential.device_specific_id,
        )?;
        global.config.credential.session_token = Some(session_token.clone());
        session_token
    } else {
        global.config.credential.session_token.clone().unwrap()
    };
    global.api_client.set_session_token(&session_token);
    // 测试登录态
    sp.update_text("测试是否登录成功...");
    match global.api_client.high_level().get_plan_list() {
        Ok(_) => {}
        Err(_) => {
            sp.update_text("测试获取信息失败，尝试重新登录");
            global.api_client.del_session_token();
            // delete session token
            let session_token = global
                .api_client
                .high_level()
                .device_id_login(
                    &global.config.credential.player_id,
                    &global.config.credential.device_specific_id,
                )
                .map_err(|e| {
                    anyhow::anyhow!(
                        "初始化登录失败: {:?}，请尝试删除配置文件重新配置，或者使用命令行参数...",
                        e
                    )
                })?;
            global.config.credential.session_token = Some(session_token.clone());
            global.api_client.set_session_token(&session_token);
        }
    }

    global
        .config_manager
        .save_config(&global.config)
        .context("Failed to save config")?;
    sp.success(&format!(
        "登陆成功！信息已保存至{}，session token: {}",
        global.config_manager.get_config_path().display(),
        session_token
    ));
    Ok(global)
}

pub mod interactive;
