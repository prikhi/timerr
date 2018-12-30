extern crate chrono;
extern crate nix;
extern crate notify_rust;

use chrono::prelude::{Local, TimeZone};
use chrono::Duration;
use nix::unistd::{fork, ForkResult};
use notify_rust::Notification;
use std::env;
use std::process::exit;
use std::thread::sleep;

fn print_usage() {
    println!(
        "timerr: Popup a notification at a specific time or after some minutes
timerr <time> <name>
    <time> can be an abosulte HH:MM time, or a +MM delta.

For example:

    timerr 20 \"Laundry is ready\"
    timerr 14:45 \"Head towards meeting...\"
    "
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let wait = parse_wait_time(&args[1]);
            let name = &args[2];
            let wait = match wait.to_std() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error converting durations: {}", e);
                    exit(1);
                }
            };
            match fork() {
                Ok(ForkResult::Parent { child: _ }) => (),
                Ok(ForkResult::Child) => {
                    sleep(wait);
                    Notification::new().summary(name).timeout(0).show().unwrap();
                }
                Err(e) => {
                    eprintln!("Could not fork background thread: {}", e);
                    exit(1);
                }
            }
        }
        _ => print_usage(),
    };
}

// Parse a `HH:MM` or `MM` string into a duration of minutes from the current time.
fn parse_wait_time(s: &str) -> Duration {
    match Local.datetime_from_str(s, "%H:%M") {
        Ok(t) => t - Local::now(),
        Err(_) => {
            let diff = match s.parse() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Could not parse a time or diff: {}", e);
                    exit(1);
                }
            };
            Duration::minutes(diff)
        }
    }
}
