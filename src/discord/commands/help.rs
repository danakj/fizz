// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::discord::{DiscordContext, DiscordError};

/// Receive an introduction to fizz.
#[poise::command(slash_command, guild_only)]
pub async fn help(ctx: DiscordContext<'_>) -> Result<(), DiscordError> {
    let msg = "Hello, if you'd like, I can alert you to pending Github PRs. \n\
* To get alerts, tell me your Github username with `/fizz my_github_is <username>`. \n\
* To get an alert fresh in the morning, tell me your timezone with `/fizz my_timezone_is <timezone>`. \n\
* If you have different workdays than Monday to Friday, you can tell me with `/fizz my_workdays_are <days>`. \n\
* To adjust at what times you will receive PR review report, you can use `/fizz my_report_times_are <times>`. \n\
* If you are a project lead and want to get pings for open leads issues, you can use `/fizz my_role_is_lead True`. \n\
* If you will be away and want to pause notifications, you can tell me with `/fizz away <number of days>`. \n\
* If you come back early from `/fizz away` and want to resume notifications, you can tell me with `/fizz back`. \n\
* If you ever want to see what your current settings are, use `/fizz whoami`. \n\
* And you can make me forget everything about you with `/fizz remove_me` \n\
\n\
I can't start alerting about GitHub PRs until I know which repository to watch, and where to send \
messages. \n\
* An administrator can set this up with `/fizz setup`\n \
";

    ctx.send(poise::CreateReply::default().content(msg).ephemeral(true))
        .await?;
    Ok(())
}
