# Overtime Calculator

<hr />

[![Build Status](https://dev.azure.com/mathspy257/overtime-calc/_apis/build/status/Mathspy.overtime-calc?branchName=master)](https://dev.azure.com/mathspy257/overtime-calc/_build/latest?definitionId=3&branchName=master)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/Mathspy/overtime-calc#license)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat)](https://github.com/Mathspy/overtime-calc/pulls)

This project is a gift to a friend to help them calculate their absurd overtimes.\
It could be useful to you if you need to do the same. It can also be useful for summing up a list of durations in general

## Usage

```sh
overtime-calc <FILE> --hours <HOURS>
```

- FILE: Should be a location to a file formatted according to the format and spec described below
- HOURS: Should be the agreed amount of time in the work contract (defaults to 12 for my friend)

### Overtime file format
The file should be a simple utf8 encoded text file that looks like this:
```
# 1/8
12:30-17:00
# 2/8
12:00-16:00 / 19:00-00:50 / 19:30-01:25

# 8/8
12:00-16:45 / 19:00-01:15
```
### Overtime file spec:
- `H?H:M?M-H?H-M?M` 24h encoded duration (`H?` and `M?` means both `05:08` and `5:8` are valid and mean same thing)
- Extra durations can be listed on same line but must be separated by `/`
- Comments are allowed and fully ignored by the calculator and can start with either `#` or `//`
- Empty lines mark the end of a contract period which means ABOVE_NEW_LINE minus HOURS (the command line argument) [Subtracting the regular contracted time from overall time to obtain overtime]
- All files end with an empty new line explicitly (so there is always at least a single -HOURS in the calculation)

## Note
The original intention was to have each line be a day with several shifts and each "contract period" to be a week but there is really nothing stopping a user from not using empty new lines except every 30 days or otherwise and having different contract periods.\
Neither is there anything stopping you from listing all your week's shifts in a single line (of course separated by `/`) if you only ever work one shift per day and want to use the fact you can have more than a "shift" on one line.\
Neither is there anything stopping you from doing `--hour 0` to calculate full work time or anything else without any subtractions

## License
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Overtime Calculator by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
