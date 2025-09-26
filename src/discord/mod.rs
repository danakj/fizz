// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

mod commands;
mod discord_data;
mod discord_error;
mod tasks;
mod util;

use std::sync::Arc;

use poise::{serenity_prelude as serenity, FrameworkContext};
use tokio::sync::Mutex;

use crate::error;

pub use discord_data::DiscordData;
use discord_error::DiscordError;

static MANAGER: Mutex<Option<Arc<serenity::ShardManager>>> = Mutex::const_new(None);

type DiscordContext<'a> = poise::Context<'a, Arc<DiscordData>, DiscordError>;

async fn on_setup(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<Arc<DiscordData>, DiscordError>,
    data: Arc<DiscordData>,
) -> Result<Arc<DiscordData>, DiscordError> {
    println!("Logged in as {}", ready.user.name);
    match poise::builtins::register_globally(ctx, &framework.options().commands).await {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("discord setup failed: {}", e).into());
        }
    }
    Ok(data)
}

async fn on_error<'a>(framework_error: poise::FrameworkError<'a, Arc<DiscordData>, DiscordError>) {
    match framework_error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            eprintln!(
                "ERROR: Running command `{}`: {}",
                ctx.command().name,
                error.reply.as_deref().unwrap_or(":no_entry: <no reply>"),
            );
            if let Some(details) = error.details {
                eprintln!("  {}", details);
            }

            // Send the error's reply back to the user.
            if let Some(reply) = error.reply {
                // Errors dropped here.
                match ctx
                    .send(
                        poise::CreateReply::default()
                            .content(format!(":no_entry: {}", reply))
                            .ephemeral(true),
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }

        other_error => {
            if let Err(e) = poise::builtins::on_error(other_error).await {
                eprintln!("ERROR: Handling error: {}", e)
            }
        }
    }
}

async fn on_event<'a>(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework_context: FrameworkContext<'a, Arc<DiscordData>, DiscordError>,
    _data: &Arc<DiscordData>,
) -> Result<(), DiscordError> {
    // println!("Event: {:?}", event);
    Ok(())
}

pub async fn run(data: Arc<DiscordData>) -> Result<(), error::Error> {
    let Ok(token) = std::env::var("DISCORD_TOKEN") else {
        return Err(error::Error::DiscordTokenMissing(
            "DISCORD_TOKEN".to_string(),
        ));
    };

    let options = poise::FrameworkOptions {
        commands: commands::all(),
        on_error: |error| Box::pin(on_error(error)),
        event_handler: |ctx, event, framework_context, data| {
            Box::pin(on_event(ctx, event, framework_context, data))
        },
        ..Default::default()
    };

    let setup_data = data.clone();
    let framework = poise::Framework::builder()
        .options(options)
        .setup(move |ctx, ready, framework| Box::pin(on_setup(ctx, ready, framework, setup_data)))
        .build();

    let intents =
        serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::GUILD_INTEGRATIONS;
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("discord client builder failed");

    tokio::spawn(set_manager(client.shard_manager.clone()));
    tokio::spawn(tasks::watch_github(client.http.clone(), data.clone()));

    match client.start().await {
        Ok(()) => Ok(()),
        Err(e) => return Err(error::Error::DiscordConnectFailed(e)),
    }
}

async fn set_manager(manager: Arc<serenity::ShardManager>) {
    let mut lock = MANAGER.lock().await;
    lock.replace(manager);
}

pub async fn stop() {
    match MANAGER.lock().await.as_ref() {
        Some(manager) => manager.shutdown_all().await,
        // Try again if we raced with setting the MANAGER.
        None => Box::pin(stop()).await,
    }
}
