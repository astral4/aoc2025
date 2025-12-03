use anyhow::{Result, bail};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tap::Pipe;

pub fn main(path: impl AsRef<Path>) -> Result<usize> {
    let mut pos: isize = 50;
    let mut count = 0;

    let input = File::open(path)?.pipe(BufReader::new);

    for line in input.lines() {
        if let Ok(line) = line {
            if let Some(num) = line.strip_prefix('L') {
                let change: isize = num.parse()?;
                let new_pos = pos - change;

                count += div_floor_100(pos - 1) - div_ceil_100(new_pos) + 1;

                pos = new_pos;
            } else if let Some(num) = line.strip_prefix('R') {
                let change: isize = num.parse()?;
                let new_pos = pos + change;

                count += div_floor_100(new_pos) - div_ceil_100(pos + 1) + 1;

                pos = new_pos;
            } else {
                bail!("unexpected rotation direction");
            }
        } else {
            bail!("failed to read line");
        }
    }

    Ok(count as usize)
}

// We have signed int div_ceil() at home
fn div_ceil_100(n: isize) -> isize {
    let offset = if n >= 0 && n % 100 != 0 { 99 } else { 0 };
    (n + offset) / 100
}

// We have signed int div_floor() at home
fn div_floor_100(n: isize) -> isize {
    let offset = if n < 0 && n % 100 != 0 { 1 } else { 0 };
    (n / 100) - offset
}
