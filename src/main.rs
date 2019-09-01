#![deny(clippy::all)]

use clap::{App, Arg, ArgMatches};
use constants::HOURS_TO_MINUTES;
use duration::Duration;
use error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::iter;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod duration;

mod error {
    #[derive(Debug, PartialEq)]
    pub struct Error(pub String);
}

mod constants {
    pub const HOURS_TO_MINUTES: i32 = 60;
}

const WRITE_ERROR: &str = "To be able to write to stdout/err";

fn main() {
    let args = App::new("Overtime calculator")
        .version("0.2")
        .author("Mathspy T. <mathspy257@gmail.com>")
        .about("Helps you calculate your overtime")
        .arg(
            Arg::with_name("location")
                .value_name("FILE")
                .help("Points to utf8 text encoded file of overtime shifts")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("hours")
                .long("hours")
                .short("h")
                .value_name("HOURS")
                .help("Determines the contract's weekly time required to work")
                .default_value("10")
                .takes_value(true),
        )
        .get_matches();

    std::process::exit(match run_app(args) {
        Ok(duration) => {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);

            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
            write!(&mut stdout, "info: ").expect(WRITE_ERROR);

            let _ = stdout.reset();
            let total_minutes = duration.minutes;
            writeln!(
                &mut stdout,
                "Total overtime is: {}h{}m",
                total_minutes / 60,
                total_minutes % 60
            )
            .expect(WRITE_ERROR);
            0
        }
        Err(error) => {
            let mut stderr = StandardStream::stderr(ColorChoice::Always);

            let _ = stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
            write!(&mut stderr, "error: ").expect(WRITE_ERROR);

            let _ = stderr.reset();
            writeln!(&mut stderr, "{}", error.0).expect(WRITE_ERROR);
            1
        }
    })
}

fn run_app(args: ArgMatches) -> Result<Duration, Error> {
    // Required parameter can't fail
    let location = args.value_of("location").unwrap();

    // There's a default for this arg so it can't fail
    let hours_per_week = args.value_of("hours").unwrap();
    let minutes_per_week = hours_per_week.parse::<i32>().map_err(|_| {
        Error(format!(
            "--hours/-h argument must be a valid integer not: {}",
            hours_per_week
        ))
    })? * HOURS_TO_MINUTES;

    let buffer = BufReader::new(
        File::open(location)
            .map_err(|err| Error(format!("Couldn't access file {} due to {}", location, err)))?,
    );

    total_time_from_buffer(buffer, minutes_per_week)
}

fn shifts_from_line(line: io::Result<String>) -> Result<Vec<String>, Error> {
    let shift =
        line.map_err(|_| Error("Error occurred while trying to read a line from file".to_owned()))?;

    if shift.starts_with('#') || shift.starts_with("//") {
        return Ok(vec![]);
    }

    Ok(shift
        .split('/')
        .map(str::trim)
        .map(str::to_owned)
        .collect::<Vec<String>>())
}

fn total_time_from_buffer<R: Read>(
    buffer: BufReader<R>,
    minutes_per_week: i32,
) -> Result<Duration, Error> {
    buffer
        .lines()
        .flat_map(shifts_from_line)
        .flatten()
        .map(|shift| {
            if shift.is_empty() {
                Ok(Duration {
                    minutes: -minutes_per_week,
                })
            } else {
                shift.parse()
            }
        })
        // Last week's non-overtime time:
        .chain(iter::once(Ok(Duration {
            minutes: -minutes_per_week,
        })))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_a_single_day_appropriately() {
        let buffer = BufReader::new(&b"12:00-16:00 / 19:00-00:50 / 01:30-01:50"[..]);
        assert_eq!(
            total_time_from_buffer(buffer, 10),
            Ok(Duration { minutes: 600 })
        );
    }

    #[test]
    fn calculates_multiple_days_appropriately() {
        let buffer = BufReader::new(
            &b"12:00-16:00 / 19:00-00:50 / 01:30-01:50\n\
            12:30-17:00 / 19:00-23:50"[..],
        );
        assert_eq!(
            total_time_from_buffer(buffer, 10),
            Ok(Duration { minutes: 1160 })
        );
    }

    #[test]
    fn calculates_multiple_weeks_and_subtracts_weekly_time() {
        let buffer = BufReader::new(
            &b"12:00-16:00 / 19:00-00:50 / 01:30-01:50\n\
            12:30-17:00 / 19:00-23:50\n\
            \n\
            12:30-17:00 / 19:00-23:50\n\
            12:00-16:00 / 19:00-00:50 / 01:30-01:50"[..],
        );
        assert_eq!(
            total_time_from_buffer(buffer, 10),
            Ok(Duration { minutes: 2320 })
        );
    }
}
