pub mod aoc2024;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let day01_part1 = aoc2024::day01::part01("./test_puzzle_input/day01_test.txt");
        assert_eq!(day01_part1, 11);
    }
}
