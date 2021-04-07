extern crate ncurses;
extern crate signal_hook;

use crate::monitor;

pub fn init() {
    
}

fn nicen_seconds(secs : u64) -> String {
    let hours : u64 = secs / 3600;
    let minutes : u64 = (secs % 3600) / 60;
    let seconds : u64 = (secs % 3600) % 60;

    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

fn to_percentage(value: f64) -> String {
    return format!("{:.1}%", (value * (100 as f64)));
}