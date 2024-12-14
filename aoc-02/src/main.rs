fn is_safe(report: &[i32]) -> bool {
    let mut sign = 0;
    let mut safe = true;
    for pair in report.windows(2) {
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
    safe
}

fn p1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut score: i32 = 0;

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if is_safe(&values) {
            score += 1;
        }
    }

    return score;
}

fn p2() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut score: i32 = 0;

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // Determine if safe, if not, try again, removing a different number each time
        let mut safe = false;

        if is_safe(&values) {
            safe = true;
        } else {
            for i in 0..values.len() {
                let mut report = values.clone();
                report.remove(i);
                if is_safe(&report) {
                    safe = true;
                    break;
                }
            }
        }

        if safe {
            score += 1;
        }
    }

    return score;
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
