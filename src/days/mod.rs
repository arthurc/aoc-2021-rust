use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod day01;

const DAYS: [Day; 1] = [Day(1)];

pub enum Input {
    Predefined,
    File(PathBuf),
    Stdin,
}

pub struct Day(i32);
impl Day {
    pub fn new(n: i32) -> Option<Self> {
        return if (1..=25).contains(&n) {
            Some(Self(n))
        } else {
            None
        };
    }

    pub fn run(&self, read: &mut impl BufRead) -> String {
        match self.0 {
            1 => day01::run(read),
            _ => panic!(),
        }
    }

    pub fn input_file(&self) -> PathBuf {
        format!("input/day{:0>2}.txt", self.0).into()
    }
}
impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

    mod day_input_file {
        use super::*;

        #[test]
        fn it_should_return_the_file_path() {
            assert_eq!(PathBuf::from("input/day01.txt"), Day(1).input_file())
        }

        #[test]
        fn it_should_be_able_to_handle_two_digit_day() {
            assert_eq!(PathBuf::from("input/day12.txt"), Day(12).input_file())
        }
    }
}
