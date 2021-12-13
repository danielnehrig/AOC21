use std::fs;
use std::io::BufRead;

pub fn input_generator(file: &str) -> Vec<usize> {
    fs::read(file)
        .unwrap()
        .lines()
        .take_while(|n| n.as_ref().unwrap().len() > 0)
        .map(|e| e.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
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
