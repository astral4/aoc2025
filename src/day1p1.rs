use anyhow::{Result, bail};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tap::Pipe;

pub fn main(path: impl AsRef<Path>) -> Result<usize> {
    let mut pos = 50;
    let mut count = 0;

    let input = File::open(path)?.pipe(BufReader::new);

    for line in input.lines() {
        let line = line?;

        if let Some(num) = line.strip_prefix('L') {
            pos -= num.parse::<isize>()?;
        } else if let Some(num) = line.strip_prefix('R') {
            pos += num.parse::<isize>()?;
        } else {
            bail!("unexpected rotation direction");
        }

        if pos % 100 == 0 {
            count += 1;
        }
    }

    Ok(count)
}
