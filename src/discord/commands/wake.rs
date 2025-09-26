// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{DiscordContext, DiscordError};

/// Generate a fresh report on active PRs immediately.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn wake(ctx: DiscordContext<'_>) -> Result<(), DiscordError> {
    crate::discord::tasks::watch_github_wake_now(ctx.guild_id().unwrap().into()).await?;

    ctx.send(
        poise::CreateReply::default()
            .content(":yawning_face:")
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
