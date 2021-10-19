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

mod commands;
mod settings;

use commands::{config::*, fun::*, misc::*};
use log::{error, info};
use pretty_env_logger::formatted_builder;
use reqwest::Client as ReqwestClient;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        help_commands,
        macros::{group, help, hook},
        Args, CommandGroup, CommandResult, DispatchError, HelpOptions, StandardFramework,
    },
    http::Http,
    model::{
        channel::Message,
        event::ResumedEvent,
        gateway::Ready,
        id::UserId,
        prelude::{Activity, GuildId},
    },
    prelude::{Client, Context, EventHandler, TypeMapKey},
};
use sqlx::{postgres::PgPoolOptions, query, PgPool};
use std::{collections::HashSet, error::Error, sync::Arc};
use tokio::sync::Mutex;

struct ShardManagerContainer;
struct ReqwestContainer;
struct PgPoolContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for ReqwestContainer {
    type Value = ReqwestClient;
}

impl TypeMapKey for PgPoolContainer {
    type Value = PgPool;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, _: Context, guilds: Vec<GuildId>) {
        info!("Connected to {} guilds.", guilds.len());
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            info!(
                "Connected as [Name: {}] [ID: {}] [Shard: {}/{}]",
                ready.user.name,
                ready.user.id,
                shard[0] + 1,
                shard[1]
            );
        } else {
            info!(
                "Connected as [Name: {}] [ID: {}]",
                ready.user.name, ready.user.id
            );
        }

        ctx.set_activity(Activity::listening("@Rustic help")).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Reconnected")
    }
}

#[group]
#[commands(ping, about)]
struct Misc;

#[group]
#[commands(cat, eightball, urban)]
struct Fun;

#[group]
#[commands(prefix)]
struct Configuration;

#[help]
#[max_levenshtein_distance(2)]
async fn bot_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let ho = help_options.clone();
    let _ = help_commands::with_embeds(ctx, msg, args, &ho, groups, owners).await;
    Ok(())
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, name: &str, result: CommandResult) {
    match result {
        Ok(()) => {}
        Err(why) => error!("Command '{}' returned an error {:?}", name, why),
    }
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(info) = error {
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    &format!(
                        ":hourglass: | **Cooldown:** Try this again in {} seconds.",
                        info.as_secs()
                    ),
                )
                .await;
        }
    }
}

#[hook]
async fn dynamic_prefix(ctx: &Context, msg: &Message) -> Option<String> {
    let prefix;

    if let Some(id) = &msg.guild_id {
        let pool = {
            let data = ctx.data.read().await;
            data.get::<PgPoolContainer>().unwrap().clone()
        };

        match query!("SELECT prefix FROM guilds WHERE guild_id=$1", id.0 as i64)
            .fetch_optional(&pool)
            .await
        {
            Ok(response) => {
                prefix = if let Some(result) = response {
                    result.prefix.unwrap_or_else(|| "~".to_string())
                } else {
                    "~".to_string()
                };
            }
            Err(why) => {
                error!("Couldn't query database for prefix: {}", why);
                prefix = "~".to_string();
            }
        };
    } else {
        // No prefix in dms
        prefix = "".to_string();
    };

    Some(prefix)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize settings
    let settings = settings::init();

    // Setup logger
    let mut logger = formatted_builder();
    for (path, level) in &settings.logging.filters {
        logger.filter(Some(path), *level);
    }
    logger.init();

    let http = Http::new_with_token(&settings.bot.token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.on_mention(Some(bot_id))
                .dynamic_prefix(dynamic_prefix)
                .owners(owners)
                .case_insensitivity(true)
        })
        .bucket("basic", |b| b.time_span(5).limit(1))
        .await
        .after(after)
        .on_dispatch_error(dispatch_error)
        .group(&MISC_GROUP)
        .group(&FUN_GROUP)
        .group(&CONFIGURATION_GROUP)
        .help(&BOT_HELP);

    let mut client = Client::builder(&settings.bot.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Something went wrong while building the client.");

    client.cache_and_http.cache.set_max_messages(256).await;

    {
        let mut data = client.data.write().await;
        let reqwest_client = ReqwestClient::builder()
            .user_agent(format!(
                "Rustic/{} (https://github.com/MrArkon/Rustic)",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;
        let pool = PgPoolOptions::new()
            .max_connections(settings.database.max_connections)
            .connect(&settings.database.url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Couldn't run database migrations");

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ReqwestContainer>(reqwest_client);
        data.insert::<PgPoolContainer>(pool);
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Something went wrong while registering Ctrl+C handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start_autosharded().await {
        error!("Something went wrong while starting the client: {:?}", why);
    }

    Ok(())
}
