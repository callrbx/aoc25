#![feature(test)]

extern crate test;
// mod day0;
mod day1;
mod util;

fn main() {
    println!("Advent of Code 2025");

    println!("day1: {:?}", day1::solve());
}
