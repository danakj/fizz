// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::str::FromStr;

use chrono::NaiveTime;

use crate::discord;
use crate::discord::{DiscordContext, DiscordError};
use crate::model;

/// Tell fizz when to let you know about outstanding PR reviews coming your way.
/// Default: 9am and 12pm.
///
/// Use `/whoami`` to find your current alert times.
#[poise::command(slash_command, guild_only)]
pub async fn my_report_times_are(
    ctx: DiscordContext<'_>,
    #[description = "Your report times, in 24h time, comma separated (e.g. '9:00,13:00')"]
    report_times: String,
) -> Result<(), DiscordError> {
    let split_report_times: Vec<String> = report_times
        .split(',')
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .collect();

    let times: Vec<NaiveTime> = match split_report_times
        .iter()
        .map(|s| NaiveTime::from_str(s))
        .collect()
    {
        Ok(times) => times,
        Err(e) => {
            return Err(DiscordError::new(
                "Unable to parse report times. Use 24-hour times like 7:00 or 14:00.",
                e,
            ));
        }
    };

    {
        let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
        let user_id: model::DiscordUserId = ctx.author().into();
        let times_clone = times.clone();
        discord::util::update_user_config(ctx, guild_id, user_id, move |c| {
            c.report_times = times_clone;
            Ok(())
        })
        .await?;
    }

    let mut times_str = String::new();
    for t in times {
        if !times_str.is_empty() {
            times_str.push_str(", ");
        }
        times_str.push_str(&t.to_string());
    }
    let reply = format!(
        ":white_check_mark: Your report times are now: {}",
        times_str
    );
    ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
        .await?;
    Ok(())
}
