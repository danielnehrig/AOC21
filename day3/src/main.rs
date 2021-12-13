use std::{fs, io::BufRead};

fn main() {
    let parsed_file: Vec<String> = fs::read("test.txt")
        .unwrap()
        .lines()
        .take_while(|e| -> bool { e.as_ref().unwrap().len() > 0 })
        .map(|e| -> String { e.unwrap() })
        .collect::<Vec<String>>();

    let mut find_gamma_rate = String::new();
    (0..5).for_each(|k| {
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
    let epsilon_rate = find_gamma_rate.clone().parse::<u8>().unwrap();
    //let result: u32 = (gamma_rate.clone().parse::<u8>().unwrap() * epsilon_rate).into();

    println!("Gamma Rate: {}", find_gamma_rate);
    println!("Epsilon Rate: {:b}", epsilon_rate);
    //println!("Result: {}", result);
}
