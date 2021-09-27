use rand::{self, Rng};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::ReqwestContainer;

#[command]
#[description = "Find some cute cat pictures!"]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let client = ctx
        .data
        .read()
        .await
        .get::<ReqwestContainer>()
        .cloned()
        .unwrap();
    let request = client.get("http://shibe.online/api/cats").send().await?;

    if request.status() != 200 {
        msg.channel_id
            .say(
                &ctx.http,
                "Something went wrong while trying to find a cat, please try again later.",
            )
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
#[aliases("8ball", "8b", "eightball")]
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
