// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use octocrab::models::issues::Issue;
use octocrab::params::issues::Sort;
use octocrab::params::Direction;
use octocrab::params::State;

use crate::error::Error;
use crate::model;

const GITHUB_URL_BASE: &str = "https://github.com";
const LABEL_LEADS_ISSUE: &str = "leads question";
const LABEL_BACKGROUND: &str = "long term issue";
const LABEL_BLOCKED: &str = "blocking work";

fn github_issue_url(cfg: &model::GuildConfig, number: u64) -> String {
    format!(
        "{}/{}/{}/pull/{}",
        GITHUB_URL_BASE, cfg.repo_owner, cfg.repo_name, number,
    )
}

pub struct LeadsIssueState {
    iter: std::vec::IntoIter<Issue>,
}

pub async fn get_leads_issues(repo_owner: &str, repo_name: &str) -> Result<LeadsIssueState, Error> {
    let labels = vec![LABEL_LEADS_ISSUE.to_string()];

    // TODO: This returns an iterator over the first page (first 100) issues only.
    let octo = octocrab::instance();
    let issue_handler = octo.issues(repo_owner, repo_name);
    let issues_builder = issue_handler
        .list()
        .state(State::Open)
        .labels(&labels)
        .per_page(100)
        .sort(Sort::Updated)
        .direction(Direction::Ascending);
    match issues_builder.send().await {
        Ok(bugs) => Ok(LeadsIssueState {
            iter: bugs.into_iter(),
        }),
        Err(e) => Err(Error::FailedToGetIssues(e)),
    }
}

#[derive(PartialEq, Eq)]
pub enum Urgency {
    Blocked,
    Normal,
    Background,
}

pub struct LeadsIssue {
    pub github_issue: Issue,
    pub url: String,
    pub urgency: Urgency,
    pub leads: Vec<model::DiscordUserId>,
}

pub fn filter_leads_issues_for_guild<'a>(
    issues: LeadsIssueState,
    cfg: &'a model::GuildConfig,
) -> impl std::iter::Iterator<Item = LeadsIssue> + 'a {
    issues.iter.map(|issue| -> LeadsIssue {
        let mut urgency = Urgency::Normal;
        for label in &issue.labels {
            if label.name == LABEL_BLOCKED {
                urgency = Urgency::Blocked;
            }
            if label.name == LABEL_BACKGROUND && urgency != Urgency::Blocked {
                urgency = Urgency::Background;
            }
        }
        let mut leads = Vec::new();
        for (discord_user_id, user_config) in &cfg.users {
            if user_config.lead {
                leads.push(discord_user_id.clone());
            }
        }

        LeadsIssue {
            url: github_issue_url(cfg, issue.number),
            github_issue: issue,
            urgency,
            leads,
        }
    })
}
