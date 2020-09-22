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

use crate::data::*;
use crate::utils::BoxedError;
use serde::de::DeserializeOwned;
use serde::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Records<T: DeserializeOwned + Serialize> {
    pub path: PathBuf,
    pub items: Vec<T>,
}

impl<T: DeserializeOwned + Serialize> Records<T> {
    pub fn load<P: AsRef<Path>>(dir: P, filename: &str) -> Result<Self, BoxedError> {
        let path = dir.as_ref().join(filename);
        let content = std::fs::read_to_string(&path).unwrap_or_else(|_| "[]".to_owned()); // TODO: check corrupted json
        let items: Vec<T> = serde_json::from_str(&content)?;
        Ok(Records { path, items })
    }
    pub fn save(&self) -> Result<(), BoxedError> {
        let values = serde_json::to_string(&self.items)?;
        std::fs::write(&self.path, &values)?;
        Ok(())
    }
    pub fn append_and_save(&mut self, item: T) -> Result<(), BoxedError> {
        self.items.push(item);
        self.save()
    }
}

impl Records<Record> {
    pub fn frequent_tasks(&self) -> Vec<String> {
        // TODO: &str
        let mut count: HashMap<&str, usize> = HashMap::new();
        for ent in &self.items {
            let c = count.entry(&ent.task).or_insert(0);
            *c += 1;
        }
        let mut count_vec = count.into_iter().collect::<Vec<_>>();
        count_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        count_vec.into_iter().map(|t| t.0.to_owned()).collect()
    }
}

pub fn load_log<P: AsRef<Path>>(dir: P) -> Result<Records<Record>, BoxedError> {
    Records::load(dir, "log.json")
}
