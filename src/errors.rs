// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// src/errors.rs

use std::fmt;

#[derive(Debug)]
pub enum Error {
    Discord(Box<poise::serenity_prelude::Error>),
    Custom(std::string::String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Discord(e) => write!(
                f,
                "discord error: {e} please report this [here](https://tuxzilla.com/squash-a-bug) through proper channels"
            ),
            Error::Custom(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Custom(e.to_string())
    }
}

impl From<poise::serenity_prelude::Error> for Error {
    fn from(e: poise::serenity_prelude::Error) -> Self {
        Error::Discord(Box::new(e))
    }
}

impl From<std::string::String> for Error {
    fn from(e: std::string::String) -> Self {
        Error::Custom(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Error::Custom(e.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::Custom(e.to_string())
    }
}
