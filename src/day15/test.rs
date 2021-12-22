#[test]
fn day16_solve_example() {
    let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let result = super::func::solve_a(input);
    assert_eq!(result, ());
}

#[test]
fn day16_solve_example() {
    let result = super::func::solve_a(include_str!("test.txt"));
    assert_eq!(result, ());
}
