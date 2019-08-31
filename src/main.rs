use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let buffer = BufReader::new(File::open(&args[1])?);

    // Will never error because this is a valid regex
    let shift_regex = Regex::new(r"(-)?(?:(\d+)h)?(?:(\d+)m)?").unwrap();

    let total_minutes = buffer
        .lines()
        .map(|shift| {
            let shift = shift.expect("each line should be accessible from the file");
            if shift.starts_with("#") || shift.starts_with("//") {
                return 0;
            }

            let shift_details = shift_regex
                .captures(&shift)
                .or_else(|| {
                    eprintln!("Expected shift: {} to be formated like 999h54m", shift);
                    None
                })
                // We already logged error and are fine with crashing
                .unwrap();

            // Get the hours from regex's capture groups
            let hours = shift_details
                .get(2)
                // Since we are capturing (\d+) it will always be a number
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .unwrap_or(0);
            // Get the minutes from regex's capture groups
            let minutes = shift_details
                .get(3)
                // Since we are capturing (\d+) it will always be a number
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .unwrap_or(0);

            let negative = if shift_details.get(1).is_some() {
                -1
            } else {
                1
            };

            negative * (hours * 60 + minutes)
        })
        .sum::<i64>();

    println!("{}h{}m", total_minutes / 60, total_minutes % 60);

    Ok(())
}
