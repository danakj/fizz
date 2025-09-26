// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::error;
use poise::serenity_prelude as serenity;

#[derive(Debug)]
pub struct DiscordError {
    pub reply: Option<String>,
    pub details: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl DiscordError {
    pub fn new<T: Into<String>, E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>>(
        reply: T,
        details: E,
    ) -> Self {
        DiscordError {
            reply: Some(reply.into()),
            details: Some(details.into()),
        }
    }
}

impl std::error::Error for DiscordError {}
impl std::fmt::Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl From<String> for DiscordError {
    fn from(reply: String) -> Self {
        DiscordError {
            reply: Some(reply),
            details: None,
        }
    }
}
impl From<&str> for DiscordError {
    fn from(reply: &str) -> Self {
        DiscordError {
            reply: Some(reply.to_string()),
            details: None,
        }
    }
}
impl From<serenity::Error> for DiscordError {
    fn from(details: serenity::Error) -> Self {
        DiscordError {
            reply: None,
            details: Some(Box::new(details)),
        }
    }
}

impl From<error::Error> for DiscordError {
    fn from(details: error::Error) -> Self {
        DiscordError {
            reply: None,
            details: Some(Box::new(details)),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for DiscordError {
    fn from(details: std::sync::PoisonError<T>) -> Self {
        DiscordError {
            reply: None,
            details: Some(details.to_string().into()),
        }
    }
}
