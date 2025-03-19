pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let equations = puzzle_input.lines();

    let mut sum = 0;
    for equation in equations {
        let (test_value, nums) = equation.split_once(':').unwrap();
        let test_value = test_value.parse().unwrap();
        let nums: Vec<usize> = nums.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        if true_equation(&nums, test_value, 0) {
            sum += test_value;
        }
    }

    sum
}

fn true_equation(nums: &[usize], test_value: usize, val: usize) -> bool {
    if nums.len() == 0 {
        if val == test_value {
            return true;
        } else {
            return false;
        }
    } else {
        return true_equation(&nums[1..], test_value, val + nums[0])
            || true_equation(&nums[1..], test_value, val * nums[0]);
    }
}

pub fn part02(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let equations = puzzle_input.lines();

    let mut sum = 0;
    for equation in equations {
        let (test_value, nums) = equation.split_once(':').unwrap();
        let test_value = test_value.parse().unwrap();
        let nums: Vec<usize> = nums.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        if true_equation_v2(&nums, test_value, 0) {
            sum += test_value;
        }
    }

    sum
}

fn concatenation(a: usize, b: usize) -> usize {
    let a = a.to_string();
    let b = b.to_string();

    (a + &b).parse().unwrap()
}

fn true_equation_v2(nums: &[usize], test_value: usize, val: usize) -> bool {
    if nums.len() == 0 {
        if val == test_value {
            return true;
        } else {
            return false;
        }
    } else {
        return true_equation_v2(&nums[1..], test_value, val + nums[0])
            || true_equation_v2(&nums[1..], test_value, val * nums[0])
            || true_equation_v2(&nums[1..], test_value, concatenation(val, nums[0]));
    }
}
