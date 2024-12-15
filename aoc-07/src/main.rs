use std::collections::VecDeque;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Node {
    add: Option<Box<Node>>,
    mul: Option<Box<Node>>,
    value: usize,
}

fn populate(mut operands: VecDeque<usize>, value: usize, leaves: &mut Vec<Box<Node>>) -> Box<Node> {
    if let Some(next) = operands.pop_front() {
        let n = Box::new(Node {
            add: Some(populate(operands.clone(), next + value, leaves)),
            mul: Some(populate(operands, next * value, leaves)),
            value: value,
        });
        return n;
    } else {
        let leaf = Box::new(Node {
            add: None,
            mul: None,
            value: value,
        });
        leaves.push(leaf.clone());
        return leaf;
    }
}

fn p1() -> usize {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut score = 0;

    for line in input.lines() {
        let (result, operands) = line.split_once(": ").unwrap();
        let result = result.parse::<usize>().unwrap();
        let mut operands: VecDeque<usize> = operands
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        // Represent input as a tree: each node has two children:
        // - one representing the result of left + right
        // - the other representing the result of left * right
        let mut leaves = vec![];
        let first = operands.pop_front().unwrap();
        let _root = populate(operands, first, &mut leaves);
        for leaf in leaves {
            if leaf.value == result {
                score += result;
                break;
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
