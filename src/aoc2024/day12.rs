pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let map: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let max_x = map[0].len();
    let max_y = map.len();

    let mut plants: std::collections::HashMap<char, Vec<(usize, usize)>> =
        std::collections::HashMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            plants
                .entry(*c)
                .and_modify(|p| p.push((y, x)))
                .or_insert(vec![(y, x)]);
        }
    }

    let mut sum = 0;
    for (_, plant_pos) in plants.iter() {
        for region in find_plant_regions(plant_pos.clone(), max_x, max_y) {
            sum += region.len() * calculate_perimeter(&region, max_x, max_y);
        }
    }

    sum
}

fn calculate_perimeter(region: &[(usize, usize)], max_x: usize, max_y: usize) -> usize {
    let mut perimeter = 0;
    for pos in region {
        let adjacent_coords = get_adjacent_coords(*pos, max_x, max_y);
        let mut i = 4;
        for apos in adjacent_coords {
            if region.contains(&apos) {
                i -= 1;
            }
        }
        perimeter += i;
    }

    perimeter
}

fn find_plant_regions(
    mut plant: Vec<(usize, usize)>,
    max_x: usize,
    max_y: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut regions = vec![];
    while plant.len() > 0 {
        let mut to_check = vec![plant[0]];
        let mut region = vec![];
        while let Some(pos) = to_check.pop() {
            let i = plant.iter().position(|x| *x == pos).unwrap();
            plant.swap_remove(i);
            region.push(pos);

            let adjacent_coords = get_adjacent_coords(pos, max_x, max_y);
            for apos in adjacent_coords {
                if plant.contains(&apos) && !to_check.contains(&apos) {
                    to_check.push(apos);
                }
            }
        }
        regions.push(region);
    }

    regions
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

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let map: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let max_x = map[0].len();
    let max_y = map.len();

    let mut plants: std::collections::HashMap<char, Vec<(usize, usize)>> =
        std::collections::HashMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            plants
                .entry(*c)
                .and_modify(|p| p.push((y, x)))
                .or_insert(vec![(y, x)]);
        }
    }

    let mut sum = 0;
    for (_, plant_pos) in plants.iter() {
        for region in find_plant_regions(plant_pos.clone(), max_x, max_y) {
            sum += region.len() * calculate_number_of_sides(&region, max_x, max_y);
        }
    }

    sum
}

fn calculate_number_of_sides(region: &[(usize, usize)], max_x: usize, max_y: usize) -> usize {
    let mut region = region.to_vec();
    region.sort_by_key(|(y, x)| y * 10 + x);

    let ys: std::collections::HashSet<usize> =
        std::collections::HashSet::from_iter(region.iter().map(|(y, _)| *y));

    let xs: std::collections::HashSet<usize> =
        std::collections::HashSet::from_iter(region.iter().map(|(_, x)| *x));

    let mut count = 0;
    //hu
    let mut lines = vec![];
    for y in ys.iter() {
        let points = region
            .iter()
            .filter(|(py, _)| py == y)
            .filter(|(py, px)| *py == 0 || !region.contains(&(py - 1, *px)))
            .collect::<Vec<&(usize, usize)>>();
        if points.len() == 0 {
            continue;
        }
        let mut line = vec![points[0]];
        for p in points.iter().skip(1) {
            if line.last().unwrap().1 + 1 == p.1 {
                line.push(p);
            } else {
                lines.push(line);
                line = vec![p];
            }
        }
        lines.push(line);
    }

    count += lines.len();

    //hd
    let mut lines = vec![];
    for y in ys {
        let points = region
            .iter()
            .filter(|(py, _)| *py == y)
            .filter(|(py, px)| *py == max_y - 1 || !region.contains(&(py + 1, *px)))
            .collect::<Vec<&(usize, usize)>>();
        if points.len() == 0 {
            continue;
        }
        let mut line = vec![points[0]];
        for p in points.iter().skip(1) {
            if line.last().unwrap().1 + 1 == p.1 {
                line.push(p);
            } else {
                lines.push(line);
                line = vec![p];
            }
        }
        lines.push(line);
    }

    count += lines.len();

    //vr
    let mut lines = vec![];
    for x in xs.iter() {
        let points = region
            .iter()
            .filter(|(_, px)| px == x)
            .filter(|(py, px)| *px == 0 || !region.contains(&(*py, px - 1)))
            .collect::<Vec<&(usize, usize)>>();
        if points.len() == 0 {
            continue;
        }
        let mut line = vec![points[0]];
        for p in points.iter().skip(1) {
            if line.last().unwrap().0 + 1 == p.0 {
                line.push(p);
            } else {
                lines.push(line);
                line = vec![p];
            }
        }
        lines.push(line);
    }

    count += lines.len();

    //vl
    let mut lines = vec![];
    for x in xs.iter() {
        let points = region
            .iter()
            .filter(|(_, px)| px == x)
            .filter(|(py, px)| *px == max_x - 1 || !region.contains(&(*py, px + 1)))
            .collect::<Vec<&(usize, usize)>>();
        if points.len() == 0 {
            continue;
        }
        let mut line = vec![points[0]];
        for p in points.iter().skip(1) {
            if line.last().unwrap().0 + 1 == p.0 {
                line.push(p);
            } else {
                lines.push(line);
                line = vec![p];
            }
        }
        lines.push(line);
    }

    count += lines.len();

    count
}
