use rdev::{listen, Event, EventType};
//use timer::Timer;
use std::process::Command;

const TOP_BAR_HEIGHT : f64 = 55.0;
fn main() {
    let mut show_topbar_cmd: Command =
        Command::new("/home/sumeet/bin/bspwm-show-topbar");
    let mut hide_topbar_cmd =
        Command::new("/home/sumeet/bin/bspwm-hide-topbar");

    //let timer_maker = Timer::new();
    let mut was_already_top = None;
    let callback = move |event: Event| {
        match event.event_type {
            EventType::MouseMove { x: _, y } => {
                let mouse_in_top_now = y <= TOP_BAR_HEIGHT;
                match (mouse_in_top_now, was_already_top) {
                    (true, None | Some(false)) => {
                        show_topbar_cmd.output().unwrap();
                        was_already_top = Some(true);
                    }
                    (false, None | Some(true)) => {
                        hide_topbar_cmd.output().unwrap();
                        was_already_top = Some(false);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    };
    listen(callback).unwrap()
}