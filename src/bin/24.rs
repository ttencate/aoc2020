use std::collections::{HashMap, HashSet};

type Coord = (i32, i32);

fn final_tile(line: &str) -> Coord {
    let (mut x, mut y) = (0, 0);
    let mut bytes = line.bytes();
    while let Some(c) = bytes.next() {
        let (dx, dy) = match c {
            b'e' => (1, 0),
            b'w' => (-1, 0),
            b'n' => match bytes.next().unwrap() {
                b'e' => (1, -1),
                b'w' => (0, -1),
                _ => panic!(),
            },
            b's' => match bytes.next().unwrap() {
                b'e' => (0, 1),
                b'w' => (-1, 1),
                _ => panic!(),
            },
            _ => panic!(),
        };
        x += dx;
        y += dy;
    }
    (x, y)
}

fn initial_black(input: &str) -> HashSet<Coord> {
    input
        .lines()
        .map(|line| final_tile(line))
        .fold(HashSet::new(), |mut black, tile| {
            if black.contains(&tile) {
                black.remove(&tile);
            } else {
                black.insert(tile);
            }
            black
        })
}

fn part1(input: &str) -> usize {
    initial_black(input).len()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 10);
    assert_eq!(part1(&aoc::input()), 322);
}

fn part2(input: &str) -> usize {
    let mut black = initial_black(input);
    for _ in 0..100 {
        let mut neigh_count = HashMap::<Coord, usize>::with_capacity(2 * black.len());
        for &(x, y) in &black {
            neigh_count.entry((x, y)).or_default();
            for (dx, dy) in [(1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1)].iter() {
                *neigh_count.entry((x + dx, y + dy)).or_default() += 1;
            }
        }
        black = neigh_count
            .iter()
            .filter_map(|(coord, &count)| {
                let is_black = if black.contains(coord) {
                    count == 1 || count == 2
                } else {
                    count == 2
                };
                if is_black { Some(*coord) } else { None }
            })
            .collect();
    }
    black.len()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 2208);
    assert_eq!(part2(&aoc::input()), 3831);
}

fn main() {
    aoc::main(part1, part2);
}
