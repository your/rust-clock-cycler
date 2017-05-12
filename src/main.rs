extern crate time;

use std::time::{Duration};
use std::thread::sleep;

/// CONFIG BEGIN

// Values higher than 60 would lead to potential losses of cycles in scope.
static SECONDS_CLOCK: u64 = 20;

// How often to match time scope?
static MINUTES_FREQUENCY: i32 = 15;
// Valid hours to run the check for.
static HOURS_RANGE: &'static [i32] = &[9, 10, 11, 12, 14, 15, 16, 17];

/// CONFIG END

fn main() {
    loop {
        check_time();
        sleep_loop();
    }
}

fn check_time() {
    let now = time::now();

    let hour   = now.tm_hour;
    let minute = now.tm_min;

    if in_scope(hour, minute) {
        println!("Time to run, it's {}:{}!", hour, minute);
        do_something();
    } else {
        println!("Nothing to do yet, cycling again.");
    }
}

fn do_something() {
    println!("OH MY OH MY OH MY...!!!!");
}

fn in_scope(hour: i32, minute: i32) -> bool {
    HOURS_RANGE.contains(&hour) && (minute % MINUTES_FREQUENCY == 0)
}

fn sleep_loop() {
    sleep(Duration::new(SECONDS_CLOCK, 0));
}
