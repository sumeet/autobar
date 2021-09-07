use rdev::{listen, Event, EventType};
//use timer::Timer;
use std::process::Command;

const TOP_BAR_SHOW_THRESHOLD : f64 = 0.0;
const TOP_BAR_HIDE_THRESHOLD : f64 = 70.0;

fn main() {
    let mut show_topbar_cmd: Command =
        Command::new("/home/sumeet/bin/bspwm-show-topbar");
    let mut hide_topbar_cmd =
        Command::new("/home/sumeet/bin/bspwm-hide-topbar");

    let mut is_top_bar_shown = None;
    let callback = move |event: Event| {
        match event.event_type {
            EventType::MouseMove { x: _, y } => {
                match is_top_bar_shown {
                    None | Some(false) if y <= TOP_BAR_SHOW_THRESHOLD => {
                        show_topbar_cmd.output().unwrap();
                        is_top_bar_shown = Some(true);
                    }
                    None | Some(true) if y > TOP_BAR_HIDE_THRESHOLD => {
                        hide_topbar_cmd.output().unwrap();
                        is_top_bar_shown = Some(false);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    };
    listen(callback).unwrap()
}