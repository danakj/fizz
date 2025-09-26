// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord;
use crate::discord::{DiscordContext, DiscordError};
use crate::model;

/// End your time away.
#[poise::command(slash_command, guild_only)]
pub async fn back(ctx: DiscordContext<'_>) -> Result<(), DiscordError> {
    {
        let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
        let user_id: model::DiscordUserId = ctx.author().into();
        discord::util::update_user_config(ctx, guild_id, user_id, |c| {
            c.away_until = None;
            Ok(())
        })
        .await?;
    }

    ctx.send(
        poise::CreateReply::default()
            .content(":white_check_mark: Welcome back!")
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
