use std::str::FromStr;

pub mod part1;

#[derive(Debug, PartialEq)]
struct Number {
    length: usize,
    value: u32,
}
impl Number {
    fn get(&self, index: usize) -> u8 {
        ((self.value >> index) & 1) as u8
    }
}
impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u32::from_str_radix(s, 2).map_err(|_| String::from("Invalid binary number"))?;

        Ok(Self {
            length: s.len(),
            value,
        })
    }
}

fn group_numbers(numbers: impl Iterator<Item = Number>) -> Vec<[usize; 2]> {
    numbers.fold(Vec::<[usize; 2]>::new(), |mut acc, x: Number| {
        acc.resize_with(x.length, Default::default);
        (0..x.length).for_each(|i| {
            if let Some(arr) = acc.get_mut(i) {
                arr[x.get(i) as usize] += 1;
            }
        });
        acc
    })
}

fn calculate_gamma_rate_and_epsilon_rate(grouped_numbers: &[[usize; 2]]) -> (u32, u32) {
    grouped_numbers
        .iter()
        .rev()
        .fold((0u32, 0u32), |(gamma, epsilon), [zero_count, one_count]| {
            (
                gamma << 1 | if zero_count > one_count { 0 } else { 1 },
                epsilon << 1 | if zero_count < one_count { 0 } else { 1 },
            )
        })
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::utils;

    use super::*;

    #[test]
    fn it_should_parse_a_number() {
        assert_eq!(
            Ok(Number {
                length: 5,
                value: 0b00100
            }),
            "00100".parse()
        );
        assert_eq!(
            Ok(Number {
                length: 5,
                value: 0b11110
            }),
            "11110".parse()
        );
    }

    #[test]
    fn it_should_be_able_to_get_number() {
        assert_eq!(
            1,
            Number {
                length: 5,
                value: 0b10101
            }
            .get(0)
        );
        assert_eq!(
            0,
            Number {
                length: 5,
                value: 0b10101
            }
            .get(1)
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            group_numbers(utils::lines(Cursor::new(
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
            ))),
            vec![[7, 5], [5, 7], [4, 8], [7, 5], [5, 7]]
        );
    }
}
