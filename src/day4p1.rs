use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tap::Pipe;

pub fn main(path: impl AsRef<Path>) -> Result<usize> {
    let mut count = 0;

    let input = File::open(path)?
        .pipe(BufReader::new)
        .lines()
        .map(|line| Ok(line?.pipe_as_ref(str::chars).collect()))
        .collect::<Result<Vec<Vec<_>>>>()?;

    let width = input.first().map_or(0, Vec::len);
    let length = input.len();

    // Returns true if cell (i, j) is a paper roll and the cells to check contain <4 paper rolls.
    // Returns false otherwise.
    let check_cell = |i: usize, j: usize, others: &[(isize, isize)]| {
        input[i][j] == '@'
            && others
                .iter()
                .map(|(di, dj)| input[(i as isize + di) as usize][(j as isize + dj) as usize])
                .filter(|&c| c == '@')
                .count()
                < 4
    };

    // Center
    for i in 1..length - 1 {
        for j in 1..width - 1 {
            if check_cell(
                i,
                j,
                &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
            ) {
                count += 1;
            }
        }
    }

    // Edges
    for j in 1..width - 1 {
        if check_cell(0, j, &[(0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]) {
            count += 1;
        }
        if check_cell(length - 1, j, &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1)]) {
            count += 1;
        }
    }
    for i in 1..length - 1 {
        if check_cell(i, 0, &[(-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]) {
            count += 1;
        }
        if check_cell(i, width - 1, &[(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0)]) {
            count += 1;
        }
    }

    // Corners
    count += usize::from(input[0][0] == '@')
        + usize::from(input[0][width - 1] == '@')
        + usize::from(input[length - 1][0] == '@')
        + usize::from(input[length - 1][width - 1] == '@');

    Ok(count)
}
