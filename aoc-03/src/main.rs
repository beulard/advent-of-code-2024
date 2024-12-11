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
    let mut input = std::fs::read_to_string("input.txt").unwrap();

    // Remove all text between a "don't()" and the following "do()"
    let mut cleaned = String::new();
    while !input.is_empty() {
        match input.find("don't()") {
            Some(idx) => {
                cleaned.push_str(input.drain(..idx + "don't()".len()).as_str());
                match input.find("do()") {
                    Some(idx) => input.drain(..idx + "do()".len()),
                    None => input.drain(..),
                };
            }
            None => cleaned.push_str(input.drain(..).as_str()),
        }
    }

    // Same algorithm as p1 on the cleaned input
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut score = 0;

    re.captures_iter(&cleaned)
        .map(|cap| cap.extract())
        .map(|(_, [left, right])| (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap()))
        .for_each(|(left, right)| {
            score += left * right;
        });
    score
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
