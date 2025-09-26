// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{self, DiscordContext, DiscordError};
use crate::model;

/// Tell fizz your github username(s). It replaces any previous names.
///
/// Use `/whoami`` to find all the github names you are assigned.
#[poise::command(slash_command, guild_only)]
pub async fn my_github_is(
    ctx: DiscordContext<'_>,
    #[description = "Your username(s) on github, separated by commas (without any @ prefix)"]
    github_names: String,
) -> Result<(), DiscordError> {
    let split_github_names: Vec<&str> = github_names.split(',').map(str::trim).collect();

    {
        let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
        let user_id: model::DiscordUserId = ctx.author().into();
        let model_github_names: Vec<model::GithubUserName> = split_github_names
            .iter()
            .map(|&s| model::GithubUserName::from_str(s))
            .collect();
        discord::util::update_user_config(ctx, guild_id, user_id, move |c| {
            c.github_names = model_github_names;
            Ok(())
        })
        .await?;
    }

    for n in split_github_names {
        let reply = format!(":white_check_mark: You are now known as '{}' on Github", n);
        ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
            .await?;
    }
    Ok(())
}
