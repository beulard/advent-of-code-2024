use core::fmt;
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum TileType {
    Empty,
    Wall,
}

#[derive(Eq, PartialEq, PartialOrd, Hash, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(Clone)]
struct Grid(Vec<Vec<TileType>>);
impl Grid {
    fn from_string(s: String) -> (Self, (i32, i32)) {
        let mut grid: Self = Grid(vec![]);
        let mut start_pos = (0, 0);
        for (j, line) in s.lines().enumerate() {
            grid.0.push(vec![]);
            let col = grid.0.last_mut().unwrap();
            for (i, c) in line.chars().enumerate() {
                col.push(match c {
                    '.' => TileType::Empty,
                    '#' => TileType::Wall,
                    '^' => {
                        start_pos = (i as i32, j as i32);
                        TileType::Empty
                    }
                    _ => panic!(),
                });
            }
        }
        (grid, start_pos)
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for line in &self.0 {
            for c in line {
                write!(
                    f,
                    "{}",
                    match c {
                        TileType::Empty => '.',
                        TileType::Wall => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn get_traversed(grid: &Grid, start_pos: (i32, i32)) -> Option<HashSet<(i32, i32)>> {
    let mut traversed: HashSet<(i32, i32)> = HashSet::new();
    let mut states: HashSet<((i32, i32), Direction)> = HashSet::new();

    let mut pos = start_pos;
    let mut direction = Direction::N;
    loop {
        traversed.insert(pos);
        if states.contains(&(pos, direction)) {
            // We have looped around
            return None;
        }
        states.insert((pos, direction));
        let next = match direction {
            Direction::N => (pos.0, pos.1 - 1),
            Direction::S => (pos.0, pos.1 + 1),
            Direction::W => (pos.0 - 1, pos.1),
            Direction::E => (pos.0 + 1, pos.1),
        };
        if next.1 as usize >= grid.0.len() || next.1 < 0 {
            break;
        }
        if next.0 as usize >= grid.0[next.1 as usize].len() || next.0 < 0 {
            break;
        }
        match grid.0[next.1 as usize][next.0 as usize] {
            TileType::Empty => {
                // Take a step
                pos = next;
            }
            TileType::Wall => {
                // Turn right
                direction = match direction {
                    Direction::N => Direction::E,
                    Direction::S => Direction::W,
                    Direction::W => Direction::N,
                    Direction::E => Direction::S,
                };
            }
        }
    }
    Some(traversed)
}

fn p1() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (grid, start_pos) = Grid::from_string(input);
    dbg!(&grid);

    get_traversed(&grid, start_pos).unwrap().len()
}

fn p2() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (grid, start_pos) = Grid::from_string(input);

    let traversed = get_traversed(&grid, start_pos).unwrap();

    let mut loops = 0;
    for (i, j) in traversed {
        // Determine if the path loops when we place an obstacle here
        let mut alt_grid = grid.clone();
        alt_grid.0[j as usize][i as usize] = TileType::Wall;

        if let None = get_traversed(&alt_grid, start_pos) {
            loops += 1;
        }
    }

    loops
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
