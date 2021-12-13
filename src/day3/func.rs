use std::{
    fs,
    io::{BufRead, Result},
};

pub fn solve_a() -> Result<usize> {
    let parsed_file: Vec<String> = fs::read("./src/day3/test.txt")?
        .lines()
        .take_while(|e| !e.as_deref().unwrap_or("").is_empty())
        .collect::<Result<Vec<String>>>()?;

    let bit_len: usize = parsed_file[0].len();
    let find_gamma_rate: String = (0..bit_len)
        .map(|k| {
            let nums = parsed_file
                .iter()
                .map(|n| n.chars().nth(k).unwrap())
                .collect::<String>();
            let zero = nums.matches("0").count();
            let one = nums.matches("1").count();

            if zero > one {
                "1"
            } else {
                "0"
            }
        })
        .collect();
    let gamma_rate = u16::from_str_radix(find_gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate = !gamma_rate & ((1 << find_gamma_rate.len()) - 1);
    let result = gamma_rate as usize * epsilon_rate as usize;

    return Ok(result);
}
