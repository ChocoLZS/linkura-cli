use crate::{cli::spinner::SpinnerManager, commands::api::ArgsAPI};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use linkura_api::{self, ApiClient, Credential};
use linkura_i18n::t;

/** ARG PARSER **/
#[derive(Parser, Debug)]
#[clap(version)]
#[command(
    name = "linkura-cli",
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

    #[clap(long = "player-id", value_name = "PLAYER_ID")]
    pub player_id: Option<String>,
    #[clap(long = "password", value_name = "PASSWORD")]
    pub password: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get api response data
    API(ArgsAPI),
    /// Get app version & res version
    Version,
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
    pub spinner_manager: SpinnerManager,
}

impl Global {
    pub fn new(args: Args) -> Self {
        let spinner_manager = SpinnerManager::new(args.quiet);
        let mut api_client = linkura_api::ApiClient::new();
        let mut config_manager = ConfigManager::new(args.config_path.clone());

        let config_res = config_manager.load_config();

        let config = if config_res.is_err() {
            tracing::error!("Failed to load config: {:?}", config_res.err());
            Self::initialize_config(&args, &config_manager, &mut api_client, &spinner_manager)
        } else {
            match config_res.unwrap() {
                Some(mut config) => {
                    if !args.skip {
                        let sp =
                            spinner_manager.create_spinner(&t!("linkura.config.checking.version"));
                        // check if latest res_version and client_version
                        let (res_version, client_version) =
                            api_client.high_level().get_app_version().unwrap();
                        if let Some(res_version) = res_version {
                            if res_version != config.credential.res_version {
                                sp.set_message(t!(
                                    "linkura.config.new.res.version",
                                    old = config.credential.res_version.clone(),
                                    new = res_version.clone()
                                ));

                                config.credential.res_version = res_version;
                            }
                        }

                        if let Some(client_version) = client_version {
                            if client_version != config.credential.client_version {
                                sp.set_message(t!(
                                    "linkura.config.new.client.version",
                                    old = config.credential.client_version.clone(),
                                    new = client_version.clone()
                                ));
                                config.credential.client_version = client_version;
                            }
                        }

                        sp.finish_with_message(t!("linkura.config.version.check.complete"));
                    }

                    config
                }
                None => Self::initialize_config(
                    &args,
                    &config_manager,
                    &mut api_client,
                    &spinner_manager,
                ),
            }
        };

        api_client.update_with_credential(&config.credential);
        Self {
            config,
            config_manager,
            api_client,
            args,
            spinner_manager,
        }
    }

    fn initialize_config(
        args: &Args,
        config_manager: &ConfigManager,
        api_client: &mut ApiClient,
        spinner_manager: &SpinnerManager,
    ) -> Config {
        tracing::warn!(
            "No config found, creating a new one to path: {}",
            config_manager.get_config_path().display()
        );
        // first time to init interactive
        let credential = interactive::get_credential_with_simple_prompt(
            api_client,
            spinner_manager,
            args.player_id.clone(),
            args.password.clone(),
        )
        .expect("Failed to get credential");
        Config { credential }
    }
}

/*  CONFIG END **/

pub fn init(args: Args) -> Result<Global> {
    tracing::info!("Initializing config...");
    let mut global = Global::new(args);
    tracing::info!("Config initialized!");

    let sp = global
        .spinner_manager
        .create_spinner_with_color(&t!("linkura.config.logging.in"), "blue");
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
    sp.set_message(t!("linkura.config.testing.login"));
    match global.api_client.high_level().get_plan_list() {
        Ok(_) => {}
        Err(_) => {
            sp.set_message(t!("linkura.config.test.failed.retry"));
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
                    anyhow::anyhow!(t!("linkura.config.login.failed", error = e.to_string()))
                })?;
            global.config.credential.session_token = Some(session_token.clone());
            global.api_client.set_session_token(&session_token);
        }
    }

    global
        .config_manager
        .save_config(&global.config)
        .context("Failed to save config")?;
    sp.finish_with_message(t!(
        "linkura.config.login.success",
        path = global
            .config_manager
            .get_config_path()
            .display()
            .to_string(),
        token = session_token
    ));
    Ok(global)
}

pub mod interactive;
