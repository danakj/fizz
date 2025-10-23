// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{self, DiscordContext, DiscordError};
use crate::model;

/// Tell fizz that you are/aren't a project lead.
///
/// Use `/whoami`` to find out if you are currently set as a lead.
#[poise::command(slash_command, guild_only)]
pub async fn my_role_is_lead(
    ctx: DiscordContext<'_>,
    #[description = "Whether you want to get pinged for leads issues"]
    true_or_false: bool,
) -> Result<(), DiscordError> {
    {
        let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
        let user_id: model::DiscordUserId = ctx.author().into();
        discord::util::update_user_config(ctx, guild_id, user_id, move |c| {
            c.lead = true_or_false;
            Ok(())
        })
        .await?;
    }

    let reply = if true_or_false {
        format!(":white_check_mark: You will be pinged for leads issues")
    } else {
        format!(":white_check_mark: You will not be pinged for leads issues")
    };
    ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
        .await?;
    Ok(())
}
