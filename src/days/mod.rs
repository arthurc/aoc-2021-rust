use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

mod day01;
mod day02;

const DAYS: [Day; 3] = [Day(1, 1), Day(1, 2), Day(2, 1)];

pub enum Input {
    Predefined,
    File(PathBuf),
    Stdin,
}

#[derive(Debug, PartialEq)]
pub struct Day(i32, i32);
impl Day {
    pub fn new(n: i32, p: i32) -> Option<Self> {
        return if (1..=25).contains(&n) && p > 0 {
            Some(Self(n, p))
        } else {
            None
        };
    }

    pub fn run(&self, read: &mut impl BufRead) -> String {
        match self {
            Day(1, 1) => day01::part1::run(read),
            Day(1, 2) => day01::part2::run(read),
            Day(2, 1) => day02::run(read),
            _ => panic!(),
        }
    }

    pub fn input_file(&self) -> PathBuf {
        format!("input/day{:0>2}.txt", self.0).into()
    }
}
impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Day(d, 1) => write!(f, "{}", d),
            Day(d, p) => write!(f, "{} part {}", d, p),
        }
    }
}

impl FromStr for Day {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("/").map(|s| s.parse::<i32>()).collect::<Vec<_>>()[..] {
            [Ok(d), Ok(p)] => Day::new(d, p).ok_or_else(|| String::from("Invalid day")),
            [Ok(d)] => Ok(Day(d, 1)),
            _ => Err(String::from("Invalid day format")),
        }
    }
}

pub fn run(day: Day, input: Input) {
    //let day = get_day(day);
    let answer = match input {
        Input::Stdin => day.run(&mut std::io::stdin().lock()),
        Input::Predefined => day.run(&mut BufReader::new(File::open(day.input_file()).unwrap())),
        Input::File(path) => day.run(&mut BufReader::new(File::open(path).unwrap())),
    };

    println!("Answer to day {}: {}", day, answer)
}

pub fn run_all() {
    for day in DAYS {
        run(day, Input::Predefined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_str {
        use super::*;

        #[test]
        fn it_should_parse_single_day() {
            assert_eq!(Ok(Day(1, 1)), "1".parse::<Day>())
        }

        #[test]
        fn it_should_parse_day_and_part() {
            assert_eq!(Ok(Day(1, 2)), "1/2".parse::<Day>())
        }
    }

    mod day_input_file {
        use super::*;

        #[test]
        fn it_should_return_the_file_path() {
            assert_eq!(PathBuf::from("input/day01.txt"), Day(1, 1).input_file())
        }

        #[test]
        fn it_should_be_able_to_handle_two_digit_day() {
            assert_eq!(PathBuf::from("input/day12.txt"), Day(12, 1).input_file())
        }
    }
}
