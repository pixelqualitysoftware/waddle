// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// This is the main file, if you'd like to contribute to botplate, please read the CONTRIBUTING.md file.

#![allow(clippy::unreadable_literal)]
#![allow(clippy::print_stdout)]
use poise::serenity_prelude::{self as serenity};
use std::time::Instant;

mod errors;
mod etc;
mod global;

pub struct Data {
    pub admins: Vec<u64>,
    pub start_time: Instant,
}

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    let start = Instant::now();
    println!("starting waddle!");
    dotenvy::dotenv().ok();

    let admins = std::env::var("ADMINS")
        .map_err(|_| errors::Error::Custom("ADMINS not set in env".into()))?;

    let admins: Vec<u64> = admins
        .split(',')
        .map(|s| {
            s.parse()
                .map_err(|_| errors::Error::Custom("ADMIN is not a valid integer".into()))
        })
        .collect::<Result<Vec<u64>, _>>()?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![etc::general::ping(), etc::general::info()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("b.".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {
                    admins,
                    start_time: start,
                })
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN")
        .map_err(|_| errors::Error::Custom("DISCORD_TOKEN not set in env".into()))?;

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    let elapsed_time = start.elapsed();
    println!("started!");
    println!("took: {} ms", elapsed_time.as_millis());

    client.start().await?;
    Ok(())
}
