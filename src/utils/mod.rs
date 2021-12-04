use std::{io::BufRead, str::FromStr};

pub fn lines<T: FromStr>(reader: impl BufRead) -> impl Iterator<Item = T> {
    reader
        .lines()
        .flat_map(|s| s.into_iter())
        .flat_map(|s| s.trim().parse::<T>().into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    mod lines {
        use super::*;

        #[test]
        fn it_should_convert_each_line() {
            assert_eq!(
                lines(Cursor::new("123\n456\n789\n")).collect::<Vec<i32>>(),
                vec![123, 456, 789]
            );
        }

        #[test]
        fn it_should_be_empty_if_no_values_can_be_read() {
            assert_eq!(lines(Cursor::new("\n")).collect::<Vec<i32>>(), vec![]);
        }

        #[test]
        fn it_should_stop_once_it_reaches_an_unparsable_value() {
            assert_eq!(
                lines(Cursor::new("123\nbbb\n")).collect::<Vec<i32>>(),
                vec![123]
            );
        }

        #[test]
        fn test_example() {
            assert_eq!(
                lines(Cursor::new(
                    "199
            200
            208
            210
            200
            207
            240
            269
            260
            263
      "
                ))
                .collect::<Vec<i32>>(),
                vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
            )
        }
    }
}
