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

use log::LevelFilter;
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use typemap::Key;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub bot: BotSettings,
    pub logging: LoggingSettings,
}

#[derive(Debug, Deserialize)]
pub struct BotSettings {
    pub token: String,
    pub prefix: String,
    pub application_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct LoggingSettings {
    pub level: LevelFilter,
    pub filters: HashMap<String, LevelFilter>,
}

impl Key for Settings {
    type Value = Arc<Mutex<Settings>>;
}

pub fn init() -> Settings {
    let config = std::fs::read_to_string("config.toml")
        .expect("Something went wrong while trying to parse the configuration file");
    toml::from_str(&config)
        .expect("Something went wrong while trying to deserialize the configuration file.")
}
