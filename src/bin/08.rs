use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
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
            "nop" => Nop(arg),
            _ => panic!(),
        })
    }
}

impl Instruction {
    fn next_offset(self) -> i64 {
        use Instruction::*;
        match self {
            Jmp(arg) => arg,
            _ => 1,
        }
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

impl Index<i64> for Program {
    type Output = Instruction;
    fn index(&self, pc: i64) -> &Self::Output {
        &self.0[pc as usize]
    }
}

impl IndexMut<i64> for Program {
    fn index_mut(&mut self, pc: i64) -> &mut Self::Output {
        &mut self.0[pc as usize]
    }
}

impl Program {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn enumerate<'a>(&'a self) -> impl Iterator<Item = (i64, Instruction)> + 'a {
        self.0.iter().enumerate().map(|(pc, instr)| (pc as i64, *instr))
    }
}

#[derive(Debug)]
struct Interpreter {
    prog: Program,
    pc: i64,
    acc: i64,
}

impl Interpreter {
    fn new(prog: Program) -> Self {
        Interpreter {
            prog,
            pc: 0,
            acc: 0,
        }
    }

    fn curr_instr(&self) -> Instruction {
        self.prog[self.pc]
    }

    fn step(&mut self) {
        let instr = self.curr_instr();
        use Instruction::*;
        match instr {
            Acc(arg) => { self.acc += arg; },
            Jmp(_) => {},
            Nop(_) => {},
        };
        self.pc += instr.next_offset();
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

fn uncorrupt(instr: Instruction) -> Instruction {
    use Instruction::*;
    match instr {
        Jmp(arg) => Nop(arg),
        Nop(arg) => Jmp(arg),
        _ => instr,
    }
}

fn part2(input: &str) -> i64 {
    let prog = input.parse::<Program>().unwrap();
    let final_pc = prog.len() as i64;

    let mut come_from = HashMap::<i64, Vec<i64>>::new();
    for (pc, instr) in prog.enumerate() {
        let next_pc = pc + instr.next_offset();
        come_from.entry(next_pc).or_default().push(pc);
    }

    let mut leads_to_end = HashSet::new();
    let mut stack = vec![final_pc];
    while let Some(pc) = stack.pop() {
        if !leads_to_end.contains(&pc) {
            leads_to_end.insert(pc);
            if let Some(froms) = come_from.get(&pc) {
                for &from in froms {
                    stack.push(from);
                }
            }
        }
    }

    let mut ip = Interpreter::new(prog);
    let mut patched = false;
    while ip.pc != final_pc {
        if !patched {
            let uncorrupted_instr = uncorrupt(ip.curr_instr());
            if leads_to_end.contains(&(ip.pc + uncorrupted_instr.next_offset())) {
                ip.prog[ip.pc] = uncorrupted_instr;
                patched = true;
            }
        }
        ip.step();
    }
    ip.acc
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 8);
    assert_eq!(part2(&aoc::input()), 2060);
}

fn main() {
    aoc::main(part1, part2);
}
