use packed_simd::SimdVector;
use std::collections::{HashMap, HashSet};

type Coord = packed_simd::i32x4;

fn neighbors_3d() -> Vec<Coord> {
    let mut result = Vec::with_capacity(26);
    for dz in -1..2 {
        for dy in -1..2 {
            for dx in -1..2 {
                if dx != 0 || dy != 0 || dz != 0 {
                    result.push(Coord::new(dx, dy, dz, 0));
                }
            }
        }
    }
    result
}

fn neighbors_4d() -> Vec<Coord> {
    let mut result = Vec::with_capacity(80);
    for dw in -1..2 {
        for dz in -1..2 {
            for dy in -1..2 {
                for dx in -1..2 {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        result.push(Coord::new(dx, dy, dz, dw));
                    }
                }
            }
        }
    }
    result
}

type State = HashSet<Coord>;

fn parse(input: &str) -> State {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .bytes()
                .enumerate()
                .filter(|&(_x, c)| c == b'#')
                .map(move |(x, _c)| Coord::new(
                        x as <Coord as SimdVector>::Element,
                        y as <Coord as SimdVector>::Element,
                        0,
                        0))
        })
        .collect()
}

fn step(state: State, neighbors: &Vec<Coord>) -> State {
    let mut neigh_count = HashMap::with_capacity(2 * state.len());
    for p in state.iter().copied() {
        neigh_count.entry(p).or_default();
        for &d in neighbors {
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
    let mut state = parse(input);
    let neighbors = neighbors_3d();
    for _ in 0..6 {
        state = step(state, &neighbors);
    }
    state.len()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 112);
    assert_eq!(part1(&aoc::input()), 265);
}

fn part2(input: &str) -> usize {
    let mut state = parse(input);
    let neighbors = neighbors_4d();
    for _ in 0..6 {
        state = step(state, &neighbors);
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
