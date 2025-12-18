use anyhow::{Error, Result};
use std::cmp::{Ordering, max};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tap::Pipe;

pub fn main(path: impl AsRef<Path>) -> Result<usize> {
    let mut input = File::open(path)?.pipe(BufReader::new).lines();

    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for line in input.by_ref() {
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

    let mut merged = vec![ranges[0]];

    for range in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if range.0 <= last.1 {
            last.1 = max(last.1, range.1);
        } else {
            merged.push(range);
        }
    }

    let mut total = 0;

    for line in input {
        let num: usize = line?.parse()?;

        if merged
            .binary_search_by(|range| {
                if range.0 > num {
                    Ordering::Greater
                } else if range.1 < num {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .is_ok()
        {
            total += 1;
        }
    }

    Ok(total)
}
