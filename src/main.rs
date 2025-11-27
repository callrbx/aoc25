#![feature(test)]

extern crate test;
mod day0;
mod util;

fn main() {
    println!("Advent of Code 2025");

    println!("day0: {:?}", day0::solve());
}
