use chrono::{DateTime, Utc};
use rayon::{prelude::*, ThreadPoolBuilder};
use num_cpus;


const DATE_FORMAT_STR: &'static str = "%a, %d %b %Y %H:%M:%S GMT";

fn main() {
    let num_threads: usize = num_cpus::get();
    let pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();

    println!("Found {} CPU cores.", num_threads);

    println!("Loop started at: {}", current_time());
    let n = pool.install(|| count_to_u32());
    // alternatives:
    // let n = pool.install(|| count_to_u64());
    // let n = pool.install(|| count_to_u128());
    // let n = pool.install(|| count_to_u16());

    println!("Loop ended at: {}", current_time());
    println!("Result: {}", n);
}

fn count_to_u128() -> usize {
    let max_value = u128::MAX;
    let result = (0..max_value)
        .into_par_iter()
        .map(|_| {
            // Increase value by 1. Needs to be volatile, so the compiler doesn't optimize it away
            let mut counter: u128 = 0;
            unsafe { core::ptr::write_volatile(&mut counter, 1) };
            counter
        })
        .sum::<u128>();

    result.try_into().unwrap()
}


fn count_to_u64() -> usize {
    let max_value = u64::MAX;
    let result = (0..max_value)
        .into_par_iter()
        .map(|_| {
            // Increase value by 1. Needs to be volatile, so the compiler doesn't optimize it away
            let mut counter: u64 = 0;
            unsafe { core::ptr::write_volatile(&mut counter, 1) };
            counter
        })
        .sum::<u64>();

    result.try_into().unwrap()
}

fn count_to_u32() -> usize {
    let max_value = u32::MAX;
    let result = (0..max_value)
        .into_par_iter()
        .map(|_| {
            // Increase value by 1. Needs to be volatile, so the compiler doesn't optimize it away
            let mut counter: u32 = 0;
            unsafe { core::ptr::write_volatile(&mut counter, 1) };
            counter
        })
        .sum::<u32>();

    result.try_into().unwrap()
}

fn count_to_u16() -> usize {
    let max_value = u16::MAX;
    let result = (0..max_value)
        .into_par_iter()
        .map(|_| {
            // Increase value by 1. Needs to be volatile, so the compiler doesn't optimize it away
            let mut counter: u16 = 0;
            unsafe { core::ptr::write_volatile(&mut counter, 1) };
            counter
        })
        .sum::<u16>();

    result.try_into().unwrap()
}


/// Uses the chrono crate to display the current date and time in RFC1123 format.
fn current_time() -> String {
    let current_time: DateTime<Utc> = Utc::now();
    let formatted_time = current_time.format(DATE_FORMAT_STR).to_string();
    formatted_time
} 