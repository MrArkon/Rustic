// Copyright (C) 2021 MrArkon

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use simple_process_stats::ProcessStats;

use crate::ShardManagerContainer;

#[command]
#[description = "Check if the bot is working."]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data_read = ctx.data.read().await;

    let shard_manager = match data_read.get::<ShardManagerContainer>() {
        Some(s) => s,
        None => {
            msg.reply(ctx, "Something went wrong, Please try again later.")
                .await?;

            return Ok(());
        }
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let latency = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => match runner.latency {
            Some(ms) => format!("{}ms", ms.as_millis()),
            _ => "?ms".to_string(),
        },
        None => {
            msg.reply(ctx, "Something went wrong, Please try again.")
                .await?;

            return Ok(());
        }
    };

    let icon_url = ctx.cache.current_user().await.face();

    msg.channel_id
        .send_message(ctx, |message| {
            message.embed(|embed| {
                embed.author(|author| {
                    author.name("Pong!");
                    author.icon_url(icon_url);
                    author
                });
                embed.description(&format!("**Shard {}**: {}", ctx.shard_id + 1, latency));
                embed.color(0xF05B4A);
                embed
            });
            message
        })
        .await?;

    Ok(())
}

#[command]
#[aliases("statistics", "stats")]
#[description = "Tells you information about the bot itself."]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let discriminator = ctx.cache.current_user().await.discriminator;
    let icon_url = ctx.cache.current_user().await.face();

    let total_guilds = ctx.cache.guilds().await.len();
    let total_shards = ctx.cache.shard_count().await;
    let total_users = ctx.cache.user_count().await;

    let process_stats = ProcessStats::get()
        .await
        .expect("Couldn't get statistics for the running process");

    msg.channel_id
        .send_message(ctx, |message| {
            message.embed(|embed| {
                embed.author(|author| {
                    author.name(format!("Rustic#{}", discriminator));
                    author.icon_url(icon_url);
                    author
                });
                embed.description(
                    "
                    Rustic is an open source multi-purpose bot packed with features
                    You can find my source code on [github](https://github.com/MrArkon/Rustic), Developed by [MrArkon](https://mrarkon.github.io)
                ",
                );
                embed.fields(vec![
                    ("Guilds", total_guilds.to_string(), true),
                    ("Users", total_users.to_string(), true),
                    ("Shard", format!("{}/{}", ctx.shard_id + 1, total_shards), true),
                    (
                        "Memory Usage",
                        format!("{} MB", process_stats.memory_usage_bytes / (1024 * 1024)),
                        true,
                    )
                ]);
                embed.color(0xF05B4A);
                embed.footer(|f| f.text("Written with Rust & Serenity-rs"));
                embed
            });
            message
        })
        .await?;

    Ok(())
}
