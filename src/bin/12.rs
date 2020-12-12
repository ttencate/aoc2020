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

struct State {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl State {
    fn rotate_dir(&mut self, angle: i64) {
        let (dx, dy) = match angle.rem_euclid(360) {
            0 => (self.dx, self.dy),
            90 => (self.dy, -self.dx),
            180 => (-self.dx, -self.dy),
            270 => (-self.dy, self.dx),
            _ => panic!(),
        };
        self.dx = dx;
        self.dy = dy;
    }

    fn move_forward(&mut self, steps: i64) {
        self.x += self.dx * steps;
        self.y += self.dy * steps;
    }

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn part1(input: &str) -> i64 {
    let mut state = State { x: 0, y: 0, dx: 1, dy: 0 };
    for Instr { op, arg } in input.lines().map(parse_instr) {
        match op {
            b'N' => state.y += arg,
            b'S' => state.y -= arg,
            b'E' => state.x += arg,
            b'W' => state.x -= arg,
            b'L' => state.rotate_dir(-arg),
            b'R' => state.rotate_dir(arg),
            b'F' => state.move_forward(arg),
            _ => panic!(),
        };
    }
    state.manhattan_distance()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 25);
    assert_eq!(part1(&aoc::input()), 2228);
}

fn part2(input: &str) -> i64 {
    let mut state = State { x: 0, y: 0, dx: 10, dy: 1 };
    for Instr { op, arg } in input.lines().map(parse_instr) {
        match op {
            b'N' => state.dy += arg,
            b'S' => state.dy -= arg,
            b'E' => state.dx += arg,
            b'W' => state.dx -= arg,
            b'L' => state.rotate_dir(-arg),
            b'R' => state.rotate_dir(arg),
            b'F' => state.move_forward(arg),
            _ => panic!(),
        };
    }
    state.manhattan_distance()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 286);
    assert_eq!(part2(&aoc::input()), 42908);
}

fn main() {
    aoc::main(part1, part2);
}
