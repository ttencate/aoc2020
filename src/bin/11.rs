#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl From<u8> for Cell {
    fn from(byte: u8) -> Cell {
        match byte {
            b'.' => Cell::Floor,
            b'L' => Cell::EmptySeat,
            b'#' => Cell::OccupiedSeat,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Board {
    nx: i64,
    ny: i64,
    cells: Vec<Vec<Cell>>,
}

impl std::str::FromStr for Board {
    type Err = ();
    fn from_str(input: &str) -> Result<Board, ()> {
        let cells = input
            .lines()
            .map(|line| line.bytes().map(Cell::from).collect())
            .collect::<Vec<Vec<Cell>>>();
        let nx = cells[0].len() as i64;
        let ny = cells.len() as i64;
        Ok(Board { nx, ny, cells })
    }
}

const DIRECTIONS: [(i64, i64); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

impl Board {
    fn contains(&self, x: i64, y: i64) -> bool {
        x >= 0 && y >= 0 && x < self.nx && y < self.ny
    }

    fn get(&self, x: i64, y: i64) -> Cell {
        if self.contains(x, y) {
            self.cells[y as usize][x as usize]
        } else {
            Cell::Floor
        }
    }

    fn step<F: Fn(i64, i64) -> Cell>(&self, f: F) -> Board {
        let new_cells = (0..self.ny)
            .map(|y| {
                (0..self.nx)
                    .map(|x| {
                        f(x, y)
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();
        Board { nx: self.nx, ny: self.ny, cells: new_cells }
    }

    fn count_neighbors(&self, x: i64, y: i64, cell: Cell) -> usize {
        DIRECTIONS.iter()
            .filter(|(dx, dy)| self.get(x + dx, y + dy) == cell)
            .count()
    }

    fn count_visible(&self, x: i64, y: i64, cell: Cell) -> usize {
        DIRECTIONS.iter()
            .filter(|&&(dx, dy)| self.visible(x, y, dx, dy) == cell)
            .count()
    }

    fn visible(&self, mut x: i64, mut y: i64, dx: i64, dy: i64) -> Cell {
        loop {
            x += dx;
            y += dy;
            if !self.contains(x, y) {
                return Cell::Floor;
            }
            let cur = self.get(x, y);
            if cur != Cell::Floor {
                return cur;
            }
        }
    }

    fn count(&self, cell: Cell) -> usize {
        self.cells
            .iter()
            .map(|row| row.iter().filter(|c| **c == cell).count())
            .sum()
    }
}

fn part1(input: &str) -> usize {
    let mut board = input.parse::<Board>().unwrap();
    loop {
        let next = board.step(|x, y| {
            let occ_neigh = board.count_neighbors(x, y, Cell::OccupiedSeat);
            let curr = board.get(x, y);
            match (curr, occ_neigh) {
                (Cell::EmptySeat, 0) => Cell::OccupiedSeat,
                (Cell::OccupiedSeat, n) if n >= 4 => Cell::EmptySeat,
                _ => curr
            }
        });
        if next == board {
            return board.count(Cell::OccupiedSeat);
        }
        board = next;
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 37);
    assert_eq!(part1(&aoc::input()), 2247);
}

fn part2(input: &str) -> usize {
    let mut board = input.parse::<Board>().unwrap();
    loop {
        let next = board.step(|x, y| {
            let vis_occ = board.count_visible(x, y, Cell::OccupiedSeat);
            let curr = board.get(x, y);
            match (curr, vis_occ) {
                (Cell::EmptySeat, 0) => Cell::OccupiedSeat,
                (Cell::OccupiedSeat, n) if n >= 5 => Cell::EmptySeat,
                _ => curr
            }
        });
        if next == board {
            return board.count(Cell::OccupiedSeat);
        }
        board = next;
    }

}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 26);
    assert_eq!(part2(&aoc::input()), 2011);
}

fn main() {
    aoc::main(part1, part2);
}
