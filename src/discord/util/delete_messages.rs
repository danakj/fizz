// Part of the Carbon Language project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::sync::Arc;

use poise::serenity_prelude as serenity;

use crate::discord::DiscordError;
use crate::model;

pub async fn delete_messages<F: FnMut(&serenity::Message) -> bool>(
    http: Arc<serenity::Http>,
    discord_channel_id: model::DiscordChannelId,
    mut f: F,
) -> Result<(), DiscordError> {
    let channel_id: serenity::ChannelId = discord_channel_id.into();

    loop {
        // TODO: Do stuff with MessagePagination to avoid asking for the same
        // messages repeatedly.
        let messages = http.get_messages(channel_id, None, None).await?;
        if messages.is_empty() {
            break;
        }

        let mut deleted = false;
        for m in messages {
            if f(&m) {
                http.delete_message(channel_id, m.id, None).await?;
                deleted = true;
            }
        }
        if !deleted {
            break;
        }
    }

    Ok(())
}
