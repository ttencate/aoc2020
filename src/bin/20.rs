use std::collections::HashMap;

type TileId = u64;

#[derive(Debug, PartialEq, Eq)]
struct Grid<T> {
    nx: i64,
    ny: i64,
    cells: Vec<T>,
}

impl<T: Copy> Grid<T> {
    fn new(nx: i64, ny: i64, value: T) -> Grid<T> {
        assert!(nx >= 0);
        assert!(ny >= 0);
        Grid {
            nx,
            ny,
            cells: vec![value; (nx * ny) as usize],
        }
    }

    fn at(&self, x: i64, y: i64) -> &T {
        assert!(0 <= x && x < self.nx && 0 <= y && y < self.ny);
        &self.cells[(y * self.nx + x) as usize]
    }

    fn at_mut(&mut self, x: i64, y: i64) -> &mut T {
        assert!(0 <= x && x < self.nx && 0 <= y && y < self.ny);
        &mut self.cells[(y * self.nx + x) as usize]
    }

    fn transformed(&self, t: Transformation) -> Grid<T> {
        let (m00, m01, m02, mut m10, mut m11, mut m12) = match t.rotation {
            0 => (1, 0, 0, 0, 1, 0),
            1 => (0, 1, 0, -1, 0, self.nx - 1),
            2 => (-1, 0, self.nx - 1, 0, -1, self.ny - 1),
            3 => (0, -1, self.ny - 1, 1, 0, 0),
            _ => panic!(),
        };
        if t.flipped {
            m10 = -m10;
            m11 = -m11;
            m12 = self.ny - 1 - m12;
        }
        let (nx, ny) = match t.rotation {
            0 | 2 => (self.nx, self.ny),
            1 | 3 => (self.ny, self.nx),
            _ => panic!(),
        };
        let mut cells = Vec::with_capacity((nx * ny) as usize);
        for y in 0..ny {
            for x in 0..nx {
                cells.push(*self.at(
                    m00 * x + m01 * y + m02,
                    m10 * x + m11 * y + m12,
                ));
            }
        }
        Grid { nx, ny, cells }
    }

    fn draw(&mut self, tile: &Grid<T>, self_x: i64, self_y: i64, tile_x: i64, tile_y: i64, w: i64, h: i64) {
        for y in 0..h {
            for x in 0..w {
                *self.at_mut(self_x + x, self_y + y) = *tile.at(tile_x + x, tile_y + y);
            }
        }
    }
}

type Tile = Grid<u8>;

impl Tile {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Tile {
        let mut lines = lines.peekable();
        let nx = lines.peek().unwrap().len();
        let cells = lines
            .flat_map(str::as_bytes)
            .cloned()
            .collect::<Vec<u8>>();
        let ny = cells.len() / nx;
        assert_eq!(cells.len(), nx * ny);
        Tile { nx: nx as i64, ny: ny as i64, cells }
    }

    fn left_edge(&self) -> u64 {
        (0..self.ny)
            .map(|y| if *self.at(0, y) == b'#' { 1 << y } else { 0 })
            .fold(0, |a, b| a | b)
    }

    fn right_edge(&self) -> u64 {
        (0..self.ny)
            .map(|y| if *self.at(self.nx - 1, y) == b'#' { 1 << y } else { 0 })
            .fold(0, |a, b| a | b)
    }

    fn top_edge(&self) -> u64 {
        (0..self.nx)
            .map(|x| if *self.at(x, 0) == b'#' { 1 << x } else { 0 })
            .fold(0, |a, b| a | b)
    }

    fn bottom_edge(&self) -> u64 {
        (0..self.nx)
            .map(|x| if *self.at(x, self.ny - 1) == b'#' { 1 << x } else { 0 })
            .fold(0, |a, b| a | b)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Transformation {
    rotation: usize,
    flipped: bool,
}

static ALL_TRANSFORMATIONS: [Transformation; 8] = [
    Transformation { rotation: 0, flipped: false },
    Transformation { rotation: 1, flipped: false },
    Transformation { rotation: 2, flipped: false },
    Transformation { rotation: 3, flipped: false },
    Transformation { rotation: 0, flipped: true },
    Transformation { rotation: 1, flipped: true },
    Transformation { rotation: 2, flipped: true },
    Transformation { rotation: 3, flipped: true },
];

impl Transformation {
    fn identity() -> Transformation {
        Transformation { rotation: 0, flipped: false }
    }

    fn all() -> &'static [Transformation] {
        &ALL_TRANSFORMATIONS
    }
}

fn parse(input: &str) -> Vec<(TileId, Tile)> {
    input.split("\n\n")
        .filter_map(|tile| {
            if tile.is_empty() {
                return None;
            }
            let mut lines = tile.lines();
            let id = lines.next().unwrap()
                .split(" ").nth(1).unwrap()
                .trim_end_matches(":").parse::<TileId>().unwrap();
            let tile = Tile::parse(lines);
            Some((id, tile))
        })
        .collect()
}

struct Solver {
    tile_ids: Vec<TileId>,
    transformed_tiles: HashMap<TileId, HashMap<Transformation, Tile>>,
}

struct SolveState {
    used_tile_ids: Vec<(TileId, bool)>,
    solution: Grid<(TileId, Transformation)>,
}

impl Solver {
    fn new(tiles: &[(TileId, Tile)]) -> Solver {
        let tile_ids = tiles
            .iter()
            .map(|(id, _)| *id)
            .collect();

        let transformed_tiles = tiles
            .iter()
            .map(|(id, tile)| {
                (*id, Transformation::all().iter().map(|&t| (t, tile.transformed(t))).collect())
            })
            .collect::<HashMap<TileId, HashMap<Transformation, Tile>>>();

        Solver { tile_ids, transformed_tiles }
    }

    fn solve(&self) -> Grid<(TileId, Transformation)> {
        let used_tile_ids = self.tile_ids
            .iter()
            .map(|&id| (id, false))
            .collect();

        let n = self.transformed_tiles.len();
        let nx = (n as f64).sqrt().floor() as usize;
        let ny = nx;
        assert_eq!(n, nx * ny);
        let solution = Grid::new(nx as i64, ny as i64, (0, Transformation::identity()));

        let mut state = SolveState { used_tile_ids, solution };

        if !self.solve_rec(&mut state, 0, 0) {
            panic!();
        }

        state.solution
    }

    fn solve_rec(&self, state: &mut SolveState, x: i64, y: i64) -> bool {
        if y >= state.solution.ny {
            return true;
        }
        for i in 0..state.used_tile_ids.len() {
            let (tile_id, used) = state.used_tile_ids[i];
            if used {
                continue;
            }
            for (&t, tile) in self.transformed_tiles.get(&tile_id).unwrap() {
                if y > 0 {
                    let tile_above = self.tile_at(state, x, y - 1);
                    if tile.top_edge() != tile_above.bottom_edge() {
                        continue;
                    }
                }
                if x > 0 {
                    let tile_left = self.tile_at(state, x - 1, y);
                    if tile.left_edge() != tile_left.right_edge() {
                        continue;
                    }
                }
                *state.solution.at_mut(x, y) = (tile_id, t);
                let (next_x, next_y) = if x + 1 == state.solution.nx {
                    (0, y + 1)
                } else {
                    (x + 1, y)
                };
                state.used_tile_ids[i].1 = true;
                if self.solve_rec(state, next_x, next_y) {
                    return true;
                }
                state.used_tile_ids[i].1 = false;
            }
        }
        false
    }

    fn tile_at(&self, state: &SolveState, x: i64, y: i64) -> &Tile {
        let (id, t) = state.solution.at(x, y);
        self.transformed_tiles.get(&id).unwrap().get(&t).unwrap()
    }
}

fn part1(input: &str) -> u64 {
    let tiles = parse(input);
    let solution = Solver::new(&tiles).solve();

    [(0, 0), (solution.nx - 1, 0), (0, solution.ny - 1), (solution.nx - 1, solution.ny - 1)]
        .iter()
        .map(|&(x, y)| solution.at(x, y).0)
        .product()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&aoc::example(0)), 20899048083289);
    assert_eq!(part1(&aoc::input()), 64802175715999);
}

fn part2(input: &str) -> usize {
    let tiles = parse(input);
    let solution = Solver::new(&tiles).solve();
    let tile_size = tiles[0].1.nx;
    let stride = tile_size - 2;
    let mut grid = Grid::new(solution.nx * stride, solution.ny * stride, b' ');
    for y in 0..solution.ny {
        for x in 0..solution.nx {
            let &(tile_id, t) = solution.at(x, y);
            let transformed_tile = tiles.iter().find(|(id, _)| *id == tile_id).unwrap().1.transformed(t);
            grid.draw(&transformed_tile, x * stride, y * stride, 1, 1, stride, stride);
        }
    }

    let (sea_monster_nx, sea_monster_ny) = (20, 3);
    let sea_monster = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   "
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .bytes()
                .enumerate()
                .filter_map(move |(x, c)| if c == b'#' { Some((x as i64, y as i64)) } else { None })
        })
        .collect::<Vec<(i64, i64)>>();

    Transformation::all()
        .iter()
        .map(|&t| {
            let mut transformed_grid = grid.transformed(t);
            for y in 0..(transformed_grid.ny - sea_monster_ny) {
                'next_x: for x in 0..(transformed_grid.nx - sea_monster_nx) {
                    for &(mx, my) in sea_monster.iter() {
                        if *transformed_grid.at(x + mx, y + my) != b'#' {
                            continue 'next_x;
                        }
                    }
                    for &(mx, my) in sea_monster.iter() {
                        *transformed_grid.at_mut(x + mx, y + my) = b'O';
                    }
                }
            }
            (0..transformed_grid.ny)
                .map(|y| (0..transformed_grid.nx).filter(|&x| *transformed_grid.at(x, y) == b'#').count())
                .sum()
        })
        .min()
        .unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&aoc::example(0)), 273);
    assert_eq!(part2(&aoc::input()), 2146);
}

fn main() {
    aoc::main(part1, part2);
}
