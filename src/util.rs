use std::fs;

use regex::Regex;

pub fn extract_day(path: &str) -> u32 {
    let re = Regex::new(r"day(\d+)\.rs$").unwrap();
    re.captures(path)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().parse().unwrap())
        .unwrap_or(0)
}

pub fn get_input(day: u32) -> String {
    let input_path = format!("{}/inputs/day{}.txt", env!("CARGO_MANIFEST_DIR"), day);

    match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to get input for day {} - {}", day, e);
            std::process::exit(1);
        }
    }
}
