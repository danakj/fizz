// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{DiscordContext, DiscordError};

/// Ping me and I will reply.
#[poise::command(slash_command, guild_only)]
pub async fn ping(ctx: DiscordContext<'_>) -> Result<(), DiscordError> {
    ctx.send(
        poise::CreateReply::default()
            .content("pong")
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
