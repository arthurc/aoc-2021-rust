use super::*;
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
