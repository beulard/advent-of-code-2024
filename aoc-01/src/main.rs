use std::iter::zip;
fn p1() -> i32 {
    
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut vec_left = vec![];
    let mut vec_right = vec![];
    input.lines().for_each(|line| {
        let (left, right) = line.split_once("   ").unwrap();
        vec_left.push(left.parse::<i32>().unwrap());
        vec_right.push(right.parse::<i32>().unwrap());
        
    });
    vec_left.sort();
    vec_right.sort();
    let mut distance = 0;
    for (left, right) in zip(vec_left, vec_right) {
        let diff = right - left;
        distance += diff.abs();
    }
    return distance;
}

fn main() {
    println!("p1 = {}", p1());
}
