use rdev::{listen, Event, EventType};
use std::ops::RangeInclusive;
use std::process::Command;
use xrandr::XHandle;

const TOP_BAR_SHOW_THRESHOLD: f64 = 0.0;
const TOP_BAR_HIDE_THRESHOLD: f64 = 140.0;

#[derive(Debug)]
struct PrimaryMonitor {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl PrimaryMonitor {
    fn xrandr_query() -> Self {
        let monitor_index = std::env::args().nth(1).map(|s| s.parse::<usize>().unwrap());

        let monitors = XHandle::open().unwrap().monitors().unwrap();

        let primary = if monitors.len() > 1 {
            if let Some(monitor_index) = monitor_index {
                monitors.into_iter().nth(monitor_index).unwrap()
            } else {
                monitors.into_iter().find(|m| m.is_primary).unwrap()
            }
        } else {
            monitors.into_iter().next().unwrap()
        };

        Self {
            x: primary.x..=primary.x + primary.width_px,
            y: primary.y..=primary.y + primary.height_px,
        }
    }

    fn clamp(&self, (x, y): (f64, f64)) -> Option<(f64, f64)> {
        (self.x.contains(&(x as _)) && self.y.contains(&(y as _)))
            .then(|| (x - *self.x.start() as f64, y - *self.y.start() as f64))
    }
}

fn main() {
    dbg!(XHandle::open()
        .unwrap()
        .monitors()
        .unwrap()
        .into_iter()
        .map(|m| { (m.name, m.x, m.y, m.is_primary) })
        .collect::<Vec<_>>());

    let primary = PrimaryMonitor::xrandr_query();
    dbg!(&primary);

    let mut show_topbar_cmd: Command = Command::new("/home/sumeet/bin/bspwm-show-topbar");
    let mut hide_topbar_cmd = Command::new("/home/sumeet/bin/bspwm-hide-topbar");

    let mut is_top_bar_shown = None;
    let callback = move |event: Event| {
        let (_, y) = match event.event_type {
            EventType::MouseMove { x, y } => match primary.clamp((x, y)) {
                Some(pos) => pos,
                None => return,
            },
            _ => return,
        };

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
    };
    listen(callback).unwrap()
}
