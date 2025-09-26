// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

mod discord;
mod error;
mod github;
mod model;

use std::{process::ExitCode, sync::Arc};

use crate::error::Error;

#[tokio::main]
async fn main() -> ExitCode {
    let cfg = match model::load() {
        Ok(c) => c,
        Err(Error::ConfigFileMissing(_)) => model::new(),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let data = Arc::new(discord::DiscordData::new(cfg));

    println!("Running...");
    let result = loop {
        tokio::select! {
            result = discord::run(data.clone()) => {
                break result;
            }
            _ = tokio::signal::ctrl_c() => {
                println!("\nReceived interrupt signal, shutting down.");
                discord::stop().await;
                break Ok(())
            }
        }
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(Error::Silent) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return ExitCode::FAILURE;
        }
    }
}
