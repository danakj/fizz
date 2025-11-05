// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use super::{DiscordChannelId, DiscordGuildId, DiscordUserId, GithubUserName};
use crate::error::Error;

const APP_NAME: &str = "fizz";
const VERSION: i32 = 1;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserConfig {
   // Settings provided by the user to control how the bot notifies them.

    /// A list of Github usernames for the user.
    #[serde(default)]
    pub github_names: Vec<GithubUserName>,
    /// Whether the user wants pings for leads issues.
    #[serde(default)]
    pub lead: bool,
    /// The timezone for the user, where they are currently working from. Dates
    /// and times specified by the user are all relative to this.
    #[serde(default)]
    pub timezone: chrono_tz::Tz,
    /// The workdays for the user as a string of numbers. 0 means Sunday, 1 means
    /// Monday, etc.
    #[serde(default = "default_workdays")]
    pub workdays: String,
    /// The times at which Fizz should alert the user about outstanding reviews.
    #[serde(default = "default_report_times")]
    pub report_times: Vec<NaiveTime>,
    /// A date on which the user will be back from being away. For dates up to
    /// and including this date, they are away.
    #[serde(default)]
    pub away_until: Option<NaiveDate>,

    // Internal state.

    /// The discord username when the user is first seen. This may become stale.
    #[serde(default)]
    pub friendly_name: String,

    /// The time when the last weekly report of non-urgent leads issues was
    // made for the user. Used to throttle these reports for each user.
    #[serde(default)]
    pub last_weekly_report: Option<DateTime<Utc>>,
}

fn default_workdays() -> String {
    "12345".to_string()
}

fn default_report_times() -> Vec<NaiveTime> {
    vec![
        NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
    ]
}

impl UserConfig {
    pub fn new(friendly_name: String) -> Self {
        Self {
            friendly_name,
            github_names: Default::default(),
            lead: Default::default(),
            timezone: Default::default(),
            workdays: default_workdays(),
            report_times: default_report_times(),
            away_until: Default::default(),
            last_weekly_report: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct GuildConfig {
    pub repo_owner: String,
    pub repo_name: String,
    /// A stable discord channel identifier.
    pub report_channel_id: DiscordChannelId,
    /// The friendly name of the channel, when it was added. It may not match
    /// reality if the channel is renamed later.
    pub report_channel_name: String,
    // Configuration for users in the Discord guild (aka server).
    #[serde(default)]
    pub users: HashMap<DiscordUserId, UserConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    version: i32,

    pub guilds: HashMap<DiscordGuildId, GuildConfig>,
}

pub fn app_name() -> &'static str {
    return APP_NAME;
}

#[cfg(unix)]
fn config_dir() -> Result<PathBuf, Error> {
    if let Ok(fizz_config_dir) = std::env::var("FIZZ_CONFIG_DIR") {
        return Ok(PathBuf::from(fizz_config_dir));
    }

    let mut path = match std::env::var("XDG_CONFIG_HOME") {
        Ok(s) => PathBuf::from(s),
        Err(_) => {
            let Some(home) = std::env::home_dir() else {
                return Err(Error::UnableToFindHomeDir);
            };
            let mut path = home;
            path.push(".config");
            path
        }
    };
    path.push(app_name());
    Ok(path)
}

fn config_file_path() -> Result<PathBuf, Error> {
    let mut path = config_dir()?;
    path.push(format!("{}.toml", app_name()));
    Ok(path)
}

pub fn new() -> Config {
    Config {
        version: VERSION,
        guilds: HashMap::new(),
    }
}

pub fn load() -> Result<Config, Error> {
    let file_path = config_file_path()?;
    if !file_path.exists() {
        return Err(Error::ConfigFileMissing(file_path));
    }
    let data = match std::fs::read_to_string(&file_path) {
        Ok(data) => data,
        Err(io) => return Err(Error::IoError(Some(file_path), io)),
    };
    let config: Config = match toml::from_str(&data) {
        Ok(config) => config,
        Err(e) => {
            return Err(Error::ConfigParsingError(
                file_path,
                e.message().to_string(),
            ))
        }
    };
    if config.version != VERSION {
        return Err(Error::ConfigParsingError(
            file_path,
            format!("Config file has unknown version {}", config.version),
        ));
    }
    Ok(config)
}

pub fn save(config: &Config) -> Result<(), Error> {
    let dir = config_dir()?;
    match std::fs::create_dir_all(&dir) {
        Ok(_) => {}
        Err(io) => return Err(Error::IoError(Some(dir), io)),
    };
    let file_path = config_file_path()?;
    let data = toml::to_string(config).unwrap();
    match std::fs::write(&file_path, data) {
        Ok(_) => {}
        Err(io) => return Err(Error::IoError(Some(file_path), io)),
    }
    Ok(())
}
