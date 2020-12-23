use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cup(usize);

impl Cup {
    fn from_label(label: usize) -> Cup {
        Cup(label - 1)
    }

    fn from_index(index: usize) -> Cup {
        Cup(index)
    }

    fn from_digit(digit: char) -> Cup {
        Self::from_label(digit.to_digit(10).unwrap() as usize)
    }

    fn index(self) -> usize {
        self.0
    }
    
    fn label(self) -> usize {
        self.0 + 1
    }

    fn minus_one(self, num_cups: usize) -> Cup {
        Cup((self.0 + num_cups - 1) % num_cups)
    }
}

impl std::fmt::Display for Cup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

struct Node {
    prev: Cup,
    next: Cup,
}

struct Cups {
    nodes: Vec<Node>,
}

impl Cups {
    fn new(cups: &[Cup]) -> Cups {
        let num_cups = cups.len();
        let mut nodes = (0..num_cups)
            .map(|idx| Node { prev: Cup::from_index(idx), next: Cup::from_index(idx) })
            .collect::<Vec<_>>();
        for i in 0..(num_cups - 1) {
            nodes[cups[i + 1].index()].prev = cups[i];
            nodes[cups[i].index()].next = cups[i + 1];
        }
        nodes[cups[0].index()].prev = cups[num_cups - 1];
        nodes[cups[num_cups - 1].index()].next = cups[0];
        Cups { nodes }
    }

    fn num_cups(&self) -> usize {
        self.nodes.len()
    }

    fn next(&self, cup: Cup) -> Cup {
        self.nodes[cup.index()].next
    }

    fn link(&mut self, a: Cup, b: Cup) {
        self.nodes[a.index()].next = b;
        self.nodes[b.index()].prev = a;
    }

    fn iter_from<'a>(&'a self, from: Cup) -> CupsIterator<'a> {
        CupsIterator { cups: self, start: from, next: Some(from) }
    }
}

struct CupsIterator<'a> {
    cups: &'a Cups,
    start: Cup,
    next: Option<Cup>,
}

impl Iterator for CupsIterator<'_> {
    type Item = Cup;
    fn next(&mut self) -> Option<Cup> {
        let next = self.next;
        if let Some(next) = next {
            let new_next = self.cups.next(next);
            self.next = if new_next != self.start { Some(new_next) } else { None };
        }
        next
    }
}

fn play(cups: &mut Cups, mut curr: Cup, num_rounds: usize) {
    let num_cups = cups.num_cups();
    for _ in 0..num_rounds {
        let a = cups.next(curr);
        let b = cups.next(a);
        let c = cups.next(b);
        cups.link(curr, cups.next(c));
        let mut dest = curr.minus_one(num_cups);
        while dest == a || dest == b || dest == c {
            dest = dest.minus_one(num_cups);
        }
        let dest_next = cups.next(dest);
        cups.link(dest, a);
        cups.link(c, dest_next);
        curr = cups.next(curr);
    }
}

fn part1(input: &str) -> String {
    let init_cups = input.trim().chars().map(Cup::from_digit).collect::<Vec<_>>();

    let mut cups = Cups::new(&init_cups);
    play(&mut cups, init_cups[0], 100);

    cups.iter_from(Cup::from_label(1)).skip(1).join("")
}

#[test]
fn test_part1() {
    assert_eq!(part1("389125467\n"), "67384529");
    assert_eq!(part1(&aoc::input()), "46978532");
}

fn part2(input: &str) -> u64 {
    let mut init_cups = input.trim().chars().map(Cup::from_digit).collect::<Vec<_>>();
    init_cups.extend((init_cups.len()..1_000_000).map(Cup::from_index));

    let mut cups = Cups::new(&init_cups);
    play(&mut cups, init_cups[0], 10_000_000);
    
    cups.iter_from(Cup::from_label(1)).skip(1).take(2).map(|cup| cup.label() as u64).product()
}

#[test]
fn test_part2() {
    assert_eq!(part2("389125467\n"), 149245887792);
    // assert_eq!(part2(&aoc::input()), );
}

fn main() {
    aoc::main(part1, part2);
}
