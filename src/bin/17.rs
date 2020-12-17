use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;

trait Coord: Copy + std::hash::Hash + Eq + Sized + Add<Output = Self> {
    fn neighbors() -> Vec<Self>;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord3(i64, i64, i64);

impl Coord for Coord3 {
    fn neighbors() -> Vec<Self> {
        let mut result = Vec::with_capacity(26);
        for dz in -1..2 {
            for dy in -1..2 {
                for dx in -1..2 {
                    if dx != 0 || dy != 0 || dz != 0 {
                        result.push(Coord3(dx, dy, dz));
                    }
                }
            }
        }
        result
    }
}

impl Add for Coord3 {
    type Output = Coord3;
    fn add(self, other: Coord3) -> Coord3 {
        Coord3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord4(i64, i64, i64, i64);

impl Coord for Coord4 {
    fn neighbors() -> Vec<Self> {
        let mut result = Vec::with_capacity(80);
        for dw in -1..2 {
            for dz in -1..2 {
                for dy in -1..2 {
                    for dx in -1..2 {
                        if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                            result.push(Coord4(dx, dy, dz, dw));
                        }
                    }
                }
            }
        }
        result
    }
}

impl Add for Coord4 {
    type Output = Coord4;
    fn add(self, other: Coord4) -> Coord4 {
        Coord4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3)
    }
}

type State<C> = HashSet<C>;

fn parse<C: Coord>(input: &str, new_coord: impl Fn(i64, i64) -> C) -> State<C> {
    let new_coord_ref = &new_coord;
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .bytes()
                .enumerate()
                .filter(|&(_x, c)| c == b'#')
                .map(move |(x, _c)| new_coord_ref(x as i64, y as i64))
        })
        .collect()
}

fn step<C: Coord>(state: State<C>) -> State<C> {
    let neighbors = C::neighbors();

    let mut neigh_count = HashMap::with_capacity(2 * state.len());
    for p in state.iter().copied() {
        neigh_count.entry(p).or_default();
        for &d in neighbors.iter() {
            *neigh_count.entry(p + d).or_default() += 1;
        }
    }

    neigh_count
        .into_iter()
        .filter_map(|(p, active_neigh)| {
            match (state.contains(&p), active_neigh) {
                (true, 2) | (true, 3) => Some(p),
                (true, _) => None,
                (false, 3) => Some(p),
                (false, _) => None,
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let mut state = parse(input, |x, y| Coord3(x, y, 0));
    for _ in 0..6 {
        state = step(state);
    }
    state.len()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 112);
    assert_eq!(part1(&aoc::input()), 265);
}

fn part2(input: &str) -> usize {
    let mut state = parse(input, |x, y| Coord4(x, y, 0, 0));
    for _ in 0..6 {
        state = step(state);
    }
    state.len()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 848);
    assert_eq!(part2(&aoc::input()), 1936);
}

fn main() {
    aoc::main(part1, part2);
}
