use anyhow::{Error, Result, bail};
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tap::Pipe;

pub fn main(path: impl AsRef<Path>) -> Result<u32> {
    let mut total = 0;

    let input = File::open(path)?.pipe(BufReader::new);

    for line in input.lines() {
        let Ok(line) = line else {
            bail!("failed to read line");
        };

        let mut chars = line.chars().rev();

        if let Some(mut next_max) = chars.next()
            && let Some(mut first_max) = chars.next()
        {
            let mut max_from_right = max(first_max, next_max);

            for c in chars {
                if c >= first_max {
                    first_max = c;
                    next_max = max_from_right;
                }
                max_from_right = max(max_from_right, c);
            }

            total += parse_digit(first_max)? * 10 + parse_digit(next_max)?;
        } else {
            bail!("not enough batteries in bank");
        }
    }

    Ok(total)
}

fn parse_digit(c: char) -> Result<u32> {
    c.to_digit(10).ok_or_else(|| Error::msg("invalid digit"))
}
