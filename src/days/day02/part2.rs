use super::*;
use crate::utils;
use std::io::BufRead;

pub fn run(read: impl BufRead) -> String {
    let mut submarine = Submarine::new();
    submarine.execute_commands(utils::lines(read));

    format!("{}", submarine.depth * submarine.horizontal_position)
}

#[derive(Debug, Default, PartialEq)]
struct Submarine {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}
impl Submarine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn execute_commands(&mut self, commands: impl IntoIterator<Item = Command>) {
        for command in commands {
            match command {
                Command::Forward(n) => {
                    self.horizontal_position += n;
                    self.depth += self.aim * n;
                }
                Command::Down(n) => self.aim += n,
                Command::Up(n) => self.aim -= n,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

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
                depth: 60,
                aim: 10
            },
            submarine
        );
    }
}
