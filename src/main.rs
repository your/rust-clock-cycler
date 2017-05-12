extern crate time;

use std::time::{Duration};
use std::thread::sleep;

/// CONFIG BEGIN

// Values lower than 1000 would lead to potential losses of cycles in scope.
static MILLISECONDS_CLOCK: u64     = 900;

// How often to match time scope?
static MINUTES_FREQUENCY: i32      = 15;
// Valid hours to run the check for.
static HOURS_RANGE: &'static [i32] = &[9, 10, 11, 12, 14, 15, 16, 17];

/// CONFIG END

static IDENTIFIER_HOUR: &'static str = "hour";
static IDENTIFIER_MIN:  &'static str = "min";

fn main() {
    loop {
        check_time();
        sleep_loop();
    }
}

fn check_time() {
    let now = time::now();

    let hour   = time_unit_to_integer(now, IDENTIFIER_HOUR);
    let minute = time_unit_to_integer(now, IDENTIFIER_MIN);

    if in_scope(hour, minute) {
        println!("Time to run, it's {}:{}!", hour, minute);
        ::do_something();
    } else {
        println!("Nothing to do yet, cycling again.");
    }
}

fn do_something() {
    println!("OH MY OH MY OH MY...!!!!");
}

fn in_scope(hour: i32, minute: i32) -> bool {
    if !HOURS_RANGE.contains(&hour) {
        return false;
    }

    if !(minute % MINUTES_FREQUENCY == 0) {
        return false;
    }

    return true;
}

fn sleep_loop() {
    sleep(Duration::from_millis(MILLISECONDS_CLOCK));
}

fn time_unit_to_integer(time: time::Tm, unit: &'static str) -> i32 {
    let format = match unit as &'static str {
        "hour" => "%H",
        "min" => "%M",
        _ => "" // panic!
    };

    return time::strftime(format, &time).unwrap().parse::<i32>().unwrap();
}
