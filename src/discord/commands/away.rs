// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord;
use crate::discord::{DiscordContext, DiscordError};
use crate::model;

/// Tell me how long you are away for, and you won't receive PR review reports
/// during that time.
///
/// Use `0` to say you are away just today.
#[poise::command(slash_command, guild_only)]
pub async fn away(
    ctx: DiscordContext<'_>,
    #[description = "How many days you are away, after today (include weekends in the count)"]
    number_of_days: u32,
) -> Result<(), DiscordError> {
    let user_timezone =
        std::sync::Arc::<std::sync::Mutex<Option<chrono_tz::Tz>>>::new(std::sync::Mutex::new(None));

    {
        let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
        let user_id: model::DiscordUserId = ctx.author().into();
        let user_timezone_clone = user_timezone.clone();
        discord::util::update_user_config(ctx, guild_id, user_id, move |c| {
            let mut tz_guard = user_timezone_clone.lock()?;
            *tz_guard = Some(c.timezone.clone());
            Ok(())
        })
        .await?;
    }
    let user_today = chrono::Local::now()
        .with_timezone(&user_timezone.lock()?.unwrap())
        .date_naive();

    let user_back = user_today
        + match chrono::TimeDelta::try_days(1 + i64::from(number_of_days)) {
            Some(b) => b,
            None => {
                return Err("Away too many days, can't do the math".into());
            }
        };

    {
        let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
        let user_id: model::DiscordUserId = ctx.author().into();
        let user_back_clone = user_back.clone();
        discord::util::update_user_config(ctx, guild_id, user_id, move |c| {
            c.away_until = Some(user_back_clone);
            Ok(())
        })
        .await?;
    }

    ctx.send(
        poise::CreateReply::default()
            .content(&format!(
                ":white_check_mark: You are now away, and will be back on {}",
                user_back
            ))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
