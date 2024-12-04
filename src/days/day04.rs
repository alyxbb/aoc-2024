use crate::Solution;

const SEARCH_WORD: &str = "XMAS";

fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_1(input: String) -> Solution {
    let input = parse(input);

    let mut sol = 0;

    for row in &input {
        for i in 0..row.len() - 3 {
            let mut valid = true;
            for (j, char) in SEARCH_WORD.chars().enumerate() {
                if !(*row.get(i + j).unwrap() == char) {
                    valid = false;
                    break;
                }
            }
            if valid {
                sol += 1;
            }
            let mut valid = true;
            for (j, char) in SEARCH_WORD.chars().rev().enumerate() {
                if !(*row.get(i + j).unwrap() == char) {
                    valid = false;
                    break;
                }
            }
            if valid {
                sol += 1;
            }
        }
    }

    for col in 0..input.get(0).unwrap().len() {
        for i in 0..input.len() - 3 {
            let mut valid: bool = true;
            for (j, char) in SEARCH_WORD.chars().enumerate() {
                if !(*input.get(i + j).unwrap().get(col).unwrap() == char) {
                    valid = false;
                    break;
                }
            }
            if valid {
                sol += 1;
            }
            let mut valid = true;
            for (j, char) in SEARCH_WORD.chars().rev().enumerate() {
                if !(*input.get(i + j).unwrap().get(col).unwrap() == char) {
                    valid = false;
                    break;
                }
            }
            if valid {
                sol += 1;
            }
        }
    }

    for col in 3..input.get(0).unwrap().len() {
        for i in 0..input.len() - 3 {
            let mut valid: bool = true;
            for (j, char) in SEARCH_WORD.chars().enumerate() {
                if !(*input.get(i + j).unwrap().get(col - j).unwrap() == char) {
                    valid = false;
                    break;
                }
            }
            if valid {
                sol += 1;
            }
            let mut valid = true;
            for (j, char) in SEARCH_WORD.chars().rev().enumerate() {
                if !(*input.get(i + j).unwrap().get(col - j).unwrap() == char) {
                    valid = false;
                    break;
                }
            }
            if valid {
                sol += 1;
            }
        }
    }

    for col in 0..input.get(0).unwrap().len() - 3 {
        for i in 0..input.len() - 3 {
            let mut valid: bool = true;
            for (j, char) in SEARCH_WORD.chars().enumerate() {
                if !(*input.get(i + j).unwrap().get(col + j).unwrap() == char) {
                    valid = false;
                    break;
                }
            }
            if valid {
                sol += 1;
            }
            let mut valid = true;
            for (j, char) in SEARCH_WORD.chars().rev().enumerate() {
                if !(*input.get(i + j).unwrap().get(col + j).unwrap() == char) {
                    valid = false;
                    break;
                }
            }
            if valid {
                sol += 1;
            }
        }
    }

    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let input = parse(input);

    let mut sol = 0;

    for row in 0..input.len() - 2 {
        for col in 0..input.get(row).unwrap().len() - 2 {
            if *input.get(row + 1).unwrap().get(col + 1).unwrap() != 'A' {
                continue;
            }
            let top_left = input.get(row).unwrap().get(col).unwrap();
            let bottom_right = input.get(row + 2).unwrap().get(col + 2).unwrap();
            let top_right = input.get(row).unwrap().get(col + 2).unwrap();
            let bottom_left = input.get(row + 2).unwrap().get(col).unwrap();
            if "MS".contains(*top_left)
                && "MS".contains(*top_right)
                && "MS".contains(*bottom_left)
                && "MS".contains(*bottom_right)
                && top_left != bottom_right
                && bottom_left != top_right
            {
                sol += 1;
            }
        }
    }

    Solution::from(sol)
}
