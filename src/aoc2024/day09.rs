#[derive(Debug, Clone, Copy)]
struct DiskSpace {
    id: Option<usize>,
    pos: (usize, usize),
}

pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let disk_map: Vec<usize> = puzzle_input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect();

    let mut free = false;
    let mut id = 0;
    let mut files = vec![];
    let mut empty_spaces = vec![];
    let mut i = 0;
    for x in disk_map {
        if x == 0 {
            free = !free;
            continue;
        }
        if free {
            empty_spaces.push(DiskSpace {
                id: None,
                pos: (i, i + x - 1),
            });
        } else {
            files.push(DiskSpace {
                id: Some(id),
                pos: (i, i + x - 1),
            });
            id += 1;
        }
        free = !free;
        i = i + x;
    }
    empty_spaces.reverse();

    let mut new_files = vec![];
    while let Some(empty_space) = empty_spaces.pop() {
        let empty_space_len = empty_space.pos.1 - empty_space.pos.0 + 1;
        let last = files.pop().unwrap();
        if empty_space.pos.0 > last.pos.0 {
            files.push(last);
            break;
        }
        let last_len = last.pos.1 - last.pos.0 + 1;
        if last_len == empty_space_len {
            new_files.push(DiskSpace {
                id: last.id,
                pos: empty_space.pos,
            });
        } else if last_len < empty_space_len {
            new_files.push(DiskSpace {
                id: last.id,
                pos: (empty_space.pos.0, empty_space.pos.0 + last_len - 1),
            });
            empty_spaces.push(DiskSpace {
                id: empty_space.id,
                pos: (empty_space.pos.0 + last_len, empty_space.pos.1),
            });
        } else if last_len > empty_space_len {
            new_files.push(DiskSpace {
                id: last.id,
                pos: empty_space.pos,
            });
            files.push(DiskSpace {
                id: last.id,
                pos: (last.pos.0, last.pos.1 - empty_space_len),
            });
        }
    }

    files.append(&mut new_files);
    let mut sum = 0;
    for f in files {
        for i in f.pos.0..=f.pos.1 {
            sum += i * f.id.unwrap();
        }
    }

    sum
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let disk_map: Vec<usize> = puzzle_input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect();

    let mut free = false;
    let mut id = 0;
    let mut files = vec![];
    let mut empty_spaces = vec![];
    let mut i = 0;
    for x in disk_map {
        if x == 0 {
            free = !free;
            continue;
        }
        if free {
            empty_spaces.push(DiskSpace {
                id: None,
                pos: (i, i + x - 1),
            });
        } else {
            files.push(DiskSpace {
                id: Some(id),
                pos: (i, i + x - 1),
            });
            id += 1;
        }
        free = !free;
        i = i + x;
    }

    let mut new_files = vec![];
    while let Some(mut file) = files.pop() {
        let file_len = file.pos.1 - file.pos.0 + 1;

        let space = empty_spaces
            .iter()
            .enumerate()
            .filter(|(_, s)| (s.pos.1 - s.pos.0 + 1) >= file_len && s.pos.0 < file.pos.0)
            .min_by_key(|(_, s)| s.pos.1);

        if let Some((i, space)) = space {
            let mut space = *space;
            empty_spaces.push(DiskSpace {
                id: None,
                pos: file.pos,
            });
            let space_len = space.pos.1 - space.pos.0 + 1;
            if space_len == file_len {
                file.pos = space.pos;
            } else {
                file.pos.0 = space.pos.0;
                file.pos.1 = space.pos.0 + file_len - 1;
                space.pos.0 = space.pos.0 + file_len;
                empty_spaces.push(space);
            }
            new_files.push(file);
            empty_spaces.remove(i);
        } else {
            new_files.push(file);
        }
    }

    files.append(&mut new_files);
    let mut sum = 0;
    for f in files {
        for i in f.pos.0..=f.pos.1 {
            sum += i * f.id.unwrap();
        }
    }

    sum
}
