#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let mut map: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let guard_pos = find_guard_pos(&map);

    traverse_map(guard_pos, &mut map);

    map.iter()
        .map(|line| line.iter())
        .flatten()
        .filter(|c| **c == 'X')
        .count()
}

fn traverse_map(guard_pos: (usize, usize), map: &mut Vec<Vec<char>>) {
    map[guard_pos.0][guard_pos.1] = 'X';

    let mut guard_pos = (guard_pos.0 as i32, guard_pos.1 as i32);
    let mut guard_direction = Direction::UP;

    let max_x = map[0].len() as i32;
    let max_y = map.len() as i32;

    loop {
        match guard_direction {
            Direction::UP => {
                guard_pos.0 -= 1;
                if guard_pos.0 < 0
                    || guard_pos.0 >= max_y
                    || guard_pos.1 < 0
                    || guard_pos.1 >= max_x
                {
                    break;
                }
                let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
                if map[gp.0][gp.1] == '#' {
                    guard_pos.0 += 1;
                    guard_direction = Direction::RIGHT;
                }
            }
            Direction::DOWN => {
                guard_pos.0 += 1;
                if guard_pos.0 < 0
                    || guard_pos.0 >= max_y
                    || guard_pos.1 < 0
                    || guard_pos.1 >= max_x
                {
                    break;
                }
                let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
                if map[gp.0][gp.1] == '#' {
                    guard_pos.0 -= 1;
                    guard_direction = Direction::LEFT;
                }
            }
            Direction::RIGHT => {
                guard_pos.1 += 1;
                if guard_pos.0 < 0
                    || guard_pos.0 >= max_y
                    || guard_pos.1 < 0
                    || guard_pos.1 >= max_x
                {
                    break;
                }
                let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
                if map[gp.0][gp.1] == '#' {
                    guard_pos.1 -= 1;
                    guard_direction = Direction::DOWN;
                }
            }
            Direction::LEFT => {
                guard_pos.1 -= 1;
                if guard_pos.0 < 0
                    || guard_pos.0 >= max_y
                    || guard_pos.1 < 0
                    || guard_pos.1 >= max_x
                {
                    break;
                }
                let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
                if map[gp.0][gp.1] == '#' {
                    guard_pos.1 += 1;
                    guard_direction = Direction::UP;
                }
            }
        }
        let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
        map[gp.0][gp.1] = 'X';
    }
}

fn find_guard_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    map.iter()
        .map(|line| line.iter().enumerate().filter(|(_x, char)| **char == '^'))
        .enumerate()
        .map(|(y, mut line)| (y, line.next()))
        .filter(|(_y, line)| line.is_some())
        .map(|(y, line)| (y, line.unwrap().0))
        .next()
        .unwrap()
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let map_org: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut map = map_org.clone();
    let guard_pos = find_guard_pos(&map);
    traverse_map(guard_pos, &mut map);
    let vistied: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_x, char)| **char == 'X')
                .map(move |(x, _char)| (y, x))
        })
        .flatten()
        .collect();

    let mut count = 0;

    let max_x = map[0].len() as i32;
    let max_y = map.len() as i32;
    let guard_pos_org = (guard_pos.0 as i32, guard_pos.1 as i32);

    for (y, x) in vistied {
        let mut visited_obstacles: std::collections::HashMap<(usize, usize), Direction> =
            std::collections::HashMap::new();
        map = map_org.clone();
        let mut path_loop = false;
        map[y][x] = '#';
        let mut guard_direction = Direction::UP;
        let mut guard_pos = (guard_pos_org.0 as i32, guard_pos_org.1 as i32);
        loop {
            match guard_direction {
                Direction::UP => {
                    guard_pos.0 -= 1;
                    if guard_pos.0 < 0
                        || guard_pos.0 >= max_y
                        || guard_pos.1 < 0
                        || guard_pos.1 >= max_x
                    {
                        break;
                    }
                    let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
                    if map[gp.0][gp.1] == '#' {
                        guard_pos.0 += 1;
                        guard_direction = Direction::RIGHT;
                        let last_diresction = visited_obstacles.insert(gp, guard_direction);
                        if last_diresction.is_some() && last_diresction.unwrap() == guard_direction
                        {
                            path_loop = true;
                            break;
                        }
                    }
                }
                Direction::DOWN => {
                    guard_pos.0 += 1;
                    if guard_pos.0 < 0
                        || guard_pos.0 >= max_y
                        || guard_pos.1 < 0
                        || guard_pos.1 >= max_x
                    {
                        break;
                    }
                    let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
                    if map[gp.0][gp.1] == '#' {
                        guard_pos.0 -= 1;
                        guard_direction = Direction::LEFT;
                        let last_diresction = visited_obstacles.insert(gp, guard_direction);
                        if last_diresction.is_some() && last_diresction.unwrap() == guard_direction
                        {
                            path_loop = true;
                            break;
                        }
                    }
                }
                Direction::RIGHT => {
                    guard_pos.1 += 1;
                    if guard_pos.0 < 0
                        || guard_pos.0 >= max_y
                        || guard_pos.1 < 0
                        || guard_pos.1 >= max_x
                    {
                        break;
                    }
                    let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
                    if map[gp.0][gp.1] == '#' {
                        guard_pos.1 -= 1;
                        guard_direction = Direction::DOWN;
                        let last_diresction = visited_obstacles.insert(gp, guard_direction);
                        if last_diresction.is_some() && last_diresction.unwrap() == guard_direction
                        {
                            path_loop = true;
                            break;
                        }
                    }
                }
                Direction::LEFT => {
                    guard_pos.1 -= 1;
                    if guard_pos.0 < 0
                        || guard_pos.0 >= max_y
                        || guard_pos.1 < 0
                        || guard_pos.1 >= max_x
                    {
                        break;
                    }
                    let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
                    if map[gp.0][gp.1] == '#' {
                        guard_pos.1 += 1;
                        guard_direction = Direction::UP;
                        let last_diresction = visited_obstacles.insert(gp, guard_direction);
                        if last_diresction.is_some() && last_diresction.unwrap() == guard_direction
                        {
                            path_loop = true;
                            break;
                        }
                    }
                }
            }
            let gp = (guard_pos.0 as usize, guard_pos.1 as usize);
            map[gp.0][gp.1] = 'X';
        }

        if path_loop {
            count += 1;
        }
    }

    count
}
