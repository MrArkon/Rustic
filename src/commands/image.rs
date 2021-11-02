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

use crate::ReqwestContainer;
use serenity::utils::ArgumentConvert;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    http::AttachmentType,
    model::prelude::{Member, Message},
    prelude::Context,
};
use std::borrow::Cow;

#[command]
#[usage = "[member]"]
#[bucket = "basic"]
#[description = "Adds a grayscale filter to your avatar or the mentioned member."]
#[aliases("gray", "grey", "greyscale")]
async fn grayscale(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = {
        if args.message().is_empty() {
            msg.author.face()
        } else {
            let m = <Member as ArgumentConvert>::convert(
                ctx,
                msg.guild_id,
                Some(msg.channel_id),
                args.message(),
            )
            .await?;
            args.advance();
            m.face()
        }
    };

    let client = ctx
        .data
        .read()
        .await
        .get::<ReqwestContainer>()
        .cloned()
        .unwrap();
    let image_bytes = client
        .get(&url)
        .send()
        .await?
        .bytes()
        .await?
        .into_iter()
        .collect::<Vec<u8>>();

    let mut bytes = Vec::new();
    let buffer = libwebp_image::webp_load_from_memory(&image_bytes)?.into_rgba8();

    image::DynamicImage::ImageRgba8(buffer)
        .grayscale()
        .write_to(&mut bytes, image::ImageOutputFormat::Png)?;

    let attachment = AttachmentType::Bytes {
        data: Cow::from(bytes),
        filename: String::from("grayscale.png"),
    };

    msg.channel_id
        .send_message(ctx, |message| {
            message.add_file(attachment);
            message
        })
        .await?;

    Ok(())
}
