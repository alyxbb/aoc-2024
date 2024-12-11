use std::collections::HashMap;

use crate::Solution;

fn parse(input: String) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

pub fn part_1(input: String) -> Solution {
    let mut input = parse(input);
    for _ in 0..25 {
        let mut next = vec![];
        for val in input {
            match val {
                0 => next.push(1),
                v if (v.checked_ilog10().unwrap() + 1) % 2 == 0 => {
                    let len = (v.checked_ilog10().unwrap() + 1) / 2;
                    let pow = 10_usize.pow(len);
                    next.push(v / pow);
                    next.push(v % pow);
                }
                v => {
                    next.push(v * 2024);
                }
            }
        }
        input = next;
    }

    let sol = input.len();
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let input = parse(input);
    let mut nums = HashMap::new();
    for key in input {
        *nums.entry(key).or_insert(0) += 1_usize;
    }
    for _ in 0..75 {
        let mut next = HashMap::new();
        for (key, val) in nums.iter() {
            match key {
                0 => {
                    *next.entry(1).or_insert(0) += *val;
                }
                k if (k.checked_ilog10().unwrap() + 1) % 2 == 0 => {
                    let len = (k.checked_ilog10().unwrap() + 1) / 2;
                    let pow = 10_usize.pow(len);
                    *next.entry(k / pow).or_insert(0) += val;
                    *next.entry(k % pow).or_insert(0) += val;
                }
                k => {
                    *next.entry(k * 2024).or_insert(0) += val;
                }
            }
        }
        nums = next;
    }
    let mut sol = 0;
    for (_, val) in nums {
        sol += val;
    }
    Solution::from(sol)
}
