use crate::{data::*, datafiles::load_log, utils::*};
use chrono::Local;
use cursive::{
    align::HAlign,
    traits::*,
    views::{Dialog, EditView, LinearLayout, SelectView, TextView},
    Cursive,
};
use std::{thread, time::Duration};

pub fn new_task(siv: &mut Cursive) {
    fn ok(s: &mut Cursive, task: &str) {
        s.call_on_name("task_pane", |view: &mut TextView| {
            view.set_content(task);
        });
        s.pop_layer();
        s.user_data::<UserData>().unwrap().record.task = task.to_owned();
        start_timing(s);
    }
    let dir = &siv.user_data::<UserData>().unwrap().config.data_dir;
    let records = load_log(dir).unwrap();
    let items = records.frequent_tasks();
    let items_clone = items.clone();
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                // the query box is on the top
                .child(
                    EditView::new()
                        // update results every time the query changes
                        .on_edit(move |siv: &mut Cursive, query: &str, _cursor: usize| {
                            let matches = search_fn(&items, query);
                            // Update the `matches` view with the filtered array of cities
                            siv.call_on_name("matches", |v: &mut SelectView| {
                                v.clear();
                                v.add_all_str(matches);
                            });
                        })
                        // submit the focused (first) item of the matches
                        .on_submit(|siv: &mut Cursive, query: &str| {
                            let matches = siv.find_name::<SelectView>("matches").unwrap();
                            if matches.is_empty() {
                                ok(siv, query);
                            } else {
                                let task = &*matches.selection().unwrap();
                                ok(siv, task);
                            };
                        })
                        .with_name("query"),
                )
                // search results below the input
                .child(
                    SelectView::new()
                        // shows all cities by default
                        .with_all_str(items_clone)
                        // Sets the callback for when "Enter" is pressed.
                        //.on_submit(show_next_window)
                        // Center the text horizontally
                        .h_align(HAlign::Center)
                        .with_name("matches")
                        .scrollable()
                        .fixed_size((20, 10)),
                )
                .fixed_width(25),
        )
        .button("Cancel", |s| {
            s.pop_layer();
        })
        .title("Enter task name"),
    );

    siv.run();
}

// Filter cities with names containing query string. You can implement your own logic here!
fn search_fn<'a, 'b, T: std::iter::IntoIterator<Item = &'a String>>(
    items: T,
    query: &'b str,
) -> Vec<&'a String> {
    items
        .into_iter()
        .filter(|&item| {
            let item = item.to_lowercase();
            let query = query.to_lowercase();
            item.contains(&query)
        })
        .collect()
}

fn start_timing(s: &mut Cursive) {
    let cb_sink = s.cb_sink().clone();
    let start_time = Local::now();
    s.user_data::<UserData>().unwrap().record.start = start_time.clone();
    thread::spawn(move || loop {
        // if s.user_data::<UserData>().unwrap().stop_timer {
        //     break;
        // };
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

fn efficiency_rating(s: &mut Cursive)
// where
//     F: FnOnce(&mut Cursive) -> (),
{
    // how would you rate your task}
    fn ok(s: &mut Cursive, efficiency: &str) {
        s.pop_layer();
        match efficiency.parse::<f32>() {
            Err(_) => {
                draw_input(s);
                s.add_layer(Dialog::info("Fail to parse number!"));
            }
            Ok(eff) => {
                if eff < -5.0 || eff > 5.0 {
                    draw_input(s);
                    s.add_layer(Dialog::info("The efficieny should be between -5 and 5!"));
                } else {
                    s.user_data::<UserData>().unwrap().record.efficiency = eff;
                }
            }
        }
    }
    fn draw_input(s: &mut Cursive) {
        s.add_layer(
            Dialog::around(EditView::new().on_submit(ok).fixed_width(20))
                .title("Rate you task (a number between -5 and 5)"), // TODO: use a Panel
        );
    }
    draw_input(s);
}

pub fn finish_task(s: &mut Cursive) {
    efficiency_rating(s);
}
