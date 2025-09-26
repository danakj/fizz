// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use tokio::sync::Mutex;

use crate::model;

pub struct DiscordData {
    pub cfg: Mutex<model::Config>,
}

impl DiscordData {
    pub fn new(cfg: model::Config) -> Self {
        Self {
            cfg: Mutex::new(cfg),
        }
    }
}
