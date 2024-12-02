use crate::Solution;

fn parse(input: String) -> (Vec<usize>, Vec<usize>) {
    let lines = input.lines();
    let mut left = vec![];
    let mut right = vec![];
    for line in lines{
        let mut line = line.split_whitespace().map(|x| x.parse::<usize>().expect("failed to parse"));
        left.push(line.next().expect("failed to parse"));
        right.push(line.next().expect("failed to parse"));
    }

    left.sort();
    right.sort();
    (left, right)
}

pub fn part_1(input: String) -> Solution {
    let (left, right) = parse(input);
    let pairs = left.iter().zip(right);

    let mut sol = 0;
    for pair in pairs{
        sol += pair.0.abs_diff(pair.1);
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let (mut left, right) = parse(input);
    left.dedup();
    
    let mut sol = 0;
    for value in left{
        let count = right.iter().filter(|&x| *x==value).count();
        sol += count * value;
    }
    Solution::from(sol)
}