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
