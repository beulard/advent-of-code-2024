#[derive(Debug, Clone)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Barrel,
    Chara,
}

fn draw(pos: (i32, i32), grid: &Vec<Vec<Tile>>) {
    for j in 0..grid.len() {
        let line = &grid[j];
        for i in 0..line.len() {
            if (i as i32, j as i32) == pos {
                print!("@");
            } else {
                print!(
                    "{}",
                    match line[i] {
                        Tile::Empty => '.',
                        Tile::Barrel => 'O',
                        Tile::Wall => '#',
                        Tile::Chara => '@',
                    }
                );
            }
        }
        println!();
    }
}

fn step(pos: (i32, i32), dir: Dir, grid: &mut Vec<Vec<Tile>>) -> Result<(i32, i32), ()> {
    let me = grid[pos.1 as usize][pos.0 as usize];

    let dest = match dir {
        Dir::N => (pos.0, pos.1 - 1),
        Dir::S => (pos.0, pos.1 + 1),
        Dir::W => (pos.0 - 1, pos.1),
        Dir::E => (pos.0 + 1, pos.1),
    };
    // println!("{:?} at {} {} -> {} {}", me, pos.0, pos.1, dest.0, dest.1);

    let result = match grid[pos.1 as usize][pos.0 as usize] {
        Tile::Empty => Ok(pos),
        Tile::Barrel | Tile::Chara => {
            if let Ok(_) = step(dest, dir, grid) {
                grid[dest.1 as usize][dest.0 as usize] = me;
                grid[pos.1 as usize][pos.0 as usize] = Tile::Empty;
                Ok(dest)
            } else {
                Err(()) // move forbidden by something up ahead -> propagate
            }
        }
        Tile::Wall => Err(()), // move forbidden right here
    };
    result
}

fn p1() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (warehouse, instructions) = input.split_once("\n\n").unwrap();

    let mut grid: Vec<Vec<Tile>> = Vec::new();
    let mut start_pos: (i32, i32) = (0, 0);

    for (j, line) in warehouse.lines().enumerate() {
        grid.push(vec![]);
        for (i, c) in line.chars().enumerate() {
            let t = match c {
                '#' => Tile::Wall,
                '@' => {
                    start_pos = (i as i32, j as i32);
                    Tile::Chara
                }
                'O' => Tile::Barrel,
                '.' => Tile::Empty,
                _ => panic!(),
            };
            grid.last_mut().unwrap().push(t);
        }
    }

    let instructions: Vec<Dir> = instructions
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '^' => Dir::N,
            'v' => Dir::S,
            '<' => Dir::W,
            '>' => Dir::E,
            _ => panic!(),
        })
        .collect();

    let mut pos = start_pos;
    for instruction in instructions {
        // draw(pos, &grid);
        // println!("{:?}", instruction);
        // println!();

        if let Ok(next) = step(pos, instruction, &mut grid) {
            pos = next;
        };
    }
    draw(pos, &grid);

    let mut score = 0;
    for (j, line) in grid.iter().enumerate() {
        for (i, &c) in line.iter().enumerate() {
            if let Tile::Barrel = c {
                score += i + j * 100;
            }
        }
    }

    score
}

fn p2() -> i32 {
    let _input = std::fs::read_to_string("input.txt").unwrap();
    0
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
