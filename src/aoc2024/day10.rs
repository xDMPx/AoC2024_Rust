pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let topographic_map: Vec<Vec<usize>> = puzzle_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let zeros = topographic_map
        .iter()
        .enumerate()
        .map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (c, (y, x))))
        .flatten()
        .filter(|(c, _)| **c == 0);

    let max_x = topographic_map[0].len();
    let max_y = topographic_map.len();

    let mut sum = 0;

    for (_, (y, x)) in zeros {
        let mut to_check = vec![(y, x)];
        let mut i = 0;
        while to_check.len() > 0 {
            let mut next_to_check: std::collections::HashSet<(usize, usize)> =
                std::collections::HashSet::new();
            for (y, x) in to_check {
                let adjacent = get_adjacent_coords((y, x), max_x, max_y);
                let adjacent = adjacent
                    .iter()
                    .filter(|(y, x)| topographic_map[*y][*x] == i + 1)
                    .map(|(y, x)| (*y, *x));
                next_to_check.extend(adjacent);
            }
            if next_to_check.len() > 0 {
                i += 1;
            }
            if i == 9 {
                sum += next_to_check.len();
                break;
            }
            to_check = next_to_check.iter().map(|(y, x)| (*y, *x)).collect();
        }
    }

    sum
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let topographic_map: Vec<Vec<usize>> = puzzle_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let zeros = topographic_map
        .iter()
        .enumerate()
        .map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (c, (y, x))))
        .flatten()
        .filter(|(c, _)| **c == 0);

    let max_x = topographic_map[0].len();
    let max_y = topographic_map.len();

    let mut sum = 0;

    for (_, (y, x)) in zeros {
        let mut to_check = vec![(y, x)];
        let mut i = 0;
        while to_check.len() > 0 {
            let mut next_to_check: Vec<(usize, usize)> = vec![];
            for (y, x) in to_check {
                let adjacent = get_adjacent_coords((y, x), max_x, max_y);
                let mut adjacent = adjacent
                    .iter()
                    .filter(|(y, x)| topographic_map[*y][*x] == i + 1)
                    .map(|(y, x)| (*y, *x))
                    .collect();
                next_to_check.append(&mut adjacent);
            }
            if next_to_check.len() > 0 {
                i += 1;
            }
            if i == 9 {
                sum += next_to_check.len();
                break;
            }
            to_check = next_to_check;
        }
    }

    sum
}

fn get_adjacent_coords((y, x): (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let x: i32 = x.try_into().unwrap();
    let y: i32 = y.try_into().unwrap();
    let max_x: i32 = max_x.try_into().unwrap();
    let max_y: i32 = max_y.try_into().unwrap();
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    let mut adjacent_coords = Vec::new();
    adjacent_coords.reserve(4);

    for (dx, dy) in &directions {
        let x = x + dx;
        let y = y + dy;

        if x < 0 || y < 0 || x >= max_x || y >= max_y {
            continue;
        }
        adjacent_coords.push((y as usize, x as usize));
    }

    adjacent_coords
}
