use crate::utils;
use std::{io::BufRead, str::FromStr};

pub fn run(read: impl BufRead) -> String {
    let mut submarine = Submarine::new();
    submarine.execute_commands(utils::lines(read));

    format!("{}", submarine.depth * submarine.horizontal_position)
}

#[derive(Debug, Default, PartialEq)]
struct Submarine {
    horizontal_position: i32,
    depth: i32,
}
impl Submarine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn execute_commands(&mut self, commands: impl IntoIterator<Item = Command>) {
        for command in commands {
            match command {
                Command::Forward(n) => self.horizontal_position += n,
                Command::Down(n) => self.depth += n,
                Command::Up(n) => self.depth -= n,
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}
impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" ").collect::<Vec<_>>()[..] {
            ["forward", s] => Ok(Command::Forward(
                s.parse().map_err(|_| String::from("Invalid number"))?,
            )),
            ["down", s] => Ok(Command::Down(
                s.parse().map_err(|_| String::from("Invalid number"))?,
            )),
            ["up", s] => Ok(Command::Up(
                s.parse().map_err(|_| String::from("Invalid number"))?,
            )),
            _ => Err(String::from("Invalid command")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_should_be_able_to_parse() {
        assert_eq!(Ok(Command::Forward(2)), "forward 2".parse());
        assert_eq!(Ok(Command::Down(20)), "down 20".parse());
        assert_eq!(Ok(Command::Up(9)), "up 9".parse());
    }

    #[test]
    fn example() {
        let mut submarine = Submarine::new();
        submarine.execute_commands(utils::lines(Cursor::new(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2",
        )));

        assert_eq!(
            Submarine {
                horizontal_position: 15,
                depth: 10
            },
            submarine
        );
    }
}
