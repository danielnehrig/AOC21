#[test]
pub fn solve_a() {
    let input = super::func::input_generator("./src/day1/test.txt");
    let result = super::func::solve_part1(input);
    assert_eq!(result, 1722);
}

#[test]
pub fn solve_b() {
    let input = super::func::input_generator("./src/day1/test.txt");
    let result = super::func::solve_part2(input);
    assert_eq!(result, 1748);
}
