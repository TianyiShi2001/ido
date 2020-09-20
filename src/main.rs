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

use chrono::Local;
use ido::{data::*, utils::*, config::{config_filename, Config, PROJECT_DIRS}};
use std::{time::Duration, thread};
use cursive::{traits::*, align::HAlign, views::{Dialog, EditView, LinearLayout, Panel, TextView}, Cursive};


fn main() {
    let mut siv = cursive::default();

    init(&mut siv);

    on_start(&mut siv);

    siv.run();
}

fn new_task(s: &mut Cursive) {
    fn ok(s: &mut Cursive, task: &str) {
        s.call_on_name("task_pane", |view: &mut TextView| {
            view.set_content(task);
        });
        s.pop_layer();
        s.user_data::<UserData>().unwrap().record.task = task.to_owned();
        start_timing(s);
    }
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok) // when pressing <Enter>
                .with_name("task")
                .fixed_width(20),
        )
        .title("Enter task name"),
    );
}

fn start_timing(s: &mut Cursive) {
    let cb_sink = s.cb_sink().clone();
    let start_time = Local::now();
    s.user_data::<UserData>().unwrap().record.start = start_time.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let now = Local::now();
        let elapsed = now.signed_duration_since(start_time);
        cb_sink
            .send(Box::new(move |s| {
                s.call_on_name("current_time", |view: &mut TextView| {
                    view.set_content(format!("{}", now.format("%H:%M:%S")));
                });
            }))
            .unwrap();
        cb_sink
            .send(Box::new(move |s| {
                s.call_on_name("time_elapsed", |view: &mut TextView| {
                    view.set_content(elapsed.pretty());
                });
            }))
            .unwrap();
    });
}

fn rating(s: &mut Cursive) {
    // how would you rate your task}
    unimplemented!()
}

// initialization; constant routines every time
fn init(s: &mut Cursive) {
    // global callbacks
    s.add_global_callback('q', |s: &mut Cursive| on_exit(s));
    s.add_global_callback('n', |s: &mut Cursive| new_task(s));

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

    let view = Dialog::around(view).h_align(HAlign::Center).button("New task (n)", |s: &mut Cursive| new_task(s)).button("Quit (q)",|s: &mut Cursive| on_exit(s));
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
                get_config(s);
            }
        }
        Err(_) => {
            get_config(s);
        }
    }
}

fn get_config(s: &mut Cursive) {
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

fn save_task(s: &mut Cursive) -> Result<(), BoxedError> {
    let mut record = s.user_data::<UserData>().unwrap().record.clone();
    let start = record.start;
    let now = Local::now();
    let duration = now.signed_duration_since(start).num_seconds() as u32;
    record.end = now;
    record.duration = duration;

    let log_file_path = datafile_path(s, "log.json");
    std::fs::create_dir_all(&s.user_data::<UserData>().unwrap().config.data_dir)?;
    let mut records: Vec<Record> = serde_json::from_str(
        &std::fs::read_to_string(&log_file_path).unwrap_or_else(|_| "[]".to_owned()),
    )?;
    records.push(record.clone());
    let records = serde_json::to_string(&records)?;
    std::fs::write(&log_file_path, &records)?;

    Ok(())
}

fn save_config(s: &mut Cursive) -> Result<(), BoxedError> {
    let config = &s.user_data::<UserData>().unwrap().config;
    let config = serde_yaml::to_string(config)?; // YAMLified config
    std::fs::create_dir_all(PROJECT_DIRS.config_dir())?;
    std::fs::write(config_filename(), &config)?;
    Ok(())
}

// finish one task
fn on_finish(s: &mut Cursive) {
    // save data
    // continue another task or to exit
    unimplemented!();
}

// before exiting the program
fn on_exit(s: &mut Cursive) {
    save_task(s).unwrap();
    save_config(s).unwrap();
    s.quit();
}

fn datafile_path(s: &mut Cursive, filename: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(&s.user_data::<UserData>().unwrap().config.data_dir);
    dir.join(filename)
}
