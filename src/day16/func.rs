use std::fs;
use std::io::BufRead;

pub fn input_generator(file: &str) -> Vec<u8> {
    fs::read(file)
        .unwrap()
        .lines()
        .take_while(|n| !n.as_ref().unwrap().is_empty())
        .map(|n| n.unwrap())
        .collect::<String>().chars().map(|n| u8::from_str_radix(&n.to_string(), 16).unwrap()).collect::<Vec<u8>>()
}

pub fn solve_a() {
    let input = input_generator("./src/day16/test.txt");
    println!("{:b}", input[19]);
    ()
}
