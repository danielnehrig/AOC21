use anyhow::{anyhow, Result};
use std::io::{self, BufRead};

pub fn input_generator(input: std::io::Stdin) -> Result<Vec<usize>> {
    input
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .parse::<i32>()
                .map_err(|e| anyhow!("failed to convert '{}': {:?}", line, e))
        })
        .collect()
}

pub fn solve_part1(input: Vec<usize>) -> usize {
    input.windows(2).map(|x| (x[0] < x[1]) as usize).sum()
}

pub fn solve_part2(input: Vec<usize>) -> usize {
    solve_part1(
        input
            .windows(3)
            .map(|x| x.iter().sum())
            .collect::<Vec<usize>>(),
    )
}

fn main() {
    let stdin = io::stdin();
    let input = input_generator(stdin).unwrap();
    let solve_1 = solve_part1(input);
    let solve_2 = solve_part2(input);
}
