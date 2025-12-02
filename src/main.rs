#![feature(test)]

use std::time::Duration;

struct Answer {
    p1: String,
    p2: String,
    t: Duration,
}

extern crate test;
// mod day0;
mod day1;
mod day2;
mod util;

fn main() {
    let days: &[fn() -> Answer] = &[day1::solve, day2::solve];

    let mut total = Duration::new(0, 0);

    println!("Advent of Code 2025");

    for (i, solve) in days.iter().enumerate() {
        let a = solve();
        println!(
            "Day {:02}: {:?}\n\tP1: {}\n\tP2: {}",
            i + 1,
            a.t,
            a.p1,
            a.p2
        );
        total += a.t;
    }

    println!("\nTotal: {:?}", total);
}
