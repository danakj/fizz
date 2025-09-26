// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use octocrab::models::pulls::PullRequest;
use octocrab::params::pulls::Sort;
use octocrab::params::Direction;
use octocrab::params::State;

use crate::error::Error;
use crate::model;

const GITHUB_URL_BASE: &str = "https://github.com";

fn github_pr_url(cfg: &model::GuildConfig, number: u64) -> String {
    format!(
        "{}/{}/{}/{}",
        GITHUB_URL_BASE, cfg.repo_owner, cfg.repo_name, number,
    )
}

#[allow(dead_code)]
pub struct Reviewer {
    pub github_user: model::GithubUserName,
    pub discord_users: Vec<model::DiscordUserId>,
}

#[allow(dead_code)]
pub struct Pr {
    pub pr: PullRequest,
    pub url: String,
    pub reviewers: Vec<Reviewer>,
}

pub struct PrState {
    iter: std::vec::IntoIter<PullRequest>,
}

pub async fn get_prs(repo_owner: &str, repo_name: &str) -> Result<PrState, Error> {
    // TODO: This returns an iterator over the first page (first 100) PRs only.
    let prs_task = octocrab::instance()
        .pulls(repo_owner, repo_name)
        .list()
        .state(State::Open)
        .per_page(100)
        .sort(Sort::Updated)
        .direction(Direction::Ascending)
        .send()
        .await;
    match prs_task {
        Ok(prs) => Ok(PrState {
            iter: prs.into_iter(),
        }),
        Err(e) => Err(Error::FailedToGetPRs(e)),
    }
}

pub fn filter_prs_for_guild<'a>(
    prs: PrState,
    cfg: &'a model::GuildConfig,
) -> impl std::iter::Iterator<Item = Pr> + 'a {
    prs.iter.map(|pr| -> Pr {
        let mut reviewers = Vec::new();
        if let Some(rs) = &pr.requested_reviewers {
            for r in rs {
                let github_user: model::GithubUserName = r.into();

                let mut discord_users = Vec::new();
                for (discord_user_id, user_config) in &cfg.users {
                    if user_config.github_names.contains(&github_user) {
                        discord_users.push(discord_user_id.clone());
                    }
                }

                reviewers.push(Reviewer {
                    github_user,
                    discord_users,
                });
            }
        }

        Pr {
            url: github_pr_url(cfg, pr.number),
            pr,
            reviewers,
        }
    })
}
