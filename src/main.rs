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

mod settings;
mod commands;

use log::{error, info};
use pretty_env_logger::formatted_builder;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        help_commands, macros::{group, help}, Args, CommandGroup, CommandResult, HelpOptions,
        StandardFramework,
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
use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;
use commands::misc::*;

use crate::settings::settings;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
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

        ctx.set_activity(Activity::listening(&format!("@{} help", ready.user.name)))
            .await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Reconnected")
    }
}

#[group]
#[commands(ping)]
struct Misc;

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

#[tokio::main]
async fn main() {
    // Initialize settings
    settings::init();

    // Setup logger
    let mut logger = formatted_builder();
    for (path, level) in &settings().logging.filters {
        logger.filter(Some(path), *level);
    }
    logger.init();

    let http = Http::new_with_token(&settings().bot.token);

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
            c.prefix(&settings().bot.prefix)
                .on_mention(Some(bot_id))
                .owners(owners)
                .allow_dm(false)
                .case_insensitivity(true)
        })
        .group(&MISC_GROUP)
        .help(&BOT_HELP);

    let mut client = Client::builder(&settings().bot.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Something went wrong while building the client.");

    {
        let mut data = client.data.write().await;

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
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
}
