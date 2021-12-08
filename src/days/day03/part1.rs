use super::*;
use crate::utils;
use std::io::BufRead;

pub fn run(read: impl BufRead) -> String {
    let grouped_numbers = group_numbers(utils::lines(read));
    let (gamma_rate, epsilon_rate) = calculate_gamma_rate_and_epsilon_rate(&grouped_numbers);

    format!("{}", epsilon_rate * gamma_rate)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            run(Cursor::new(
                "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010"
            )),
            "198"
        );
    }
}
