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

use crate::PgPoolContainer;
use log::error;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use sqlx::query;

#[command]
#[only_in(guilds)]
#[usage = "[prefix]"]
#[description = "Not providing the prefix will show the current prefix. Providing the prefix will set the it to that prefix. To use spaces in your prefix surround it with double quotation marks \"an example \""]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let new_prefix = args.single_quoted::<String>().unwrap();

    if new_prefix.is_empty() {
        let prefix;
        let guild_id = msg.guild_id.unwrap().0 as i64;

        let pool = {
            let data = ctx.data.read().await;
            data.get::<PgPoolContainer>().unwrap().clone()
        };

        match query!("SELECT prefix FROM guilds WHERE guild_id=$1", guild_id)
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

        msg.channel_id
            .say(
                &ctx.http,
                format!("The current guild prefix is: `{}`", prefix),
            )
            .await?;

        return Ok(());
    }

    let pool = {
        let data = ctx.data.read().await;
        data.get::<PgPoolContainer>().unwrap().clone()
    };

    let guild_id = msg.guild_id.unwrap().0 as i64;
    query!("INSERT INTO guilds (guild_id, prefix) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET prefix = $2 WHERE guilds.guild_id = $1", guild_id, new_prefix)
        .execute(&pool)
        .await?;

    msg.channel_id
        .say(&ctx.http, format!("Updated prefix to: `{}`", new_prefix))
        .await?;

    Ok(())
}
