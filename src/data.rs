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
use crate::config::Config;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct UserData {
    pub record: Record,
    pub config: Config,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub task: String, // not `&'a str` for simplicity. There won't be too much data anyways.
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub duration: u32,
    pub efficiency: f32, // is it positive or negative? to what extent?
}

impl std::default::Default for Record {
    fn default() -> Self {
        Record {
            task: String::default(),
            start: Local::now(),
            end: Local::now(),
            duration: 0u32,
            efficiency: 0f32,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    task: String,
    categories: Vec<String>,
    tags: Vec<String>,
}
