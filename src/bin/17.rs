use std::collections::{HashMap, HashSet};

type State = HashSet<(i64, i64, i64)>;

fn parse(input: &str) -> State {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .bytes()
                .enumerate()
                .filter(|&(_x, c)| c == b'#')
                .map(move |(x, _c)| (x as i64, y as i64, 0))
        })
        .collect()
}

fn step(state: State) -> State {
    let mut neigh_count = HashMap::with_capacity(2 * state.len());
    for (x, y, z) in state.iter() {
        for dz in -1..2 {
            for dy in -1..2 {
                for dx in -1..2 {
                    let p = (x + dx, y + dy, z + dz);
                    if dx == 0 && dy == 0 && dz == 0 {
                        neigh_count.entry(p).or_default();
                    } else {
                        *neigh_count.entry(p).or_default() += 1;
                    }
                }
            }
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

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

#[test]
fn test_part2() {
    // assert_eq!(part2(&aoc::example(0)), );
    // assert_eq!(part2(&aoc::input()), );
}

fn main() {
    aoc::main(part1, part2);
}
