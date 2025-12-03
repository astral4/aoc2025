use anyhow::{Error, Result};
use std::fs::read_to_string;
use std::path::Path;
use tap::Pipe;

// Lazy brute force
pub fn main(path: impl AsRef<Path>) -> Result<usize> {
    read_to_string(path)?
        .split(',')
        .map(|range| {
            range
                .split_once('-')
                .ok_or_else(|| Error::msg("no hyphen found"))
                .and_then(|(start, end)| Ok((start.parse()?, end.parse()?)))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flat_map(|(start, end)| start..=end)
        .filter(|num: &usize| {
            let s = format!("{num}{num}");
            s[1..s.len() - 1].contains(&num.to_string())
        })
        .sum::<usize>()
        .pipe(Ok)
}

// Lazy brute force; no error handling
pub fn main2(path: impl AsRef<Path>) -> usize {
    read_to_string(path)
        .unwrap()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .flat_map(|(start, end)| start.parse().unwrap()..=end.parse().unwrap())
        .filter(|num: &usize| {
            let s = format!("{num}{num}");
            s[1..s.len() - 1].contains(&num.to_string())
        })
        .sum()
}
