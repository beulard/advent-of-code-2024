use std::collections::VecDeque;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Node {
    add: Option<Box<Node>>,
    mul: Option<Box<Node>>,
    value: usize,
}

fn populate(mut operands: VecDeque<usize>, lhs: usize, leaves: &mut Vec<Box<Node>>) -> Box<Node> {
    if let Some(rhs) = operands.pop_front() {
        let n = Box::new(Node {
            add: Some(populate(operands.clone(), lhs + rhs, leaves)),
            mul: Some(populate(operands, lhs * rhs, leaves)),
            value: lhs,
        });
        return n;
    } else {
        let leaf = Box::new(Node {
            add: None,
            mul: None,
            value: lhs,
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Node2 {
    add: Option<Box<Node2>>,
    mul: Option<Box<Node2>>,
    cat: Option<Box<Node2>>,
    value: usize,
}

fn populate2(
    mut operands: VecDeque<usize>,
    lhs: usize,
    leaves: &mut Vec<Box<Node2>>,
) -> Box<Node2> {
    if let Some(rhs) = operands.pop_front() {
        let n = Box::new(Node2 {
            add: Some(populate2(operands.clone(), lhs + rhs, leaves)),
            mul: Some(populate2(operands.clone(), lhs * rhs, leaves)),
            cat: {
                let catted = [lhs.to_string(), rhs.to_string()]
                    .concat()
                    .parse::<usize>()
                    .unwrap();
                Some(populate2(operands, catted, leaves))
            },
            value: lhs,
        });
        return n;
    } else {
        let leaf = Box::new(Node2 {
            add: None,
            mul: None,
            cat: None,
            value: lhs,
        });
        leaves.push(leaf.clone());
        return leaf;
    }
}

fn p2() -> usize {
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
        let _root = populate2(operands, first, &mut leaves);
        for leaf in leaves {
            if leaf.value == result {
                score += result;
                break;
            }
        }
    }
    score
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
