use crate::utils;
use std::io::BufRead;

pub fn run(read: impl BufRead) -> String {
    format!(
        "{}",
        utils::lines(read)
            .directions::<1>()
            .filter(Direction::increased)
            .count()
    )
}

#[derive(PartialEq, Debug)]
struct Direction {
    from: Option<i32>,
    to: i32,
}
impl Direction {
    pub fn increased(&self) -> bool {
        if let Some(from) = self.from {
            self.to > from
        } else {
            false
        }
    }
}

struct Directions<I, const W: usize> {
    iter: I,
    from: Option<i32>,
    window: [i32; W],
}
impl<I: Iterator<Item = i32>, const W: usize> Directions<I, W> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            from: None,
            window: [0; W],
        }
    }
}
impl<I, const W: usize> Iterator for Directions<I, W>
where
    I: Iterator<Item = i32>,
{
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let to = self.iter.next()?;
        let from = self.from.replace(to);

        Some(Direction { from, to })
    }
}

trait IteratorExt {
    fn directions<const W: usize>(self) -> Directions<Self, W>
    where
        Self: Sized + Iterator<Item = i32>,
    {
        Directions::new(self)
    }
}
impl<T: ?Sized> IteratorExt for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_should_be_able_to_handle_a_single_entry() {
        let mut iter = utils::lines(Cursor::new("123")).directions::<1>();
        assert_eq!(
            (
                Some(Direction {
                    from: None,
                    to: 123
                }),
                None
            ),
            (iter.next(), iter.next())
        );
    }

    #[test]
    fn it_should_be_able_to_handle_multiple_entries() {
        let mut iter = utils::lines(Cursor::new("123\n456")).directions::<1>();
        assert_eq!(
            (
                Some(Direction {
                    from: None,
                    to: 123
                }),
                Some(Direction {
                    from: Some(123),
                    to: 456
                }),
                None
            ),
            (iter.next(), iter.next(), iter.next())
        );
    }

    #[test]
    fn it_should_be_able_to_handle_no_entries() {
        let mut iter = utils::lines(Cursor::new("")).directions::<1>();
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_example() {
        assert_eq!(
            utils::lines(Cursor::new(
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
            .directions::<1>()
            .filter(Direction::increased)
            .count(),
            7
        )
    }

    #[test]
    fn x() {
        let mut iter = utils::lines(Cursor::new(
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
",
        ))
        .directions::<3>();
        assert_eq!(
            Some(Direction {
                from: None,
                to: 607
            }),
            iter.next()
        );
    }
}
