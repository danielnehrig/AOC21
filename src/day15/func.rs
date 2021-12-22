use std::collections::{HashMap, VecDeque};
pub fn solve_a(input: &str) -> i32 {
    let lines = input.trim().split("\n");
    let (mut grid, mut best_route) = (HashMap::new(), HashMap::new());
    let (max_y, mut max_x) = (lines.clone().collect::<Vec<_>>().len() - 1, 0);

    lines.enumerate().for_each(|(y, k)| {
        k.chars().enumerate().for_each(|(x, e)| {
            grid.insert((y, x), e.to_string().parse::<i32>().unwrap());
            best_route.insert((y, x), i32::MAX);
            max_x = x;
        });
    });

    let mut traverse = VecDeque::new();
    traverse.push_back(((0, 0), 0));
    while let Some(((y, x), cost)) = traverse.pop_front() {
        if cost < best_route[&(y, x)] {
            best_route.insert((y, x), cost);
            for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (y, x) = ((y as isize) + dy, (x as isize) + dx); 
                if y >= 0 && x >= 0 && y <= max_y as isize && x <= max_x as isize {
                    traverse.push_back((
                        (y as usize, x as usize),
                        cost + grid[&(y as usize, x as usize)],
                    ));
                }
            }
        }
    }

    best_route[&(max_y, max_x)]
}
