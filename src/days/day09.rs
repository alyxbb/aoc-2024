use crate::Solution;

fn parse(input: String) -> Vec<Option<usize>> {
    let mut res = vec![];
    let mut next_num = 0;
    let mut empty = false;

    for num in input.chars().map(|char| char.to_digit(10).unwrap()) {
        for _ in 0..num {
            let to_add = match empty {
                true => None,
                false => Some(next_num),
            };
            res.push(to_add);
        }
        empty = !empty;
        if empty {
            next_num += 1;
        }
    }
    res
}

pub fn part_1(input: String) -> Solution {
    let mut input = parse(input);

    while input.contains(&None) {
        let last = input.pop().unwrap();
        if last == None {
            continue;
        }
        let free = input.iter().position(|value| *value == None).unwrap();
        input[free] = last;
    }

    let mut sol = 0;
    for (i, val) in input.iter().enumerate() {
        sol += i * val.unwrap();
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let mut input = parse(input);
    let max_id = input
        .iter()
        .filter(|val| **val != None)
        .last()
        .unwrap()
        .unwrap();

    for id in (0..=max_id).rev() {
        let size = input.iter().filter(|val| **val == Some(id)).count();
        if size == 0 {
            continue;
        }

        let Some(start_pos) = input
            .windows(size)
            .position(|window| window.iter().filter(|item| **item != None).count() == 0)
        else {
            continue;
        };
        let from_pos = input.iter().position(|val| *val == Some(id)).unwrap();
        if from_pos < start_pos {
            continue;
        }
        input = input
            .iter()
            .map(|val| if *val == Some(id) { None } else { *val })
            .collect();
        for pos in start_pos..start_pos + size {
            input[pos] = Some(id);
        }
    }

    let mut sol = 0;
    for (i, val) in input.iter().enumerate() {
        let val = match val {
            Some(x) => x,
            None => &0,
        };
        sol += i * val;
    }
    Solution::from(sol)
}
