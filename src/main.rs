use aoc2024_rust::aoc2024;

fn main() {
    println!("\tDay 01: ");
    let day01_part1 = aoc2024::day01::part01("./puzzle_input/day01.txt");
    println!("\t\tpart 1: {day01_part1}");
    let day01_part2 = aoc2024::day01::part02("./puzzle_input/day01.txt");
    println!("\t\tpart 2: {day01_part2}");

    println!("\tDay 02: ");
    let day02_part1 = aoc2024::day02::part01("./puzzle_input/day02.txt");
    println!("\t\tpart 1: {day02_part1}");
    let day02_part2 = aoc2024::day02::part02("./puzzle_input/day02.txt");
    println!("\t\tpart 2: {day02_part2}");

    println!("\tDay 03: ");
    let day03_part1 = aoc2024::day03::part01("./puzzle_input/day03.txt");
    println!("\t\tpart 1: {day03_part1}");
    let day03_part2 = aoc2024::day03::part02("./puzzle_input/day03.txt");
    println!("\t\tpart 2: {day03_part2}");

    println!("\tDay 04: ");
    let day04_part1 = aoc2024::day04::part01("./puzzle_input/day04.txt");
    println!("\t\tpart 1: {day04_part1}");
}
