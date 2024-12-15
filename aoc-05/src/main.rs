use std::collections::HashMap;

fn is_update_ok(update: &[i32], deps: &HashMap<i32, Vec<i32>>) -> bool {
    let mut ok = true;
    let mut updated: Vec<&i32> = Vec::with_capacity(update.len());
    'update: for page in update {
        // update not ok if page depends on a page present in the update but not yet updated
        let depends = match deps.get(&page) {
            Some(x) => x.iter().filter(|&&p| update.contains(&p)).collect(),
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
    ok
}

fn get_dependency_map(rules: &[(i32, i32)]) -> HashMap<i32, Vec<i32>> {
    let mut deps = HashMap::<i32, Vec<i32>>::new();
    for (left, right) in rules {
        match deps.get_mut(&right) {
            Some(x) => {
                x.push(*left);
            }
            None => {
                deps.insert(*right, vec![*left]);
            }
        }
    }
    deps
}

fn p1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Construct list of dependencies for each page
    let deps = get_dependency_map(
        &rules
            .lines()
            .map(|line| line.split_once("|").unwrap())
            .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
            .collect::<Vec<(i32, i32)>>(),
    );

    let mut score = 0;
    for update in updates.lines() {
        let pages = update
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if is_update_ok(&pages, &deps) {
            score += pages[pages.len() / 2];
        }
    }

    score
}

fn p2() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Construct list of dependencies for each page
    let deps = get_dependency_map(
        &rules
            .lines()
            .map(|line| line.split_once("|").unwrap())
            .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
            .collect::<Vec<(i32, i32)>>(),
    );

    let mut score = 0;
    for update in updates.lines() {
        let pages = update
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut ok = true;
        let mut corrected = pages.clone();

        let mut i = 0;
        'update: while i < pages.len() {
            // update not ok if page depends on a page present in the update but not yet updated
            let depends = match deps.get(&corrected[i]) {
                Some(x) => x.iter().filter(|&&p| pages.contains(&p)).collect(),
                None => vec![],
            };

            for d in &depends {
                if corrected.iter().position(|x| x == *d).unwrap() > i {
                    ok = false;
                    // Invalid update: this page depends on d
                    // Invert this page and the dependent one to fix
                    let src_pos = i;
                    let dest_pos = corrected.iter().position(|x| x == *d).unwrap();
                    let tmp = corrected[src_pos];
                    corrected[src_pos] = **d;
                    corrected[dest_pos] = tmp;
                    i = src_pos;
                    continue 'update;
                }
            }
            i += 1;
        }
        if !ok {
            score += corrected[corrected.len() / 2];
        }
    }

    score
}

fn main() {
    println!("p1 = {}", p1());
    println!("p2 = {}", p2());
}
