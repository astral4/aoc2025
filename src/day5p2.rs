use anyhow::{Error, Result};
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tap::Pipe;

pub fn main(path: impl AsRef<Path>) -> Result<usize> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for line in File::open(path)?.pipe(BufReader::new).lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let (start, end) = line
            .split_once('-')
            .ok_or_else(|| Error::msg("expected range"))?;

        ranges.push((start.parse()?, end.parse()?));
    }

    if ranges.is_empty() {
        return Ok(0);
    }

    ranges.sort_unstable_by_key(|range| range.0);

    let mut count = 0;
    let (mut curr_start, mut curr_end) = ranges[0];

    for (start, end) in ranges.into_iter().skip(1) {
        if start <= curr_end + 1 {
            curr_end = max(curr_end, end);
        } else {
            count += curr_end - curr_start + 1;
            (curr_start, curr_end) = (start, end);
        }
    }
    count += curr_end - curr_start + 1;

    Ok(count)
}
