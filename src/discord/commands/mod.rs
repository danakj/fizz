// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

mod away;
mod back;
mod help;
mod my_github_is;
mod my_report_times_are;
mod my_role_is_lead;
mod my_timezone_is;
mod my_workdays_are;
mod ping;
mod remove_me;
mod report_all;
mod setup;
mod wake;
mod whoami;
mod whois_everyone;

use std::sync::Arc;

use crate::discord::{DiscordContext, DiscordData, DiscordError};

#[poise::command(
    slash_command,
    guild_only,
    subcommands(
        "away::away",
        "back::back",
        "help::help",
        "my_report_times_are::my_report_times_are",
        "my_role_is_lead::my_role_is_lead",
        "my_github_is::my_github_is",
        "my_workdays_are::my_workdays_are",
        "my_timezone_is::my_timezone_is",
        "ping::ping",
        "remove_me::remove_me",
        "report_all::report_all",
        "setup::setup",
        "wake::wake",
        "whoami::whoami",
        "whois_everyone::whois_everyone"
    )
)]
pub async fn fizz(_: DiscordContext<'_>) -> Result<(), DiscordError> {
    Ok(())
}

pub fn all() -> Vec<poise::Command<Arc<DiscordData>, DiscordError>> {
    vec![fizz()]
}
