pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let reports = puzzle_input.lines().map(|line| {
        let line = line.trim().split(' ').map(|x| x.parse::<i32>().unwrap());
        line.collect::<Vec<i32>>()
    });

    let safe_reports = reports
        .map(|levels| {
            let plevel = levels.iter();
            let llevels = levels.iter().skip(1);

            let sum = llevels
                .zip(plevel)
                .map(|(p, l)| {
                    let diff = p - l;
                    if diff < 4 && diff > -4 {
                        i32::signum(diff)
                    } else {
                        0
                    }
                })
                .sum::<i32>();
            let sum = i32::abs(sum);

            sum + 1 == levels.len().try_into().unwrap()
        })
        .filter(|x| *x);

    safe_reports.count()
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let reports = puzzle_input.lines().map(|line| {
        let line = line.trim().split(' ').map(|x| x.parse::<i32>().unwrap());
        line.collect::<Vec<i32>>()
    });

    let safe_reports = reports
        .map(|levels| {
            if is_safe(&levels) {
                return true;
            }

            (0..levels.len()).any(|i| {
                let (left, right) = levels.split_at(i);
                let levels = [left, &right[1..]].concat();
                if is_safe(&levels) {
                    return true;
                }

                false
            })
        })
        .filter(|x| *x);

    safe_reports.count()
}

fn is_safe(levels: &[i32]) -> bool {
    let plevel = levels.iter();
    let llevels = levels.iter().skip(1);

    let sum = llevels
        .zip(plevel)
        .map(|(p, l)| {
            let diff = p - l;
            if diff < 4 && diff > -4 {
                i32::signum(diff)
            } else {
                0
            }
        })
        .sum::<i32>();
    let sum = i32::abs(sum);

    sum + 1 == levels.len().try_into().unwrap()
}
