use crate::constants::HOURS_TO_MINUTES;
use crate::error::Error;
use regex::{Match, Regex};
use std::iter::Sum;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq)]
pub struct Duration {
    pub minutes: i32,
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(shift: &str) -> Result<Self, Self::Err> {
        // Will never error because this is a valid regex
        let shift_regex = Regex::new(r"(\d{1,2}):(\d{1,2})-(\d{1,2}):(\d{1,2})").unwrap();

        let shift_details = shift_regex.captures(&shift).ok_or_else(|| {
            Error(format!(
                "Expected shift: {} to be formated like 12:30-17:00",
                shift
            ))
        })?;

        let hours_start = parse_time(shift_details.get(1));
        let minutes_start = parse_time(shift_details.get(2));
        let mut hours_end = parse_time(shift_details.get(3));
        let minutes_end = parse_time(shift_details.get(4));

        if hours_start > hours_end {
            // This is safe because i8 from two numbers max is 99
            // 99 + 24 = 123 which is still below the bound of i32
            hours_end += 24;
        }

        let shift_minutes =
            (hours_end - hours_start) * HOURS_TO_MINUTES + (minutes_end - minutes_start);

        // We know for a fact that this is positive due to the
        // hours_start > hours_end check above
        Ok(Duration {
            minutes: shift_minutes,
        })
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, other: Self) {
        self.minutes += other.minutes;
    }
}

impl Sum for Duration {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::default(), |mut total, duration| {
            total += duration;
            total
        })
    }
}

// The usage of signed integers here might seem questionable but it's essential to
// the subtraction of minutes from each other where we want to persevere the sign
fn parse_time(time: Option<Match<'_>>) -> i32 {
    // The unwrap below is unreachable because we will return if the Regex
    // didn't capture the numbers
    time.map(|m| m.as_str().parse::<i32>().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_assign_adds_correctly() {
        let mut duration = Duration { minutes: -50 };
        duration += Duration { minutes: 550 };
        assert_eq!(duration, Duration { minutes: 500 });

        let mut duration = Duration { minutes: 123 };
        duration += Duration { minutes: 550 };
        assert_eq!(duration, Duration { minutes: 673 });
    }

    #[test]
    fn an_iterator_of_durations_can_be_summed() {
        let durations = vec![
            Duration { minutes: -50 },
            Duration { minutes: 550 },
            Duration { minutes: 123 },
        ];
        assert_eq!(
            durations.into_iter().sum::<Duration>(),
            Duration { minutes: 623 }
        );
    }

    #[test]
    fn it_can_turn_strings_into_durations() {
        assert_eq!("12:30-17:00".parse(), Ok(Duration { minutes: 270 }));
        assert_eq!("19:00-23:50".parse(), Ok(Duration { minutes: 290 }));
        assert_eq!("12:00-16:00".parse(), Ok(Duration { minutes: 240 }));
        assert_eq!("19:00-00:50".parse(), Ok(Duration { minutes: 350 }));
        assert_eq!("12:30-17:45".parse(), Ok(Duration { minutes: 315 }));
        assert_eq!("19:00-1:15".parse(), Ok(Duration { minutes: 375 }));

        assert_eq!(
            "19:00-:15".parse::<Duration>(),
            Err(Error(String::from(
                "Expected shift: 19:00-:15 to be formated like 12:30-17:00"
            )))
        );
    }
}
