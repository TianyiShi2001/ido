use crate::{
    config::{config_filename, Config, PROJECT_DIRS},
    data::*,
    datafiles::Records,
    utils::*,
};
use chrono::Local;
use cursive::{
    align::HAlign,
    traits::*,
    views::{Dialog, EditView, LinearLayout, Panel, TextView},
    Cursive,
};
use std::{path::Path, thread, time::Duration};

pub fn new_task(s: &mut Cursive) {
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
        .button("Cancel", |s| {
            s.pop_layer();
        })
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
        // TODO: stop on pause/finish
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
