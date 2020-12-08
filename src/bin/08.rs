use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let parts = s.split(" ").collect::<Vec<_>>();
        assert_eq!(parts.len(), 2);
        let op = parts[0];
        let arg = parts[1].parse::<i64>().unwrap();
        use Instruction::*;
        Ok(match op {
            "acc" => Acc(arg),
            "jmp" => Jmp(arg),
            "nop" => Nop,
            _ => panic!(),
        })
    }
}

#[derive(Debug)]
struct Program(Vec<Instruction>);

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(Program(s.lines().map(Instruction::from_str).map(Result::unwrap).collect()))
    }
}

#[derive(Debug)]
struct Interpreter {
    program: Program,
    pc: i64,
    acc: i64,
}

impl Interpreter {
    fn new(program: Program) -> Self {
        Interpreter {
            program,
            pc: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        let instr = self.program.0[self.pc as usize];
        use Instruction::*;
        self.pc += match instr {
            Acc(arg) => {
                self.acc += arg;
                1
            },
            Jmp(arg) => {
                arg
            },
            Nop => {
                1
            },
        }
    }
}

fn part1(input: &str) -> i64 {
    let prog = input.parse::<Program>().unwrap();
    let mut ip = Interpreter::new(prog);
    let mut visited = HashSet::new();
    while !visited.contains(&ip.pc) {
        visited.insert(ip.pc);
        ip.step();
    }
    ip.acc
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 5);
    assert_eq!(part1(&aoc::input()), 1801);
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
