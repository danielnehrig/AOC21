use std::io::BufRead;

#[derive(Debug)]
pub struct Sub {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32,
}

impl Sub {
    pub fn new() -> Sub {
        Sub {
            aim: 0,
            depth: 0,
            horizontal: 0,
        }
    }
}

pub fn solve_b() -> i32 {
    let mut sub = Sub::new();
    let parsed_file = std::fs::read("./src/day2/test.txt")
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

    return sub.horizontal * sub.depth;
}
