use anyhow::{Error, Result, bail};
use std::array;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tap::Pipe;

pub fn main(path: impl AsRef<Path>) -> Result<usize> {
    let mut total = 0;

    let input = File::open(path)?.pipe(BufReader::new);

    for line in input.lines() {
        if let Ok(line) = line {
            // For each digit, store sorted list of positions
            let mut digit_positions: [_; 10] = array::from_fn(|_| Vec::new());

            for (i, c) in line.char_indices() {
                digit_positions[parse_digit(c)? as usize].push(i);
            }

            // Pointers: track first position >= current left for each digit
            let mut ptrs = [0; 10];
            let mut left = 0;
            let mut pow = 11;

            'a: for j in 0..12 {
                let right = line.len() - 12 + j;

                for d in (0..10).rev() {
                    let positions = &digit_positions[d];

                    // Advance pointer past positions < left
                    while let Some(&pos) = positions.get(ptrs[d])
                        && pos < left
                    {
                        ptrs[d] += 1;
                    }

                    // Check if valid position exists
                    if let Some(&pos) = positions.get(ptrs[d])
                        && pos <= right
                    {
                        total += 10usize.pow(pow) * d;
                        pow -= 1;
                        left = pos + 1;
                        continue 'a;
                    }
                }

                bail!("not enough batteries in bank");
            }
        } else {
            bail!("failed to read line");
        }
    }

    Ok(total)
}

fn parse_digit(c: char) -> Result<u32> {
    c.to_digit(10).ok_or_else(|| Error::msg("invalid digit"))
}
