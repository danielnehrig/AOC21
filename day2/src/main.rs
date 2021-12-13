use std::io::{BufRead, Read};

#[derive(Debug)]
struct Sub {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Sub {
    fn new() -> Sub {
        Sub {
            aim: 0,
            depth: 0,
            horizontal: 0,
        }
    }
}

fn main() {
    let mut sub = Sub::new();
    let parsed_file = std::fs::read("test.txt")
        .unwrap()
        .lines()
        .take_while(|n| n.as_ref().unwrap().len() > 0)
        .map(|n| n.unwrap())
        .collect::<Vec<String>>();

    let formated_parse: _ = parsed_file
        .clone()
        .into_iter()
        .map(|s| -> Vec<_> { s.split_whitespace().map(str::to_string).collect::<Vec<_>>() })
        .collect::<Vec<Vec<String>>>();

    let calculate = formated_parse.into_iter().for_each(|n| {
        let val = n[1].parse::<i32>().unwrap();
        let name = &n[0];
        match name.as_ref() {
            "forward" => {
                sub.horizontal = sub.horizontal + val;
                if sub.aim != 0 {
                    sub.depth = (val * sub.aim) + sub.depth;
                }
            }
            "up" => {
                sub.aim = sub.aim - val;
            }
            "down" => {
                sub.aim = sub.aim + val;
            }
            _ => {}
        }
    });

    println!("{}", sub.horizontal * sub.depth);
    println!("{:?}", sub);
}
