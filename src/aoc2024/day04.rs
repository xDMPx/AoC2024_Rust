pub fn part01(file_path: &str) -> usize {
    let word_search: Vec<Vec<char>> = std::fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut xmas_word_count = 0;
    for (y, line) in word_search.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                for (ax, ay) in get_adjacent_coords(&word_search, (x, y)) {
                    let dir = (ax as isize - x as isize, ay as isize - y as isize);
                    if word_search[ay][ax] != 'M' {
                        continue;
                    }
                    let d = get_valid_neighbor(&word_search, (ax, ay), dir);
                    if d == None {
                        continue;
                    }
                    let (ax, ay) = d.unwrap();
                    if word_search[ay][ax] != 'A' {
                        continue;
                    }
                    let d = get_valid_neighbor(&word_search, (ax, ay), dir);
                    if d == None {
                        continue;
                    }
                    let (ax, ay) = d.unwrap();
                    if word_search[ay][ax] == 'S' {
                        xmas_word_count += 1;
                    }
                }
            }
        }
    }

    xmas_word_count
}

fn get_adjacent_coords(
    word_search: &Vec<Vec<char>>,
    (x, y): (usize, usize),
) -> Vec<(usize, usize)> {
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut adjacent_coords = Vec::new();

    for (dx, dy) in &directions {
        if let Some((x, y)) = get_valid_neighbor(word_search, (x, y), (*dx, *dy)) {
            adjacent_coords.push((x, y));
        }
    }

    adjacent_coords
}

fn get_valid_neighbor(
    word_search: &Vec<Vec<char>>,
    (x, y): (usize, usize),
    (dx, dy): (isize, isize),
) -> Option<(usize, usize)> {
    let x = x as isize + dx;
    let y = y as isize + dy;

    if x >= 0 && y >= 0 {
        let x = x as usize;
        let y = y as usize;

        if x < word_search.len() && y < word_search[x].len() {
            return Some((x, y));
        }
    }

    return None;
}

pub fn part02(file_path: &str) -> usize {
    let word_search: Vec<Vec<char>> = std::fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut xmas_count = 0;
    for (y, line) in word_search.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'A' {
                let a = get_adjacent_coords_2(&word_search, (x, y));
                if a.len() != 4 {
                    continue;
                }
                if !((word_search[a[0].1][a[0].0] == 'S' && word_search[a[1].1][a[1].0] == 'M')
                    || (word_search[a[0].1][a[0].0] == 'M' && word_search[a[1].1][a[1].0] == 'S'))
                {
                    continue;
                }
                if (word_search[a[2].1][a[2].0] == 'S' && word_search[a[3].1][a[3].0] == 'M')
                    || (word_search[a[2].1][a[2].0] == 'M' && word_search[a[3].1][a[3].0] == 'S')
                {
                    xmas_count += 1;
                }
            }
        }
    }

    xmas_count
}

fn get_adjacent_coords_2(
    word_search: &Vec<Vec<char>>,
    (x, y): (usize, usize),
) -> Vec<(usize, usize)> {
    let directions = [(-1, -1), (1, 1), (1, -1), (-1, 1)];

    let mut adjacent_coords = Vec::new();

    for (dx, dy) in &directions {
        if let Some((x, y)) = get_valid_neighbor(word_search, (x, y), (*dx, *dy)) {
            adjacent_coords.push((x, y));
        }
    }

    adjacent_coords
}
