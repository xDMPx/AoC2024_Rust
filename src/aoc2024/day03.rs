pub fn part01(file_path: &str) -> usize {
    let memory: String = std::fs::read_to_string(file_path).unwrap();

    let splits = memory.split("mul(").skip(1);

    let mulres = splits.map(|s| {
        if let Some((lnum, rnum)) = extract_mul_nums(s) {
            lnum * rnum
        } else {
            0
        }
    });

    mulres.sum()
}

fn extract_mul_nums(split: &str) -> Option<(usize, usize)> {
    let (lnum, rnum) = split.split_once(',')?;
    let lnum = lnum.parse().ok()?;
    let rnum = rnum.split_once(')')?.0.parse().ok()?;

    Some((lnum, rnum))
}
