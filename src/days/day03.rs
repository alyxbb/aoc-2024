use regex::Regex;

use crate::Solution;

pub fn part_1(input: String) -> Solution {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let captures = regex.captures_iter(&input);
    let mut sol = 0;

    for capture in captures {
        sol += capture.get(1).unwrap().as_str().parse::<usize>().unwrap() * capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do(n't)?\(\)").unwrap();
    let captures = regex.captures_iter(&input);

    let mut sol = 0;
    let mut enabled = true;

    for capture in captures {
        match &capture.get(0).unwrap().as_str()[..3] {
            "do(" => enabled = true,
            "don" => enabled = false,
            "mul" => {
                if enabled {
                    sol += capture.get(1).unwrap().as_str().parse::<usize>().unwrap() * capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                }
            }
            _ => panic!()
            
        }
    }
    Solution::from(sol)
}
