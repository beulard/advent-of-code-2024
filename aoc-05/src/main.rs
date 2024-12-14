use std::collections::HashMap;

fn p1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Construct list of dependencies for each page
    let mut deps = HashMap::<i32, Vec<i32>>::new();
    for (left, right) in rules
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
    {
        match deps.get_mut(&right) {
            Some(x) => {
                x.push(left);
            }
            None => {
                deps.insert(right, vec![left]);
            }
        }
    }

    let mut score = 0;
    for update in updates.lines() {
        let pages = update
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut updated: Vec<&i32> = Vec::with_capacity(pages.len());
        let mut ok = true;
        'update: for page in &pages {
            // update not ok if page depends on a page present in the update but not yet updated
            let depends = match deps.get(&page) {
                Some(x) => x.iter().filter(|&&p| pages.contains(&p)).collect(),
                None => vec![],
            };

            for d in &depends {
                if !updated.contains(d) {
                    ok = false;
                    break 'update;
                }
            }
            updated.push(page);
        }
        if ok {
            score += pages[pages.len() / 2];
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
