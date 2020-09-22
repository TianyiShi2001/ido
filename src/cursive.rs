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

use crate::{config::Config, data::*, datafiles::Records, utils::*};
use chrono::Local;
use cursive::{
    align::HAlign,
    traits::*,
    views::{Dialog, LinearLayout, Panel, TextView},
    Cursive,
};

mod config;
mod task;
mod utils;

pub fn run() {
    let mut siv = cursive::default();

    init(&mut siv);

    on_start(&mut siv);

    siv.run();
}

// initialization; constant routines every time
fn init(s: &mut Cursive) {
    // global callbacks
    s.add_global_callback('q', |s: &mut Cursive| on_exit(s));
    s.add_global_callback('n', |s: &mut Cursive| task::new_task(s));

    // build the UI
    let task_pane = Panel::new(TextView::new("").with_name("task_pane")).title("Task"); // information related to the current task; task name, categories, tags, etc.

    let current_time =
        Panel::new(TextView::new("").with_name("current_time")).title("current time");

    let time_elapsed =
        Panel::new(TextView::new("").with_name("time_elapsed")).title("time elapsed");

    let time_pane = LinearLayout::vertical()
        .child(time_elapsed.fixed_size((20, 5)))
        .child(current_time.fixed_size((20, 5)));

    let view = LinearLayout::horizontal()
        .child(task_pane.fixed_size((50, 10)))
        .child(time_pane.fixed_size((20, 10)));

    let view = Dialog::around(view)
        .h_align(HAlign::Center)
        .button("New task (n)", |s: &mut Cursive| task::new_task(s))
        .button("Quit (q)", |s: &mut Cursive| on_exit(s));
    s.add_layer(view);

    // init state
    s.set_user_data(UserData::default());
}

// start the program
fn on_start(s: &mut Cursive) {
    match Config::try_load() {
        Ok(config) => {
            if config.data_dir != "" {
                s.user_data::<UserData>().unwrap().config = config;
            } else {
                config::get_config(s);
            }
        }
        Err(_) => {
            config::get_config(s);
        }
    }
}

fn save_task(s: &mut Cursive) -> Result<(), BoxedError> {
    //task::finish_task(s);
    let mut record = s.user_data::<UserData>().unwrap().record.clone();
    s.user_data::<UserData>().unwrap().record = Record::default();
    let start = record.start;
    let now = Local::now();
    let duration = now.signed_duration_since(start).num_seconds() as u32;
    record.end = now;
    record.duration = duration;

    let dir = &s.user_data::<UserData>().unwrap().config.data_dir;
    let mut r = Records::<Record>::load(dir, "log.json")?;
    r.append_and_save(record)?;

    Ok(())
}

// finish one task
fn on_finish(s: &mut Cursive) {
    // save data
    save_task(s).unwrap();
    // continue another task or to exit
    unimplemented!();
}

// before exiting the program
fn on_exit(s: &mut Cursive) {
    //s.user_data::<UserData>().unwrap().stop_timer = true;
    save_task(s).unwrap();
    config::save_config(s).unwrap();
    s.quit();
}
