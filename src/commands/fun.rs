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

use rand::{self, Rng};
use serde::Deserialize;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::ReqwestContainer;

#[command]
#[bucket = "basic"]
#[description = "Find some cute cat pictures!"]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let client = ctx
        .data
        .read()
        .await
        .get::<ReqwestContainer>()
        .cloned()
        .unwrap();
    let request = client.get("https://shibe.online/api/cats").send().await?;

    if request.status() != 200 {
        msg.channel_id
            .say(&ctx.http, "Something went wrong, please try again later.")
            .await?;

        return Ok(());
    }

    let response: Vec<String> = request.json().await?;

    msg.channel_id
        .send_message(ctx, |message| {
            message.embed(|embed| {
                embed.title("Have a cute cat!");
                embed.image(&response[0]);
                embed.color(0xF05B4A);
                embed
            });
            message
        })
        .await?;

    Ok(())
}

#[command]
#[min_args(1)]
#[bucket = "basic"]
#[usage = "<question>"]
#[aliases("8ball", "8b")]
#[description = "Ask a question to the magic 8ball"]
async fn eightball(ctx: &Context, msg: &Message) -> CommandResult {
    let responses: [&str; 20] = [
        "It is certain.",
        "It is decidedly so.",
        "Without a doubt.",
        "Yes definitely.",
        "You may rely on it.",
        "As I see it, yes.",
        "Most likely.",
        "Outlook good.",
        "Yes.",
        "Signs point to yes.",
        "Reply hazy, try again.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don't count on it.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Very doubtful.",
    ];

    msg.reply(
        &ctx.http,
        &format!(
            ":8ball: **8ball:** {}",
            responses[rand::thread_rng().gen_range(0..=20)]
        ),
    )
    .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct UrbanResponse {
    list: Vec<Definition>,
}

#[derive(Debug, Deserialize)]
struct Definition {
    definition: String,
    permalink: String,
    word: String,
    author: String,
    thumbs_up: u32,
    thumbs_down: u32,
    written_on: String,
}

#[command]
#[bucket = "basic"]
#[usage = "<word>"]
#[description = "Searches urban dictionary."]
async fn urban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        msg.channel_id
            .say(&ctx.http, "Give me a word to look up for.")
            .await?;
        return Ok(());
    }

    let word = args.rest();

    let client = ctx
        .data
        .read()
        .await
        .get::<ReqwestContainer>()
        .cloned()
        .unwrap();

    let request = client
        .get("http://api.urbandictionary.com/v0/define")
        .query(&[("term", word)])
        .send()
        .await?;

    if request.status() != 200 {
        msg.channel_id
            .say(&ctx.http, "Something went wrong, please try again later.")
            .await?;

        return Ok(());
    }

    let response: UrbanResponse = request.json().await?;

    if response.list.is_empty() {
        msg.channel_id
            .say(&ctx.http, "No results found, sorry.")
            .await?;
        return Ok(());
    }

    let definition = response.list.get(0).unwrap();

    msg.channel_id
        .send_message(ctx, |message| {
            message.embed(|embed| {
                embed.title(&definition.word);
                embed.url(&definition.permalink);
                embed.description(&definition.definition);
                embed.field(
                    "Votes",
                    format!(
                        ":thumbsup: {} :thumbsdown: {}",
                        &definition.thumbs_up, &definition.thumbs_down
                    ),
                    false,
                );
                embed.footer(|f| f.text(format!("by {}", &definition.author)));
                embed.color(0xF05B4A);
                embed
            });
            message
        })
        .await?;

    Ok(())
}
