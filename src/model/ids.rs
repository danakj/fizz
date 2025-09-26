// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use serde::{Deserialize, Serialize};

/// A stable discord guild (server) identifier.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Debug)]
#[repr(transparent)]
pub struct DiscordGuildId(pub String);
impl DiscordGuildId {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// A stable discord user identifier.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Debug)]
pub struct DiscordUserId(pub String);
impl DiscordUserId {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl std::fmt::Display for DiscordUserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<@{}>", self.0)
    }
}

/// A stable discord channel (in a guild/server) identifier.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Debug)]
pub struct DiscordChannelId(pub DiscordGuildId, pub String);
impl DiscordChannelId {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() || self.1.is_empty()
    }
}

/// A github user name.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Debug)]
pub struct GithubUserName(pub String);
impl GithubUserName {
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}
impl std::fmt::Display for GithubUserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
