use std::fs;

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
