pub mod aoc2024;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let day01_part1 = aoc2024::day01::part01("./test_puzzle_input/day01_test.txt");
        assert_eq!(day01_part1, 11);
        let day01_part2 = aoc2024::day01::part02("./test_puzzle_input/day01_test.txt");
        assert_eq!(day01_part2, 31);

        let day02_part1 = aoc2024::day02::part01("./test_puzzle_input/day02_test.txt");
        assert_eq!(day02_part1, 2);
        let day02_part2 = aoc2024::day02::part02("./test_puzzle_input/day02_test.txt");
        assert_eq!(day02_part2, 4);

        let day03_part1 = aoc2024::day03::part01("./test_puzzle_input/day03_test.txt");
        assert_eq!(day03_part1, 161);
        let day03_part2 = aoc2024::day03::part02("./test_puzzle_input/day03_test_2.txt");
        assert_eq!(day03_part2, 48);

        let day04_part1 = aoc2024::day04::part01("./test_puzzle_input/day04_test.txt");
        assert_eq!(day04_part1, 18);
        let day04_part2 = aoc2024::day04::part02("./test_puzzle_input/day04_test.txt");
        assert_eq!(day04_part2, 9);

        let day05_part1 = aoc2024::day05::part01("./test_puzzle_input/day05_test.txt");
        assert_eq!(day05_part1, 143);
        let day05_part2 = aoc2024::day05::part02("./test_puzzle_input/day05_test.txt");
        assert_eq!(day05_part2, 123);

        let day06_part1 = aoc2024::day06::part01("./test_puzzle_input/day06_test.txt");
        assert_eq!(day06_part1, 41);
        let day06_part2 = aoc2024::day06::part02("./test_puzzle_input/day06_test.txt");
        assert_eq!(day06_part2, 6);

        let day07_part1 = aoc2024::day07::part01("./test_puzzle_input/day07_test.txt");
        assert_eq!(day07_part1, 3749);
        let day07_part2 = aoc2024::day07::part02("./test_puzzle_input/day07_test.txt");
        assert_eq!(day07_part2, 11387);

        let day08_part1 = aoc2024::day08::part01("./test_puzzle_input/day08_test.txt");
        assert_eq!(day08_part1, 14);
        let day08_part2 = aoc2024::day08::part02("./test_puzzle_input/day08_test.txt");
        assert_eq!(day08_part2, 34);

        let day09_part1 = aoc2024::day09::part01("./test_puzzle_input/day09_test.txt");
        assert_eq!(day09_part1, 1928);
        let day09_part2 = aoc2024::day09::part02("./test_puzzle_input/day09_test.txt");
        assert_eq!(day09_part2, 2858);

        let day10_part1 = aoc2024::day10::part01("./test_puzzle_input/day10_test.txt");
        assert_eq!(day10_part1, 36);
    }
}
