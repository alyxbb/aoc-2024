use std::collections::{HashMap, HashSet};

use crate::Solution;

fn parse(input: String) -> (HashMap<char, HashSet<(isize, isize)>>, (usize, usize)) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut antennas = HashMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char != '.' {
                antennas
                    .entry(*char)
                    .or_insert(HashSet::new())
                    .insert((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    (antennas, (width, height))
}

pub fn part_1(input: String) -> Solution {
    let (antennas, size) = parse(input);
    let mut antinodes = HashSet::new();

    for (_key, value) in antennas.iter() {
        for point_1 in value {
            for point_2 in value {
                if point_1 == point_2 {
                    continue;
                }
                let diff = (point_2.0 - point_1.0, point_2.1 - point_1.1);
                let first_antinode = (point_2.0 + diff.0, point_2.1 + diff.1);
                let second_antinode = (point_1.0 - diff.0, point_1.1 - diff.1);
                antinodes.insert(first_antinode);
                antinodes.insert(second_antinode);
            }
        }
    }
    let sol = antinodes
        .iter()
        .filter(|antinode| {
            antinode.0 < size.0.try_into().unwrap()
                && antinode.1 < size.1.try_into().unwrap()
                && antinode.0 >= 0
                && antinode.1 >= 0
        })
        .count();
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let (antennas, size) = parse(input);
    let mut antinodes = HashSet::new();

    for (_key, value) in antennas.iter() {
        for point_1 in value {
            for point_2 in value {
                if point_1 == point_2 {
                    continue;
                }
                let diff = (point_2.0 - point_1.0, point_2.1 - point_1.1);
                let mut hcf = 0;
                for i in 1..=diff.0.abs().min(diff.1.abs()) {
                    if diff.0 % i == 0 && diff.1 % i == 0 {
                        hcf = i;
                    }
                }
                let jumps = (diff.0 / hcf, diff.1 / hcf);
                let mut next = *point_1;

                let mut i = 0;

                while 0 <= next.0
                    && 0 <= next.1
                    && next.0 < size.0.try_into().unwrap()
                    && next.1 < size.1.try_into().unwrap()
                {
                    antinodes.insert(next);
                    i += 1;
                    next = (point_1.0 + (jumps.0 * i), point_1.1 + (jumps.1 * i))
                }
                let mut next = *point_1;
                while 0 <= next.0
                    && 0 <= next.1
                    && next.0 < size.0.try_into().unwrap()
                    && next.1 < size.1.try_into().unwrap()
                {
                    antinodes.insert(next);
                    i -= 1;
                    next = (point_1.0 + (jumps.0 * i), point_1.1 + (jumps.1 * i))
                }
            }
        }
    }
    let sol = antinodes.len();
    Solution::from(sol)
}
