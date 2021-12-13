use std::path;

#[test]
pub fn solve_a() {
    let input = super::func::input_generator("./day1/src/test.txt");
    let result = super::func::solve_part1(input);
    assert_eq!(result, 1000);
}

#[test]
pub fn solve_b() {
    let input = super::func::input_generator("./day1/src/test.txt");
    let result = super::func::solve_part2(input);
    assert_eq!(result, 1000);
}
