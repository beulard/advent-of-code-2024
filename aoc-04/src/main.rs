fn p1() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let chars: Vec<char> = input.lines().fold(vec![], |mut acc, line| {
        line.chars().for_each(|c| {
            acc.push(c);
        });
        acc
    });

    let width = input.lines().last().unwrap().len();

    // Turn into strings in all possible directions:
    // left to right, right to left, top to bottom, bottom to top
    // diagonally ne, se, sw, nw
    // total 4 straight and 4 diagonals

    let mut strings: Vec<String> = vec![];

    for line in input.lines() {
        strings.push(String::from(line));
        strings.push(line.chars().rev().collect());
    }

    for col in (0..width).map(|i| chars.iter().skip(i).step_by(width).collect::<String>()) {
        strings.push(col.clone());
        strings.push(col.chars().rev().collect());
    }

    // Diagonals
    for i in 0..width {
        let len: usize = i + 1;
        // println!("{} {}", i, len);
        let mut ne: String = String::new();
        let mut se = String::new();
        for j in 0..len {
            // SE bottom left triangle, starting at bottom left (0, L)
            let c = chars[j + (width - i + j - 1) * width];
            se.push(c);
            // NE top left triangle, starting at top right (0, 0)
            let c = chars[j + (len - j - 1) * width];
            ne.push(c);
        }
        // println!("{}", se);
        strings.push(ne.clone());
        strings.push(ne.chars().rev().collect());

        // println!("{}", se);
        strings.push(se.clone());
        strings.push(se.chars().rev().collect());
    }
    // println!("{}", strings.len());
    for i in 1..width {
        // println!("{} {}", i, len);
        let mut ne = String::new();
        let mut se = String::new();
        for j in i..width {
            // SE top right triangle, starting (1, 0)
            let c = chars[j + (j - i) * width];
            se.push(c);
            // NE bottom right triangle, starting from (1, L)
            let c = chars[j + (width + i - j - 1) * width];
            ne.push(c);
        }
        // println!("{}", se);
        strings.push(se.clone());
        strings.push(se.chars().rev().collect());

        // println!("{}", ne);
        strings.push(ne.clone());
        strings.push(ne.chars().rev().collect());
    }

    // println!("{}", strings.len());

    let mut score = 0;
    for s in strings {
        score += s.match_indices("XMAS").count();
    }
    return score;
}

fn p2() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let chars: Vec<char> = input.lines().fold(vec![], |mut acc, line| {
        line.chars().for_each(|c| {
            acc.push(c);
        });
        acc
    });

    let width = input.lines().last().unwrap().len();

    let a_indices: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == 'A')
        .map(|(i, _)| i)
        .filter(|&i| {
            i >= width && i % width >= 1 && i % width < width - 1 && i < width * (width - 1)
        })
        .collect();

    let mut xmas_score = 0;

    for idx in a_indices {
        let x = idx % width;
        let y = idx / width;

        let nw = (x - 1) + (y - 1) * width;
        let ne = (x + 1) + (y - 1) * width;
        let se = (x + 1) + (y + 1) * width;
        let sw = (x - 1) + (y + 1) * width;

        if (chars[nw] == 'M' && chars[se] == 'S') || (chars[nw] == 'S' && chars[se] == 'M') {
            if (chars[sw] == 'M' && chars[ne] == 'S') || (chars[sw] == 'S' && chars[ne] == 'M') {
                xmas_score += 1;
            }
        }
    }

    xmas_score
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
