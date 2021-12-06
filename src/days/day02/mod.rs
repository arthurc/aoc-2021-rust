use std::str::FromStr;

pub mod part1;
pub mod part2;

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

    #[test]
    fn it_should_be_able_to_parse() {
        assert_eq!(Ok(Command::Forward(2)), "forward 2".parse());
        assert_eq!(Ok(Command::Down(20)), "down 20".parse());
        assert_eq!(Ok(Command::Up(9)), "up 9".parse());
    }
}
