use std::collections::HashSet;

use crate::Solution;

fn parse(input: String) -> (Vec<Vec<u8>>, HashSet<(usize, usize)>) {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut trailheads = HashSet::new();
    for (y, line) in map.iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            if *num == 0 {
                trailheads.insert((x, y));
            }
        }
    }
    (map, trailheads)
}

fn check_route_valid(
    trailhead: &(usize, usize),
    trailhead_num: u8,
    map: &Vec<Vec<u8>>,
) -> HashSet<(usize, usize)> {
    let mut found_positions = HashSet::new();

    let to_check = vec![
        (trailhead.0 + 1, trailhead.1),
        (trailhead.0, trailhead.1 + 1),
        (trailhead.0.overflowing_sub(1).0, trailhead.1),
        (trailhead.0, trailhead.1.overflowing_sub(1).0),
    ];

    for pos in to_check {
        let Some(row) = map.get(pos.1) else {
            continue;
        };
        let Some(val) = row.get(pos.0) else {
            continue;
        };
        if *val == trailhead_num + 1 {
            if *val == 9 {
                found_positions.insert(pos);
            } else {
                found_positions.extend(check_route_valid(&pos, trailhead_num + 1, map));
            }
        }
    }
    found_positions
}

fn check_route_valid_2(
    trailhead: &(usize, usize),
    trailhead_num: u8,
    map: &Vec<Vec<u8>>,
) -> Vec<(usize, usize)> {
    let mut found_positions = vec![];

    let to_check = vec![
        (trailhead.0 + 1, trailhead.1),
        (trailhead.0, trailhead.1 + 1),
        (trailhead.0.overflowing_sub(1).0, trailhead.1),
        (trailhead.0, trailhead.1.overflowing_sub(1).0),
    ];

    for pos in to_check {
        let Some(row) = map.get(pos.1) else {
            continue;
        };
        let Some(val) = row.get(pos.0) else {
            continue;
        };
        if *val == trailhead_num + 1 {
            if *val == 9 {
                found_positions.push(pos);
            } else {
                found_positions.extend(check_route_valid_2(&pos, trailhead_num + 1, map));
            }
        }
    }
    found_positions
}

pub fn part_1(input: String) -> Solution {
    let (map, trailheads) = parse(input);
    let mut sol = 0;

    for trailhead in trailheads {
        sol += check_route_valid(&trailhead, 0, &map).len();
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let (map, trailheads) = parse(input);
    let mut sol = 0;

    for trailhead in trailheads {
        sol += check_route_valid_2(&trailhead, 0, &map).len();
    }
    Solution::from(sol)
}
