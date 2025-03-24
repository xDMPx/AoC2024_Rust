pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let map = puzzle_input
        .lines()
        .map(|line| line.chars().enumerate())
        .enumerate();

    let mut antennas: std::collections::HashMap<char, Vec<(usize, usize)>> =
        std::collections::HashMap::new();

    let mut max_x = None;
    let mut max_y = None;
    for (y, line) in map {
        if max_y == None || max_y.unwrap() < y {
            max_y = Some(y);
        }
        for (x, c) in line {
            if max_x == None || max_x.unwrap() < x {
                max_x = Some(x);
            }
            if c == '.' {
                continue;
            }
            antennas
                .entry(c)
                .and_modify(|pos| pos.push((y, x)))
                .or_insert(vec![(y, x)]);
        }
    }

    let max_x = max_x.unwrap() as i32;
    let max_y = max_y.unwrap() as i32;

    let mut antinodes = std::collections::HashSet::new();
    for (_, pos) in antennas {
        for i in 0..pos.len() {
            let center = (pos[i].0 as i32, pos[i].1 as i32);
            for j in 0..pos.len() {
                if i == j {
                    continue;
                }
                let p = (pos[j].0 as i32, pos[j].1 as i32);
                let diff = (p.0 - center.0, p.1 - center.1);
                let antinode = (center.0 - diff.0, center.1 - diff.1);

                if antinode.0 < 0 || antinode.1 < 0 || antinode.0 > max_y || antinode.1 > max_x {
                    continue;
                }

                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len()
}
