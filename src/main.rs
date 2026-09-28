// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/main.rs

#![allow(clippy::unreadable_literal)]
#![allow(clippy::print_stdout)]
use poise::serenity_prelude::{self as serenity};
use serenity::ChannelId;
use std::time::Instant;
use webhook::github_webhook;

mod errors;
mod etc;
mod global;
mod types;
mod webhook;

pub struct Data {
    pub start_time: Instant,
}

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    let start = Instant::now();
    println!("starting waddle!");
    dotenvy::dotenv().ok();

    let webhook_secret = std::env::var("GITHUB_WEBHOOK_SECRET")
        .map_err(|_| errors::Error::Custom("GITHUB_WEBHOOK_SECRET not set in env".into()))?;
    let webhook_address =
        std::env::var("GITHUB_WEBHOOK_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());

    let target_channel_id: u64 = std::env::var("GITHUB_CHANNEL_ID")
        .map_err(|_| errors::Error::Custom("GITHUB_CHANNEL_ID not set in env".into()))?
        .parse()
        .map_err(|_| errors::Error::Custom("GITHUB_CHANNEL_ID is not a valid integer".into()))?;

    let mut github_events = github_webhook::start(webhook_address, webhook_secret).await?;

    let token = std::env::var("DISCORD_TOKEN")
        .map_err(|_| errors::Error::Custom("DISCORD_TOKEN not set in env".into()))?;

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

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

                Ok(Data { start_time: start })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    let http_client = client.http.clone();

    tokio::spawn(async move {
        let channel = ChannelId::new(target_channel_id);
        while let Some(event) = github_events.recv().await {
            println!(
                "Received GitHub event: {} (ID: {:?})",
                event.name, event.delivery_id
            );

            let embed = github_webhook::format_event(&event);

            if let Some((content, button)) = embed {
                let content = serenity::CreateMessage::default()
                    .embed(content)
                    .components(vec![button]);
                if let Err(err) = channel.send_message(&http_client, content).await {
                    eprintln!("Failed to send GitHub notification to Discord: {err}");
                }
            }
        }
    });

    let elapsed_time = start.elapsed();
    println!("started!");
    println!("took: {} ms", elapsed_time.as_millis());

    client.start().await?;
    Ok(())
}
