// https://adventofcode.com/2016/day/2

use std::fs;

fn part1(input: &str) -> String {
    let mut ans: String = String::new();
    let keypad: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let mut x: usize = 1;
    let mut y: usize = 1;

    for line in input.lines() {
        for direction in line.chars() {
            match direction {
                'U' => x = if x == 0 { 0 } else { x - 1 },
                'D' => x = if x == 2 { 2 } else { x + 1 },
                'L' => y = if y == 0 { 0 } else { y - 1 },
                'R' => y = if y == 2 { 2 } else { y + 1 },
                _ => {}
            }
        }

        ans = format!("{}{}", ans, keypad[x][y]);
    }

    ans
}

fn part2(input: &str) -> String {
    let mut ans: String = String::new();
    let keypad: [[char; 7]; 7] = [
        ['0', '0', '0', '0', '0', '0', '0'],
        ['0', '0', '0', '1', '0', '0', '0'],
        ['0', '0', '2', '3', '4', '0', '0'],
        ['0', '5', '6', '7', '8', '9', '0'],
        ['0', '0', 'A', 'B', 'C', '0', '0'],
        ['0', '0', '0', 'D', '0', '0', '0'],
        ['0', '0', '0', '0', '0', '0', '0'],
    ];

    let mut x: usize = 3;
    let mut y: usize = 1;

    for line in input.lines() {
        for direction in line.chars() {
            match direction {
                'U' => x = if keypad[x - 1][y] == '0' { x } else { x - 1 },
                'D' => x = if keypad[x + 1][y] == '0' { x } else { x + 1 },
                'L' => y = if keypad[x][y - 1] == '0' { y } else { y - 1 },
                'R' => y = if keypad[x][y + 1] == '0' { y } else { y + 1 },
                _ => {}
            }
        }

        ans = format!("{}{}", ans, keypad[x][y]);
    }

    ans
}
fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Failed to read input file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}


