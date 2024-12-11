use regex::Regex;

fn p1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut score = 0;

    re.captures_iter(&input)
        .map(|cap| cap.extract())
        .map(|(_, [left, right])| (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()))
        .for_each(|(left, right)| {
            score += left * right;
        });
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
