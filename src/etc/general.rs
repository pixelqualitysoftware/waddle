// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/etc/general.rs

#![allow(clippy::unreadable_literal)]
use poise::serenity_prelude::{self as serenity};

use crate::etc::helpers;
use crate::global;

use crate::errors::Error;

/// ping the bot to check latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    ctx.say("fuck").await?;
    Ok(())
}

/// get information about waddle
#[poise::command(slash_command)]
pub async fn info(ctx: poise::Context<'_, crate::Data, Error>) -> Result<(), Error> {
    let Some(sys) = helpers::get_sysinfo() else {
        ctx.say("❌ Failed to retrieve system statistics.").await?;
        return Ok(());
    };

    let bot_uptime = ctx.data().start_time.elapsed().as_secs();

    let info_embed = serenity::CreateEmbed::new()
        .title("waddle info")
        .description("i dont have a desc")
        .field(
            "Bot Uptime",
            helpers::convert_uptime_2_human(bot_uptime),
            false,
        )
        .field(
            "Bot Memory",
            helpers::convert_bytes_2_megabytes(sys.bot_memory),
            false,
        )
        .field(
            "OS",
            sys.os_name.unwrap_or_else(|| "Unknown".to_string()),
            false,
        )
        .field(
            "Host Uptime",
            helpers::convert_uptime_2_human(sys.h_uptime),
            false,
        )
        .field(
            "Host Memory",
            format!(
                "{} / {}",
                helpers::convert_bytes_2_gigabytes(sys.h_used_memory),
                helpers::convert_bytes_2_gigabytes(sys.h_total_memory)
            ),
            false,
        )
        .footer(global::random_footer())
        .color(0x7289DA);

    ctx.send(poise::CreateReply::default().embed(info_embed))
        .await?;
    Ok(())
}
