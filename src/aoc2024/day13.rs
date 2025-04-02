#[derive(Debug)]
struct PrizeData {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let prizes = extract_prizes(puzzle_input);

    let mut tokens = 0;
    for prize in prizes {
        let mut token_cost = None;
        let mut i = 0;
        while i * prize.button_a.0 < prize.prize.0 {
            let j = (prize.prize.0 - i * prize.button_a.0) / (prize.button_b.0);
            if i * prize.button_a.0 + j * prize.button_b.0 == prize.prize.0
                && i * prize.button_a.1 + j * prize.button_b.1 == prize.prize.1
            {
                if token_cost.is_none() {
                    token_cost = Some(3 * i + j);
                } else {
                    token_cost = Some(token_cost.unwrap().min(3 * i + j));
                }
            }
            i += 1;
        }
        tokens += token_cost.unwrap_or(0);
    }

    tokens
}

fn extract_prizes(puzzle_input: String) -> Vec<PrizeData> {
    let list = puzzle_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() != 0);
    let mut state = 0;
    let mut prizes = vec![];
    for data in list {
        match state {
            0 => {
                let mut pd = PrizeData {
                    button_a: (0, 0),
                    button_b: (0, 0),
                    prize: (0, 0),
                };
                let (x, y) = data
                    .trim_start_matches("Button A: X+")
                    .split_once(',')
                    .unwrap();
                let x = x.parse().unwrap();
                let y = y.trim_start_matches(" Y+").parse().unwrap();
                pd.button_a = (x, y);
                prizes.push(pd);
                state = 1;
            }
            1 => {
                let mut pd = prizes.pop().unwrap();
                let (x, y) = data
                    .trim_start_matches("Button B: X+")
                    .split_once(',')
                    .unwrap();
                let x = x.parse().unwrap();
                let y = y.trim_start_matches(" Y+").parse().unwrap();
                pd.button_b = (x, y);
                prizes.push(pd);
                state = 2;
            }
            2 => {
                let mut pd = prizes.pop().unwrap();
                let (x, y) = data
                    .trim_start_matches("Prize: X=")
                    .split_once(',')
                    .unwrap();
                let x = x.parse().unwrap();
                let y = y.trim_start_matches(" Y=").parse().unwrap();
                pd.prize = (x, y);
                prizes.push(pd);
                state = 0;
            }
            _ => {
                unreachable!("Invalid state");
            }
        }
    }

    prizes
}
