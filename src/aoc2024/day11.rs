pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let mut state = std::collections::HashMap::<usize, usize>::from_iter(
        puzzle_input
            .trim()
            .split(" ")
            .map(|s| (s.parse().unwrap(), 1)),
    );

    for _ in 0..25 {
        let mut new_state = std::collections::HashMap::new();
        let stones: Vec<usize> = state.keys().map(|x| *x).collect();
        for stone_num in stones {
            if stone_num == 0 {
                let old_count = state.insert(stone_num, 0).unwrap();
                new_state
                    .entry(1)
                    .and_modify(|count| *count += old_count)
                    .or_insert(old_count);
            } else if is_even_dig(stone_num) {
                let (l, r) = split_dig(stone_num);
                let old_count = state.insert(stone_num, 0).unwrap();
                new_state
                    .entry(l)
                    .and_modify(|count| *count += old_count)
                    .or_insert(old_count);
                new_state
                    .entry(r)
                    .and_modify(|count| *count += old_count)
                    .or_insert(old_count);
            } else {
                let old_count = state.insert(stone_num, 0).unwrap();
                new_state
                    .entry(stone_num * 2024)
                    .and_modify(|count| *count += old_count)
                    .or_insert(old_count);
            }
        }

        state = new_state;
    }

    state.iter().map(|(_, c)| c).sum()
}

fn split_dig(x: usize) -> (usize, usize) {
    let x = x.to_string();
    let (l, r) = x.split_at(x.len() / 2);

    (l.parse().unwrap(), r.parse().unwrap())
}

fn is_even_dig(x: usize) -> bool {
    let mut x = x;
    let mut i = 0;
    while x != 0 {
        x /= 10;
        i += 1;
    }

    (i % 2) == 0
}
