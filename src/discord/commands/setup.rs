// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use poise::serenity_prelude as serenity;

use crate::discord::{self, DiscordContext, DiscordError};
use crate::model;

/// Asks the bot to join a channel and report on active PRs there.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn setup(
    ctx: DiscordContext<'_>,
    #[description = "The channel to report in"]
    #[channel_types("Text")]
    channel: serenity::Channel,
    #[description = "The GitHub repository owner/organization"] repo_owner: String,
    #[description = "The GitHub repository name"] repo_name: String,
) -> Result<(), DiscordError> {
    let serenity::Channel::Guild(guild_channel) = &channel else {
        return Err("Unexpected channel type".into());
    };

    let guild_id: model::DiscordGuildId = guild_channel.guild_id.into();
    let channel_id: model::DiscordChannelId = guild_channel.into();
    println!(
        "Asked to report PRS in guild {} channel {} ({}) by {}",
        guild_id.0,
        channel_id.1,
        guild_channel.name(),
        ctx.author().name,
    );

    discord::util::update_guild_config(ctx, guild_id, |c| {
        c.repo_owner = repo_owner;
        c.repo_name = repo_name;
        c.report_channel_id = channel_id;
        c.report_channel_name = guild_channel.name().to_owned();
        Ok(())
    })
    .await?;

    let reply = format!(
        ":white_check_mark: Reporting PRs in channel #{}",
        guild_channel.name()
    );
    ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
        .await?;
    Ok(())
}
