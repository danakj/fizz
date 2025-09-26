// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use poise::serenity_prelude as serenity;

use crate::model;

// Github crate to our model.
// --------------------------

impl From<&octocrab::models::Author> for model::GithubUserName {
    fn from(name: &octocrab::models::Author) -> Self {
        Self(name.login.to_string())
    }
}

// Discord crate to our model.
// ---------------------------

impl From<&serenity::User> for model::DiscordUserId {
    fn from(user: &serenity::User) -> Self {
        Self(format!("{}", user.id))
    }
}

impl From<serenity::GuildId> for model::DiscordGuildId {
    fn from(id: serenity::GuildId) -> Self {
        Self(format!("{}", id))
    }
}

impl From<&serenity::GuildId> for model::DiscordGuildId {
    fn from(id: &serenity::GuildId) -> Self {
        Self(format!("{}", id))
    }
}

impl From<&serenity::GuildChannel> for model::DiscordChannelId {
    fn from(channel: &serenity::GuildChannel) -> Self {
        Self(channel.guild_id.into(), format!("{}", channel.id))
    }
}

impl From<serenity::GuildChannel> for model::DiscordChannelId {
    fn from(channel: serenity::GuildChannel) -> Self {
        Self(channel.guild_id.into(), format!("{}", channel.id))
    }
}

impl From<serenity::User> for model::UserConfig {
    fn from(value: serenity::User) -> Self {
        model::UserConfig::new(value.name.clone())
    }
}

impl From<&serenity::User> for model::UserConfig {
    fn from(value: &serenity::User) -> Self {
        model::UserConfig::new(value.name.clone())
    }
}

// Our model to Discord crate.
// ---------------------------

impl From<model::DiscordChannelId> for serenity::ChannelId {
    fn from(value: model::DiscordChannelId) -> Self {
        use std::str::FromStr;
        serenity::ChannelId::from_str(&value.1).expect("failed to make ChannelId from model value")
    }
}

impl From<&model::DiscordChannelId> for serenity::ChannelId {
    fn from(value: &model::DiscordChannelId) -> Self {
        use std::str::FromStr;
        serenity::ChannelId::from_str(&value.1).expect("failed to make ChannelId from model value")
    }
}

impl From<model::DiscordUserId> for serenity::UserId {
    fn from(value: model::DiscordUserId) -> Self {
        use std::str::FromStr;
        serenity::UserId::from_str(&value.0).expect("failed to make UserId from model value")
    }
}

impl From<&model::DiscordUserId> for serenity::UserId {
    fn from(value: &model::DiscordUserId) -> Self {
        use std::str::FromStr;
        serenity::UserId::from_str(&value.0).expect("failed to make UserId from model value")
    }
}
