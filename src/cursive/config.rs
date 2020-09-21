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

use crate::{
    config::{config_filename, PROJECT_DIRS},
    data::*,
    utils::*,
};
use cursive::{
    traits::*,
    views::{Dialog, EditView},
    Cursive,
};

pub fn get_config(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(|s: &mut Cursive, data_dir: &str| {
                    s.pop_layer();
                    s.user_data::<UserData>().unwrap().config.data_dir = data_dir.to_owned();
                })
                .with_name("directory")
                .fixed_width(30),
        )
        .title("Specify the directory to store data"),
    );
}

pub fn save_config(s: &mut Cursive) -> Result<(), BoxedError> {
    let config = &s.user_data::<UserData>().unwrap().config;
    let config = serde_yaml::to_string(config)?; // YAMLified config
    std::fs::create_dir_all(PROJECT_DIRS.config_dir())?;
    std::fs::write(config_filename(), &config)?;
    Ok(())
}
