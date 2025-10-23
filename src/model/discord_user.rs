// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};

use crate::model;

pub fn discord_user_report_times(
    guild_config: &model::GuildConfig,
    discord_user_id: &model::DiscordUserId,
) -> Vec<DateTime<Utc>> {
    let Some(user_config) = guild_config.users.get(discord_user_id) else {
        return vec![];
    };

    let user_timezone = &user_config.timezone;
    let user_workdays = &user_config.workdays;
    let user_away_until = &user_config.away_until;

    let mut out = Vec::new();

    // Today's date for the user.
    let user_today = chrono::Local::now()
        .with_timezone(user_timezone)
        .date_naive();
    let user_day_number = (user_today.weekday().number_from_sunday() - 1).to_string();
    if !user_workdays.contains(&user_day_number) {
        return vec![];
    }

    if user_today > user_away_until.unwrap_or(NaiveDate::MIN) {
        let utc_datetime = |date, time| {
            // Get the `time` during `date` for the user. Then convert them to UTC
            // time.
            NaiveDateTime::new(date, time)
                .and_local_timezone(user_timezone.clone())
                .map(|dt| dt.with_timezone(&Utc {}))
                .single()
        };

        for user_time in &user_config.report_times {
            if let Some(time) = utc_datetime(user_today, user_time.clone()) {
                out.push(time);
            }
        }
    }

    out
}

pub fn discord_user_weekly_report_needed(
    guild_config: &model::GuildConfig,
    discord_user_id: &model::DiscordUserId,
) -> bool {
    let Some(user_config) = guild_config.users.get(discord_user_id) else {
        return false;
    };

    let user_timezone = &user_config.timezone;

    // Today's date for the user.
    let user_today = chrono::Local::now()
        .with_timezone(user_timezone)
        .date_naive();

    let Some(last_report) = user_config.last_weekly_report else {
        return true;
    };

    let user_last_report = last_report.with_timezone(user_timezone).date_naive();
    let days_since = (user_last_report - user_today).num_days();
    days_since >= 7
}
