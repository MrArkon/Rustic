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

use config::{Config, ConfigError, File};
use log::LevelFilter;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::collections::HashMap;

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

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.set_default("logging.level", "WARN")?;
        s.set_default("logging.filters.rustic", "INFO")?;

        s.merge(File::with_name("config.toml")).unwrap();

        s.try_into()
    }
}

static SETTINGS: OnceCell<Settings> = OnceCell::new();

pub fn settings() -> &'static Settings {
    SETTINGS.get().expect("Settings were not initialized")
}

pub fn init() {
    match Settings::new() {
        Ok(settings) => {
            let _ = SETTINGS.set(settings);
        }
        Err(e) => {
            panic!("Failed to parse settings: {}", e);
        }
    }
}
