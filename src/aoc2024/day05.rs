use std::u32;

#[derive(Debug)]
#[allow(non_snake_case)]
struct Rule {
    X: u32,
    Y: u32,
}

pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let puzzle_input = puzzle_input.lines();

    let rules: Vec<Rule> = puzzle_input
        .clone()
        .map_while(|line| {
            if line.contains('|') {
                let rule = line.split_once('|').unwrap();

                Some(Rule {
                    X: rule.0.parse().unwrap(),
                    Y: rule.1.parse().unwrap(),
                })
            } else {
                None
            }
        })
        .collect();

    let updates = puzzle_input.skip_while(|line| line.contains('|')).skip(1);
    let updates = updates.map(|line| {
        line.split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>()
    });

    let mut middle_sum = 0;

    for update in updates {
        let valid_update = update
            .iter()
            .map(|x| check_after_page_rules(&update, &rules, *x))
            .fold(true, |acc, x| acc && x);

        if valid_update {
            let len = update.iter().len();
            middle_sum += update[len / 2];
        }
    }

    middle_sum.try_into().unwrap()
}

fn check_after_page_rules(update: &[u32], rules: &[Rule], page: u32) -> bool {
    let after_pages: Vec<u32> = rules.iter().filter(|r| r.X == page).map(|r| r.Y).collect();
    let pos = update.iter().position(|&x| x == page).unwrap();
    let update_before_pages = &update[..pos];

    let mut valid = true;
    for page in update_before_pages {
        valid &= !after_pages.contains(page);
    }

    valid
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let puzzle_input = puzzle_input.lines();

    let rules: Vec<Rule> = puzzle_input
        .clone()
        .map_while(|line| {
            if line.contains('|') {
                let rule = line.split_once('|').unwrap();

                Some(Rule {
                    X: rule.0.parse().unwrap(),
                    Y: rule.1.parse().unwrap(),
                })
            } else {
                None
            }
        })
        .collect();

    let updates = puzzle_input.skip_while(|line| line.contains('|')).skip(1);
    let updates = updates.map(|line| {
        line.split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>()
    });

    let mut middle_sum = 0;

    for update in updates {
        let valid_update = is_valid_update(&update, &rules);
        if !valid_update {
            let update = order_updates(&update, &rules);
            let len = update.iter().len();
            middle_sum += update[len / 2];
        }
    }

    middle_sum.try_into().unwrap()
}

fn order_updates(update: &[u32], rules: &[Rule]) -> Vec<u32> {
    let mut i = 0;
    let mut update = update.to_vec();
    let mut tmp = Vec::with_capacity(update.len());
    while !is_valid_update(&update, rules) {
        let page = update[i];
        let after_pages: Vec<u32> = rules.iter().filter(|r| r.X == page).map(|r| r.Y).collect();
        let pos = update.iter().position(|&x| x == page).unwrap();
        let update_before_pages = update[..pos].to_vec();

        let mut valid = true;
        for (j, page) in update_before_pages.iter().enumerate() {
            valid &= !after_pages.contains(page);
            if !valid {
                let j_val = update[j];
                for (e, x) in update.iter().enumerate() {
                    if e == j {
                        continue;
                    }
                    if e == i {
                        tmp.push(*x);
                        continue;
                    }
                    tmp.push(*x);
                }
                tmp.push(j_val);
                for (i, x) in tmp.iter().enumerate() {
                    update[i] = *x;
                }
                tmp.clear();
                i -= 1;
            }
        }

        i += 1;
    }

    update
}

fn is_valid_update(update: &[u32], rules: &[Rule]) -> bool {
    update
        .iter()
        .map(|x| check_after_page_rules(&update, &rules, *x))
        .fold(true, |acc, x| acc && x)
}
