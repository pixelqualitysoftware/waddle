// Copyright (C) 2026 Tuxzilla <tuxzilla@tuxzilla.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// /src/etc/helpers.rs

use sysinfo::System;

// sysinfo wasnt satisfactory, so i made this dumb thing
fn get_pretty_name() -> Option<String> {
    let content = std::fs::read_to_string("/etc/os-release").ok()?;
    content
        .lines()
        .find_map(|line| line.strip_prefix("PRETTY_NAME="))
        .map(|v| v.trim_matches('"').to_string())
}

pub struct SysInfo {
    pub os_name: Option<String>,
    pub h_used_memory: u64,
    pub h_total_memory: u64,
    pub h_uptime: u64,
    pub bot_memory: u64,
}

pub fn get_sysinfo() -> Option<SysInfo> {
    let mut sys = System::new();
    sys.refresh_all();

    let pid = sysinfo::get_current_pid().ok()?;

    let bot_memory = sys.process(pid).map_or(0, sysinfo::Process::memory);

    // h = host if you cant read

    Some(SysInfo {
        os_name: get_pretty_name(),
        h_used_memory: sys.used_memory(),
        h_total_memory: sys.total_memory(),
        h_uptime: System::uptime(),
        bot_memory,
    })
}

#[allow(clippy::cast_precision_loss)]
pub fn convert_bytes_2_gigabytes(bytes: u64) -> String {
    format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
}

#[allow(clippy::cast_precision_loss)]
pub fn convert_bytes_2_megabytes(bytes: u64) -> String {
    format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
}

pub fn convert_uptime_2_human(uptime: u64) -> String {
    let seconds = uptime % 60;
    let minutes = (uptime / 60) % 60;
    let hours = (uptime / 3600) % 24;
    let days = uptime / 86400;
    format!("{days}d {hours}h {minutes}m {seconds}s")
}
