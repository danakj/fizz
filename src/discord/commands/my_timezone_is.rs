// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{self, DiscordContext, DiscordError};
use crate::model;

/// Tell fizz your timezone. See names in
/// https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
///
/// Use `/whoami`` to find what your timezone is set to.
#[poise::command(slash_command, guild_only)]
pub async fn my_timezone_is(
    ctx: DiscordContext<'_>,
    #[description = "Your timezone (e.g. 'PST', 'US/Pacific')"] timezone: String,
) -> Result<(), DiscordError> {
    let Ok(tz) = chrono_tz::Tz::from_str_insensitive(&timezone) else {
        return Err(format!("Unknown timezone '{}'", &timezone).into());
    };

    {
        let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
        let user_id: model::DiscordUserId = ctx.author().into();
        discord::util::update_user_config(ctx, guild_id, user_id, move |c| {
            c.timezone = tz;
            Ok(())
        })
        .await?;
    }

    let reply = format!(":white_check_mark: Your timezone is now '{}'", &timezone);
    ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
        .await?;
    Ok(())
}
