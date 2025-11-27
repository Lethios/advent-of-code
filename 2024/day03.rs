// https://adventofcode.com/2024/day/3

use regex::Regex;
use std::fs;

fn part1(input: &str) -> u32 {
    let mut multiplication_sum: u32 = 0;
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for line in input.lines() {
        for caps in re.captures_iter(line) {
            multiplication_sum += caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap();
        }
    }

    multiplication_sum
}

fn part2(input: &str) -> u32 {
    0
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

