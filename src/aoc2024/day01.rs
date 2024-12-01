pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let lists = puzzle_input.lines().map(|line| {
        let mut line = line.trim().split("   ");
        let lv = line.next().unwrap().trim().parse::<u32>().unwrap();
        let rv = line.next().unwrap().trim().parse::<u32>().unwrap();

        (lv, rv)
    });

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = lists.unzip();
    left.sort_unstable();
    right.sort_unstable();

    let distances = left.into_iter().zip(right.into_iter()).map(|(l, r)| {
        let l = l as i32;
        let r = r as i32;
        let d = i32::abs(r - l);

        let d: usize = d.try_into().unwrap();
        d
    });

    distances.sum()
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let lists = puzzle_input.lines().map(|line| {
        let mut line = line.trim().split("   ");
        let lv = line.next().unwrap().trim().parse::<u32>().unwrap();
        let rv = line.next().unwrap().trim().parse::<u32>().unwrap();

        (lv, rv)
    });

    let (left, right): (Vec<u32>, Vec<u32>) = lists.unzip();

    let mut similarity_score = 0;
    for l in left.iter() {
        let similarity = right.iter().filter(|x| **x == *l).count();
        similarity_score += (*l as usize) * similarity;
    }

    similarity_score
}
