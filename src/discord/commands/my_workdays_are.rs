// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{self, DiscordContext, DiscordError};
use crate::model;

/// Tell fizz your workdays, separated by commas.
///
/// Use `/whoami`` to find your current workdays.
#[poise::command(slash_command, guild_only)]
pub async fn my_workdays_are(
    ctx: DiscordContext<'_>,
    #[description = "Your workdays, separated by commas (e.g. 'monday,tuesday')"] day_names: String,
) -> Result<(), DiscordError> {
    let split_day_names: Vec<String> = day_names
        .split(',')
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .collect();

    let mut workdays = String::new();
    let mut push_day_num = |name: &str, num: char| {
        if split_day_names.iter().any(|s| s == name) {
            workdays.push(num);
        }
    };
    push_day_num("sunday", '0');
    push_day_num("monday", '1');
    push_day_num("tuesday", '2');
    push_day_num("wednesday", '3');
    push_day_num("thursday", '4');
    push_day_num("friday", '5');
    push_day_num("saturday", '6');

    {
        let guild_id: model::DiscordGuildId = ctx.guild_id().unwrap().into();
        let user_id: model::DiscordUserId = ctx.author().into();
        let workdays_copy = workdays.clone();
        discord::util::update_user_config(ctx, guild_id, user_id, move |c| {
            c.workdays = workdays_copy;
            Ok(())
        })
        .await?;
    }

    let mut workday_names = String::new();
    let mut push_day_name = |name: &str, num: char| {
        if workdays.contains(num) {
            if !workday_names.is_empty() {
                workday_names.push_str(", ");
            }
            workday_names.push_str(name);
        }
    };
    push_day_name("Sunday", '0');
    push_day_name("Monday", '1');
    push_day_name("Tuesday", '2');
    push_day_name("Wednesday", '3');
    push_day_name("Thursday", '4');
    push_day_name("Friday", '5');
    push_day_name("Saturday", '6');

    let reply = format!(
        ":white_check_mark: Your workdays are now: {}",
        workday_names
    );
    ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
        .await?;
    Ok(())
}
