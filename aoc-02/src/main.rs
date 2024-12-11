fn p1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut score: i32 = 0;

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut sign = 0;
        let mut safe = true;
        for pair in values.windows(2) {
            let diff = pair[1] - pair[0];
            if diff.abs() < 1 || diff.abs() > 3 {
                safe = false;
                break;
            }
            if sign != 0 && diff.signum() != sign {
                safe = false;
                break;
            }
            sign = diff.signum();
        }
        if safe {
            score += 1;
        }
    }

    return score;
}

fn p2() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    0
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
