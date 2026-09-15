// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/global.rs

use poise::serenity_prelude::CreateEmbedFooter;
use rand::RngExt;

// this could be a lot more elegant and nice to work with, but.. im a lazy ass.

// also i feel like this WILL be used
pub fn _is_admin(author: u64, admins: &[u64]) -> bool {
    admins.contains(&author)
}

// this? probably not.
pub fn _make_numbers_pretty(num: i64) -> String {
    let negative = num < 0;
    let s = num.unsigned_abs().to_string();
    let mut string = String::new();

    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            string.push(',');
        }
        string.push(ch);
    }

    let mut pretty: String = string.chars().rev().collect();
    if negative {
        pretty.insert(0, '-');
    }
    pretty
}

pub fn random_footer() -> CreateEmbedFooter {
    let mut rng = rand::rng();
    let version = env!("CARGO_PKG_VERSION");
    let messages = [
        "botplate-rs is cool",
        "check out our github repo!",
        "how random is random..?",
        "tuxzilla is in your walls",
        "yo yall seen those creepy footers??",
        "FOOTER",
        "dude why are you reading this",
        "billions must love",
        "wait what is this server again",
        "tuxzilla vs making a good bot",
        "mold -run cargo build --release",
        "this is like for some dev server, idk",
        "67676767",
    ];
    let message = messages[rng.random_range(0..messages.len())];
    CreateEmbedFooter::new(format!("{message} | waddle | {version}"))
}
