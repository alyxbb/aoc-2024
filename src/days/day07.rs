use crate::Solution;

fn parse(input: String) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            (
                parts.0.parse().unwrap(),
                parts.1.split(" ").map(|num| num.parse().unwrap()).collect(),
            )
        })
        .collect()
}
fn find_target(target: usize, mut nums: Vec<usize>) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let first = nums.remove(0);
    let second = nums.remove(0);
    let mut mul_option = nums.clone();
    let mut add_option = nums.clone();
    mul_option.insert(0, first * second);
    add_option.insert(0, first + second);
    find_target(target, add_option) || find_target(target, mul_option)
}

fn find_target_2(target: usize, mut nums: Vec<usize>) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let first = nums.remove(0);
    let second = nums.remove(0);
    let mut mul_option = nums.clone();
    let mut add_option = nums.clone();
    let mut con_option = nums.clone();
    mul_option.insert(0, first * second);
    add_option.insert(0, first + second);
    con_option.insert(0, format!("{}{}", first, second).parse().unwrap());
    find_target_2(target, add_option)
        || find_target_2(target, mul_option)
        || find_target_2(target, con_option)
}

pub fn part_1(input: String) -> Solution {
    let input = parse(input);
    let mut sol = 0;

    for line in input {
        if find_target(line.0, line.1) {
            sol += line.0
        }
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let input = parse(input);
    let mut sol = 0;

    for line in input {
        if find_target_2(line.0, line.1) {
            sol += line.0
        }
    }
    Solution::from(sol)
}
