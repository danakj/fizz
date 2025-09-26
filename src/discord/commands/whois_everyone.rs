// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{DiscordContext, DiscordError};
use crate::model;

/// Reports all Discord to Github user mappings.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn whois_everyone(ctx: DiscordContext<'_>) -> Result<(), DiscordError> {
    let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();

    let mut any = false;
    let mut reply = format!(":wave: Hello, here's all the registered Github users:\n");

    {
        let cfg_guard = ctx.data().cfg.lock().await;
        if let Some(guild_config) = cfg_guard.guilds.get(&guild_id) {
            for (discord_user, user_config) in &guild_config.users {
                for n in &user_config.github_names {
                    reply.push_str(&format!(
                        "- {} is known as '{}' on Github\n",
                        discord_user, n
                    ));
                    any = true;
                }
            }
        }
    }

    if !any {
        reply.push_str("There are no Github usernames registered yet\n");
    }

    ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
        .await?;
    Ok(())
}
