use anyhow::{Error, Result};
use std::cmp::{max, min};
use std::fs::read_to_string;
use std::path::Path;

pub fn main(path: impl AsRef<Path>) -> Result<usize> {
    let mut total = 0;

    let input = read_to_string(path)?;
    let ranges = input
        .split(',')
        .map(|range| {
            range
                .split_once('-')
                .ok_or_else(|| Error::msg("no hyphen found"))
        })
        .collect::<Result<Vec<_>>>()?;

    for (start, end) in ranges {
        // Note: we use .len() instead of .chars().count() because we assume chars are 1 byte each
        let start_digits = start.len();
        let end_digits = end.len();

        let start_i = if start_digits % 2 == 0 {
            let front = start[..(start_digits / 2)].parse()?;
            let back = start[(start_digits / 2)..].parse()?;
            if front >= back { front } else { front + 1 }
        } else {
            10usize.pow((start_digits / 2) as u32)
        };

        let end_i = if end_digits % 2 == 0 {
            let front = end[..(end_digits / 2)].parse()?;
            let back = end[(end_digits / 2)..].parse()?;
            if front <= back { front } else { front - 1 }
        } else {
            10usize.pow((start_digits / 2 + 1) as u32) - 1
        };

        // Process each digit count separately
        for k in (start_digits / 2) as u32..=(end_digits / 2) as u32 {
            // Find the range of k-digit numbers within [start, end]
            let range_start = 10usize.pow(k - 1); // First k-digit number
            let range_end = 10usize.pow(k) - 1; // Last k-digit number

            let first = max(start_i, range_start);
            let last = min(end_i, range_end);

            if first <= last {
                // Sum of arithmetic sequence
                let sum = (last - first + 1) * (first + last) / 2;

                // Each number n becomes n * (10^k + 1) when concatenated
                let multiplier = 10usize.pow(k) + 1;
                total += sum * multiplier;
            }
        }
    }

    Ok(total)
}
