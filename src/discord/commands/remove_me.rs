// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{self, DiscordContext, DiscordError};
use crate::model;

/// Tell fizz to forget everything about you.
#[poise::command(slash_command, guild_only)]
pub async fn remove_me(ctx: DiscordContext<'_>) -> Result<(), DiscordError> {
    let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
    let user_id: model::DiscordUserId = ctx.author().into();
    discord::util::update_guild_config(ctx, guild_id, move |c| {
        c.users.remove(&user_id);
        Ok(())
    })
    .await?;

    let reply = ":white_check_mark: You have been removed.";
    ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
        .await?;
    Ok(())
}
