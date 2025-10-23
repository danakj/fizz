// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::sync::Arc;

use chrono::{DateTime, Utc};
use poise::serenity_prelude as serenity;
use tokio::sync::Mutex;

use crate::discord;
use crate::discord::{DiscordData, DiscordError};
use crate::github;
use crate::model;

const WAKE_UP_FREQ_SECONDS: u64 = 60 * 5;

static CANCEL_SLEEP: Mutex<Option<tokio::sync::mpsc::Sender<model::DiscordGuildId>>> =
    Mutex::const_new(None);

pub async fn watch_github(http: Arc<serenity::Http>, data: Arc<DiscordData>) {
    let mut interval: tokio::time::Interval =
        tokio::time::interval(std::time::Duration::from_secs(WAKE_UP_FREQ_SECONDS));

    // A channel for `report()` to wake this task up before `interval`, asking
    // for an immediate report in `filter_guild_id`.
    let (send_cancel_sleep, mut recv_cancel_sleep) = tokio::sync::mpsc::channel(100);
    CANCEL_SLEEP.lock().await.replace(send_cancel_sleep);

    let mut last_report_timestamp = Utc::now();

    loop {
        // Wait for the next update period.
        let mut filter_guild_id = None;
        tokio::select! {
            guild_id = recv_cancel_sleep.recv() => {
                filter_guild_id = Some(guild_id.expect("CANCEL_SLEEP was closed"))
            }
            _ = interval.tick() => {
            }
        }

        let now = Utc::now();

        let run_result = report_alerts(
            http.clone(),
            data.clone(),
            &last_report_timestamp,
            &now,
            std::mem::replace(&mut filter_guild_id, None),
        )
        .await;
        match run_result {
            Ok(()) => {
                last_report_timestamp = now;
            }
            Err(e) => {
                eprintln!("ERROR: watching github {}", e);
                // Wait a bit and try again.
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                continue;
            }
        }
    }
}

async fn report_alerts(
    http: Arc<serenity::Http>,
    data: Arc<DiscordData>,
    last_report_timestamp: &DateTime<Utc>,
    now: &DateTime<Utc>,
    ignore_time_for_guild_id: Option<model::DiscordGuildId>,
) -> Result<(), DiscordError> {
    struct GuildAlerts {
        discord_channel_id: model::DiscordChannelId,
        discord_user_ids: Vec<model::DiscordUserId>,
        prs: Arc<Vec<github::Pr>>,
        issues: Arc<Vec<github::LeadsIssue>>,
    }
    let mut alerts = Vec::new();

    struct GuildWeeklyAlerts {
        discord_guild_id: model::DiscordGuildId,
        discord_channel_id: model::DiscordChannelId,
        discord_user_ids: Vec<model::DiscordUserId>,
        issues: Arc<Vec<github::LeadsIssue>>,
    }
    let mut weekly_alerts = Vec::new();

    let should_report = |guild_id: &model::DiscordGuildId, time: &DateTime<Utc>| {
        let ignore_time = match &ignore_time_for_guild_id {
            Some(ignored_guild_id) => ignored_guild_id == guild_id,
            None => false,
        };
        ignore_time || (last_report_timestamp < time && now >= time)
    };

    {
        let cfg_guard = data.cfg.lock().await;
        for (guild_id, guild_config) in &cfg_guard.guilds {
            if guild_config.report_channel_id.is_empty() {
                continue;
            }

            // This is an `await` while we have a mutex guard on `data.cfg`, but
            // its okay because we don't give access to `data` here. The future
            // waits on Github access only.
            let prs_state =
                github::get_prs(&guild_config.repo_owner, &guild_config.repo_name).await?;
            let issues_state =
                github::get_issues(&guild_config.repo_owner, &guild_config.repo_name).await?;

            let mut discord_user_ids_to_alert: Vec<model::DiscordUserId> = Vec::new();
            let mut discord_user_ids_to_weekly_alert: Vec<model::DiscordUserId> = Vec::new();
            for (discord_user_id, _) in &guild_config.users {
                let user_alerts = model::discord_user_report_times(guild_config, discord_user_id);

                // Look for any alert times that we have passed since the last report attempt.
                if user_alerts.iter().any(|r| should_report(guild_id, r)) {
                    discord_user_ids_to_alert.push(discord_user_id.clone());

                    if model::discord_user_weekly_report_needed(guild_config, discord_user_id) {
                        discord_user_ids_to_weekly_alert.push(discord_user_id.clone());
                    }
                }
            }

            let prs: Arc<Vec<_>> =
                Arc::new(github::filter_prs_for_guild(prs_state, guild_config).collect());
            let issues: Arc<Vec<_>> = Arc::new(
                github::filter_leads_issues_for_guild(issues_state, guild_config).collect(),
            );

            alerts.push(GuildAlerts {
                discord_channel_id: guild_config.report_channel_id.clone(),
                discord_user_ids: discord_user_ids_to_alert,
                prs,
                issues: issues.clone(),
            });
            weekly_alerts.push(GuildWeeklyAlerts {
                discord_guild_id: guild_id.clone(),
                discord_channel_id: guild_config.report_channel_id.clone(),
                discord_user_ids: discord_user_ids_to_weekly_alert,
                issues,
            });
        }
    }
    // Drop the mutex guard before doing any `await` to avoid blocking other tasks.

    for alert in weekly_alerts {
        for discord_user_id in &alert.discord_user_ids {
            report_weekly_alerts_for_user(
                http.clone(),
                alert.issues.clone(),
                alert.discord_channel_id.clone(),
                discord_user_id.clone(),
            )
            .await?;

            {
                let mut cfg_guard = data.cfg.lock().await;
                if let Some(guild_config) = cfg_guard.guilds.get_mut(&alert.discord_guild_id) {
                    for discord_user_id in &alert.discord_user_ids {
                        if let Some(user_config) = guild_config.users.get_mut(discord_user_id) {
                            user_config.last_weekly_report = Some(*now);
                        }
                    }
                }
            }
            discord::util::save_config(&data).await?;
        }
    }

    for alert in alerts {
        for discord_user_id in alert.discord_user_ids {
            report_alerts_for_user(
                http.clone(),
                alert.prs.clone(),
                alert.issues.clone(),
                alert.discord_channel_id.clone(),
                discord_user_id,
            )
            .await?;
        }
    }

    Ok(())
}

async fn delete_messages_with_prefix(
    http: Arc<serenity::Http>,
    discord_channel_id: model::DiscordChannelId,
    prefix: String,
) -> Result<(), DiscordError> {
    let user = http.get_current_user().await?;
    discord::util::delete_messages(http, discord_channel_id, |m| {
        if let Some(flags) = m.flags {
            if flags.contains(serenity::MessageFlags::EPHEMERAL) {
                return false;
            }
        }
        if m.author.id != user.id {
            return false;
        }

        m.content.starts_with(&prefix)
    })
    .await
}

fn format_pr(pr: &github::Pr) -> String {
    let mut msg: String = String::new();
    msg.push_str(&format!("[PR #{}](<{}>)", pr.github_pr.number, pr.url));
    if let Some(user) = &pr.github_pr.user {
        msg.push_str(&format!(" **{}**", user.login));
    }
    if let Some(title) = &pr.github_pr.title {
        msg.push_str(&format!("\n    {}", title));
        // Close unbalanced formatting characters.
        if title.chars().filter(|c| *c == '`').count() % 2 == 1 {
            msg.push('`');
        }
    }
    msg
}

fn format_issue(issue: &github::LeadsIssue) -> String {
    let mut msg = String::new();
    msg.push_str(&format!(
        "[Issue #{}](<{}>)",
        issue.github_issue.number, issue.url
    ));
    let user = &issue.github_issue.user;
    msg.push_str(&format!(" **{}**", user.login));

    let title = &issue.github_issue.title;
    msg.push_str(&format!("\n    {}", title));
    // Close unbalanced formatting characters.
    if title.chars().filter(|c| *c == '`').count() % 2 == 1 {
        msg.push('`');
    }
    msg
}

async fn report_alerts_for_user(
    http: Arc<serenity::Http>,
    prs: Arc<Vec<github::Pr>>,
    issues: Arc<Vec<github::LeadsIssue>>,
    discord_channel_id: model::DiscordChannelId,
    discord_user_id: model::DiscordUserId,
) -> Result<(), DiscordError> {
    const PR_HEADER: &str = ":notepad_spiral: PRs for review ";
    const BLOCKING_ISSUES_HEADER: &str = ":fire_engine: Open leads issues (blocking) ";

    delete_messages_with_prefix(
        http.clone(),
        discord_channel_id.clone(),
        format!("{}{}", PR_HEADER, discord_user_id),
    )
    .await?;
    delete_messages_with_prefix(
        http.clone(),
        discord_channel_id.clone(),
        format!("{}{}", BLOCKING_ISSUES_HEADER, discord_user_id),
    )
    .await?;

    let mut msg = String::new();

    let user_prs = prs.iter().filter(|pr| {
        pr.reviewers
            .iter()
            .any(|r| r.discord_users.contains(&discord_user_id))
    });
    let mut any_prs = false;
    for pr in user_prs {
        if !any_prs {
            msg.push_str(&format!("{}{}", PR_HEADER, discord_user_id));
        }
        msg.push_str(&format!("\n* {}", format_pr(&pr)));
        any_prs = true;
    }

    let mut any_issues = false;
    let user_issues = issues.iter().filter(|issue: &_| {
        issue.urgency == github::Urgency::Blocked && issue.leads.contains(&discord_user_id)
    });
    for issue in user_issues {
        if !any_issues {
            if any_prs {
                msg.push_str(&format!("\n\n{}", BLOCKING_ISSUES_HEADER));
            } else {
                msg.push_str(&format!("{}{}", BLOCKING_ISSUES_HEADER, discord_user_id));
            }
        }
        msg.push_str(&format!("\n* {}", format_issue(&issue)));
        any_issues = true;
    }

    if any_prs || any_issues {
        let msg = serenity::CreateMessage::new().content(msg);
        http.send_message(discord_channel_id.into(), vec![], &msg)
            .await?;
    }
    Ok(())
}

async fn report_weekly_alerts_for_user(
    http: Arc<serenity::Http>,
    issues: Arc<Vec<github::LeadsIssue>>,
    discord_channel_id: model::DiscordChannelId,
    discord_user_id: model::DiscordUserId,
) -> Result<(), DiscordError> {
    const NONURGENT_ISSUES_HEADER: &str = ":chipmunk: Open leads issues (non-blocking) ";

    delete_messages_with_prefix(
        http.clone(),
        discord_channel_id.clone(),
        format!("{}{}", NONURGENT_ISSUES_HEADER, discord_user_id),
    )
    .await?;

    let mut msg = format!("{}{}", NONURGENT_ISSUES_HEADER, discord_user_id);

    let user_issues = issues.iter().filter(|issue: &_| {
        issue.urgency == github::Urgency::Normal && issue.leads.contains(&discord_user_id)
    });

    let mut any_issues = false;
    for issue in user_issues {
        msg.push_str(&format!("\n* {}", format_issue(&issue)));
        any_issues = true;
    }

    if any_issues {
        let msg = serenity::CreateMessage::new().content(msg);
        http.send_message(discord_channel_id.into(), vec![], &msg)
            .await?;
    }
    Ok(())
}

pub async fn watch_github_wake_now(guild_id: model::DiscordGuildId) -> Result<(), DiscordError> {
    let guard = CANCEL_SLEEP.lock().await;
    if let Some(sender) = guard.as_ref() {
        match sender.send(guild_id).await {
            Ok(()) => {}
            Err(e) => return Err(e.to_string().into()),
        }
    }
    Ok(())
}
