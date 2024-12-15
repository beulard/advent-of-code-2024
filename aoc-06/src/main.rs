use core::fmt;
use std::collections::HashSet;

#[derive(Debug)]
enum TileType {
    Empty,
    Wall,
}

struct Grid(Vec<Vec<TileType>>);
impl Grid {
    fn from_string(s: String) -> (Self, (usize, usize)) {
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
                        start_pos = (i, j);
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

fn p1() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut passed: HashSet<(usize, usize)> = HashSet::new();
    let (grid, start_pos) = Grid::from_string(input);
    dbg!(&grid);

    let mut pos = start_pos;
    enum Direction {
        N,
        S,
        W,
        E,
    }
    let mut direction = Direction::N;
    loop {
        passed.insert(pos);
        let next = match direction {
            Direction::N => (pos.0, pos.1 - 1),
            Direction::S => (pos.0, pos.1 + 1),
            Direction::W => (pos.0 - 1, pos.1),
            Direction::E => (pos.0 + 1, pos.1),
        };
        if next.1 >= grid.0.len() {
            break;
        }
        if next.0 >= grid.0[next.1].len() {
            break;
        }
        match grid.0[next.1][next.0] {
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

    passed.len()
}

fn p2() -> i32 {
    let _input = std::fs::read_to_string("input.txt").unwrap();
    0
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
