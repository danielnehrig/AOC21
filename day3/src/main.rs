use std::{fs, io::BufRead};

fn main() {
    let parsed_file: Vec<String> = fs::read("test.txt")
        .unwrap()
        .lines()
        .take_while(|e| -> bool { e.as_ref().unwrap().len() > 0 })
        .map(|e| -> String { e.unwrap() })
        .collect::<Vec<String>>();

    let bit_len: usize = parsed_file[0].len();
    let mut find_gamma_rate = String::new();
    (0..bit_len).for_each(|k| {
        let nums = parsed_file
            .clone()
            .into_iter()
            .map(|n| n.chars().nth(k).unwrap())
            .collect::<String>();
        let zero = nums.matches("0").count();
        let one = nums.matches("1").count();

        if zero > one {
            find_gamma_rate.extend(["1"]);
        } else {
            find_gamma_rate.extend(["0"]);
        }
    });
    let gamma_rate = u16::from_str_radix(find_gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate = !gamma_rate & ((1 << find_gamma_rate.len()) - 1);
    let result = i32::from(gamma_rate as i32 * epsilon_rate as i32);

    println!("Gamma Rate String: {}", find_gamma_rate);
    println!("Parsed Dec Gamma Rate: {}", gamma_rate);
    println!("Parsed Dec Epsilon Rate: {}", epsilon_rate);
    println!("Result {}", result);
}
