// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use poise::serenity_prelude as serenity;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    #[allow(dead_code)]
    Silent,
    UnableToFindHomeDir,
    IoError(Option<PathBuf>, std::io::Error),
    ConfigFileMissing(PathBuf),
    ConfigParsingError(PathBuf, String),
    FailedToGetIssues(octocrab::Error),
    FailedToGetPRs(octocrab::Error),
    DiscordTokenMissing(String),
    DiscordConnectFailed(serenity::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            Silent => Ok(()),
            UnableToFindHomeDir => write!(f, "unable to find home dir"),
            IoError(Some(path), io) => write!(f, "{}: {}", io, path.display()),
            IoError(None, io) => write!(f, "{}", io),
            ConfigFileMissing(path) => write!(f, "no config found at {}", path.display()),
            ConfigParsingError(path, msg) => write!(
                f,
                "error parsing config file: {} (path: {})",
                msg,
                path.display()
            ),
            FailedToGetIssues(e) => write!(f, "unable to get GitHub Issues: {}", e),
            FailedToGetPRs(e) => write!(f, "unable to get GitHub PRs: {}", e),
            DiscordTokenMissing(var) => {
                write!(f, "missing discord token in {} environment variable", var)
            }
            DiscordConnectFailed(e) => write!(f, "unable to connect to discord: {}", e),
        }
    }
}
