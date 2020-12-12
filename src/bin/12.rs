struct State {
    x: i64,
    y: i64,
    bearing: i64,
}

impl State {
    fn new() -> Self {
        State { x: 0, y: 0, bearing: 90 }
    }
}

struct Instr {
    op: u8,
    arg: i64,
}

fn parse_instr(line: &str) -> Instr {
    Instr {
        op: line.bytes().nth(0).unwrap(),
        arg: line[1..].parse::<i64>().unwrap(),
    }
}

fn step(mut state: State, instr: Instr) -> State {
    let arg = instr.arg;
    match instr.op {
        b'N' => state.y += arg,
        b'S' => state.y -= arg,
        b'E' => state.x += arg,
        b'W' => state.x -= arg,
        b'L' => state.bearing -= arg,
        b'R' => state.bearing += arg,
        b'F' => match state.bearing.rem_euclid(360) {
            0 => state.y += arg,
            90 => state.x += arg,
            180 => state.y -= arg,
            270 => state.x -= arg,
            _ => panic!(),
        },
        _ => panic!(),
    }
    state
}

fn part1(input: &str) -> i64 {
    let state = input
        .lines()
        .map(parse_instr)
        .fold(State::new(), step);
    state.x.abs() + state.y.abs()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 25);
    assert_eq!(part1(&aoc::input()), 2228);
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
