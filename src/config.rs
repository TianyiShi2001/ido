// Copyright (C) 2020 Tianyi Shi
//
// This file is part of carpe-diem.
//
// carpe-diem is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// carpe-diem is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with carpe-diem.  If not, see <http://www.gnu.org/licenses/>.

// const PROGRAM_NAME: &'static str = "ido"

use crate::utils::BoxedError;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use serde::*;
use std::path::PathBuf;

lazy_static! {
    pub static ref PROJECT_DIRS: ProjectDirs = ProjectDirs::from("", "", "ido").unwrap();
}

pub fn config_filename() -> PathBuf {
    PROJECT_DIRS.config_dir().join("config.yml")
}

use config;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub data_dir: String,
}

impl Config {
    pub fn try_load() -> Result<Self, BoxedError> {
        let mut config = config::Config::new();
        config.merge(config::File::from(config_filename()))?;
        let config: Config = config.try_into()?;
        Ok(config)
    }
}
