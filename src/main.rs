extern crate time;

mod bar;
#[macro_use] mod component;
mod source;

use bar::Bar;

use component::core::{Component, ConcatComponent};
use component::format::*;

use source::*;

use std::thread;

macro_rules! def {
    ($x:ident, $y:expr) => {
        const $x : &'static str = $y;
    };
}

def!(BLACK, "#2D2B33");
def!(WHITE, "#FEFFFF");
def!(RED, "#C86C75");
def!(GREEN, "#D1E8B8");
def!(BLUE, "#757B8E");
def!(CYAN, "#B4BCC9");
def!(ORANGE, "#E47B66");

fn main() {
    let mut bar = bar::LemonBar::new();
    let power = dynamic!{
        init => power::PowerSource::new(),
        render(data: power::PowerSource) => {
            let (d, state) = data.get();
            let sym = match (d, state) {
                (_, power::BatteryStatus::Full) => "\u{00E200}",
                (_, power::BatteryStatus::Charging) => "\u{00E201}",
                (0.0...10.0, _) => "\u{00E211}",
                (10.0...25.0, _) => "\u{00E1FD}",
                (25.0...90.0, _) => "\u{00E1FE}",
                _ => "\u{00E1FF}"
            };
            component!(bg(if d > 10.0 { BLUE } else { RED }) => [" ", sym, format!(" {} ", d)])
        }
    };
    let bspwm = dynamic!{
        init => bspwm::BspwmSource::new(),
        render(data: bspwm::BspwmSource) => {
            let d = data.get();
            let mut v: Vec<Box<Component>> = Vec::new();
            for ref desktop in &d.desktops {
                v.push(Box::new(match (desktop.focus, desktop.occupied) {
                    (_, bspwm::OccupiedMode::Urgent)
                        => component!(bg(ORANGE) => [format!(" {} ", desktop.name)]),
                    (false, bspwm::OccupiedMode::Occupied)
                        => component!(bg(CYAN) => [format!(" {} ", desktop.name)]),
                    (true, _)
                        => component!(bg(GREEN) => [format!(" {} ", desktop.name)]),
                    (_, _)
                        => component!(bg(BLUE) => [format!(" {} ", desktop.name)])
                }));
            }
            ConcatComponent::new(v)
        }
    };
    let mpd = dynamic!{
        init => mpd::MpdSource::new("localhost:6700"),
        render(data: mpd::MpdSource) => {
            let d = data.get();
            match *d {
                mpd::MpdState::NotConnected =>
                    component!(bg(ORANGE) => [" \u{00E0B3} "]),
                mpd::MpdState::Error(ref e) =>
                    component!(bg(RED) => [format!(" \u{00E0B3}{} ", e)]),
                mpd::MpdState::Connected(ref status, ref song) => {
                    let t = time::now();
                    let secs: usize = t.tm_sec as usize + 60 * t.tm_min as usize;
                    match status.state {
                        mpd::State::Play => component!(bg(BLUE) => [
                            " \u{00E09A} ", match *song {
                                Some(ref s) => {
                                    let file = s.file.clone() + " ";
                                    let i = secs % file.len();
                                    let clen = 40;
                                    let out: String = file.chars().cycle().skip(i).take(clen).collect();
                                    component![format!("{} ", out)]
                                },
                                None => component!["-"]
                            }
                        ]),
                        _ => component!(bg(BLUE) => [" \u{00E057} "])
                    }
                }
            }
        }
    };
    let date = lazy!(time::strftime("\u{00E015} %a %m %b %H:%M:%S", &time::now()).unwrap());
    let vol = lazy!(component!(bg(BLUE) => [" \u{00E152} ", format!("{:.0} ", alsa::get_volume())]));
    let tree = component![
        component!(align_left => [bspwm]),
        component!(align_center => [date]),
        component!(align_right => [mpd, " ", power, " ", vol, " "])
    ];
    loop {
        tree.render(&mut bar);
        bar.flush();
        thread::sleep_ms(200);
    }
}
