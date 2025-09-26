// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{DiscordContext, DiscordError};
use crate::model;

pub async fn update_guild_config<F: FnOnce(&mut model::GuildConfig) -> Result<(), DiscordError>>(
    ctx: DiscordContext<'_>,
    guild_id: model::DiscordGuildId,
    f: F,
) -> Result<(), DiscordError> {
    {
        let mut cfg_guard = ctx.data().cfg.lock().await;
        f(cfg_guard.guilds.entry(guild_id).or_default())?;
    }
    save_config(ctx).await?;
    Ok(())
}

pub async fn update_user_config<F: FnOnce(&mut model::UserConfig) -> Result<(), DiscordError>>(
    ctx: DiscordContext<'_>,
    guild_id: model::DiscordGuildId,
    user_id: model::DiscordUserId,
    f: F,
) -> Result<(), DiscordError> {
    {
        let mut cfg_guard = ctx.data().cfg.lock().await;
        let guild_config = cfg_guard.guilds.entry(guild_id).or_default();
        let user_config = guild_config
            .users
            .entry(user_id)
            .or_insert_with(|| ctx.author().into());
        f(user_config)?;
    }
    save_config(ctx).await?;
    Ok(())
}

async fn save_config(ctx: DiscordContext<'_>) -> Result<(), DiscordError> {
    let cfg_guard = ctx.data().cfg.lock().await;
    match model::save(&cfg_guard) {
        Ok(()) => Ok(()),
        Err(e) => Err(DiscordError::new("Failed to save config!", e)),
    }
}
