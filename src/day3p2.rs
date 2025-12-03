use anyhow::{Error, Result, bail};
use arrayvec::ArrayVec;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tap::Pipe;

pub fn main(path: impl AsRef<Path>) -> Result<usize> {
    let mut total = 0;

    let input = File::open(path)?.pipe(BufReader::new);

    for line in input.lines() {
        let Ok(line) = line else {
            bail!("failed to read line");
        };

        let mut stack = ArrayVec::<_, 12>::new();

        for (i, c) in line.char_indices() {
            while stack.last().is_some_and(|&top| top < c) && stack.len() + line.len() - i > 12 {
                stack.pop();
            }
            if stack.len() < 12 {
                stack.push(c);
            }
        }

        total += stack
            .into_iter()
            .try_fold(0, |acc, c| parse_digit(c).map(|d| 10 * acc + d))?;
    }

    Ok(total)
}

fn parse_digit(c: char) -> Result<usize> {
    c.to_digit(10)
        .map_or_else(|| Err(Error::msg("invalid digit")), |d| Ok(d as usize))
}
