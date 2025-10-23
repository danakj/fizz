// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use chrono::{NaiveDate, NaiveTime};

use crate::discord::{DiscordContext, DiscordError};
use crate::model;

/// Tells you your current settings.
#[poise::command(slash_command, guild_only)]
pub async fn whoami(ctx: DiscordContext<'_>) -> Result<(), DiscordError> {
    let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
    let user_id: model::DiscordUserId = ctx.author().into();

    let mut my_github_names: Vec<model::GithubUserName> = Vec::new();
    let mut my_timezone = chrono_tz::Tz::default();
    let mut my_workdays = String::new();
    let mut my_report_times: Vec<NaiveTime> = Vec::new();
    let mut my_away_until: Option<NaiveDate> = None;
    let mut my_lead: bool = false;

    {
        let cfg_guard = ctx.data().cfg.lock().await;
        if let Some(guild_config) = cfg_guard.guilds.get(&guild_id) {
            if let Some(user_config) = guild_config.users.get(&user_id) {
                my_github_names = user_config.github_names.iter().cloned().collect();
                my_timezone = user_config.timezone.clone();
                my_workdays = user_config.workdays.clone();
                my_report_times = user_config.report_times.clone();
                my_away_until = user_config.away_until.clone();
                my_lead = user_config.lead;
            }
        }
    }

    let mut reply = format!(":wave: Hello, {}, here's what I know about you:\n", user_id);

    if my_github_names.is_empty() {
        reply.push_str("* You do not have any Github usernames registered yet\n");
    } else {
        for n in my_github_names {
            reply.push_str(&format!("* You are known as '{}' on Github\n", n));
        }
    }

    if my_lead {
        reply.push_str("* You are a lead and will get pinged for leads issues\n");
    } else {
        reply.push_str("* You are not a lead and won't get pinged for leads issues\n");
    }

    reply.push_str(&format!("* Your timezone is {}\n", my_timezone));
    for (num, name) in [
        ("0", "Sunday"),
        ("1", "Monday"),
        ("2", "Tuesday"),
        ("3", "Wednesday"),
        ("4", "Thursday"),
        ("5", "Friday"),
        ("6", "Saturday"),
    ] {
        if my_workdays.contains(num) {
            reply.push_str(&format!("* Your workdays include {}\n", name));
        }
    }

    for time in my_report_times {
        reply.push_str(&format!(
            "* Your requested times for PR review reports include {}\n",
            time
        ));
    }

    if let Some(away) = my_away_until {
        let user_today = chrono::Local::now()
            .with_timezone(&my_timezone)
            .date_naive();
        if away > user_today {
            reply.push_str(&format!("* You are away, and back on {}", away));
        }
    }

    ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
        .await?;
    Ok(())
}
