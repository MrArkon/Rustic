use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::ShardManagerContainer;

#[command]
#[description = "Check if the bot is working"]
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
