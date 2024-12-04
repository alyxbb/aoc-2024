use crate::Solution;

fn parse(input: String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn check_valid(line: &mut Vec<usize>) -> bool {
    if !line.is_sorted() {
        line.reverse();
        if !line.is_sorted() {
            return false;
        }
    }
    let mut last = line.pop().unwrap();
    while let Some(current) = line.pop() {
        let diff = current.abs_diff(last);
        last = current;
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

pub fn part_1(input: String) -> Solution {
    let input = parse(input);

    let mut sol = 0;
    for mut line in input {
        if check_valid(&mut line) {
            sol += 1;
        }
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let input = parse(input);

    let mut sol = 0;
    for line in input {
        if check_valid(&mut line.clone()) {
            sol += 1;
        } else {
            for i in 0..line.len() {
                let mut new_line = line.clone();
                new_line.remove(i);
                if check_valid(&mut new_line) {
                    sol += 1;
                    break;
                }
            }
        }
    }
    Solution::from(sol)
}
