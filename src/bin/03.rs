type Cell = u8;
type Grid = Vec<Vec<Cell>>;
const TREE: u8 = b'#';

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn count_trees(grid: &Grid, x_step: usize, y_step: usize) -> usize {
    let mut x = x_step;
    let mut y = y_step;
    let mut count = 0;
    while y < grid.len() {
        let line = &grid[y];
        if line[x % line.len()] == TREE {
            count += 1;
        }
        x += x_step;
        y += y_step;
    }
    count
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    count_trees(&grid, 3, 1)
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 7);
    assert_eq!(part1(&aoc::input()), 191);
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|&(x_step, y_step)| count_trees(&grid, x_step, y_step))
        .product()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 336);
    assert_eq!(part2(&aoc::input()), 1478615040);
}

fn main() {
    aoc::main(part1, part2);
}
