use chrono::{DateTime, Utc};
const DATE_FORMAT_STR: &'static str = "%a, %d %b %Y %H:%M:%S GMT";

fn main() {
    // counter
    let mut counter:u64 = 0;

    // current percentage value
    let mut last_percentage: i8 = 0;

    // print the current timestamp to console. This is the starting time.
    println!("Loop started at: {}", current_time());

    loop {
        // Increase value by 1. Needs to be volatile, so the compiler doesn't optimize it away
        unsafe { core::ptr::write_volatile(&mut counter, counter + 1) };

        // Display percentage
        let percentage: f64 = 100.0 / u64::MAX as f64 * counter as f64;
        if percentage.floor() > last_percentage as f64 {
            last_percentage = percentage.floor() as i8;
            println!("{}% reached",last_percentage)
        }

        // Break out of the loop if we reached the MAX value.
        if counter == u64::MAX {
            break;
        }
    }

    // print the current timestamp to console. This is the ending time.
    println!("Loop ended at: {}", current_time());
}

/// Uses the chrono crate to display the current date and time in RFC1123 format.
fn current_time() -> String {
    let current_time: DateTime<Utc> = Utc::now();
    let formatted_time = current_time.format(DATE_FORMAT_STR).to_string();
    formatted_time
}