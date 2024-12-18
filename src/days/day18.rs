use std::usize;

use crate::Solution;

#[derive(Clone, Copy, Debug)]
struct Node {
    visited: bool,
    distance: usize,
    passable: bool,
}

fn parse(input: String) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let nums = line.split_once(",").unwrap();
            (nums.0.parse().unwrap(), nums.1.parse().unwrap())
        })
        .collect()
}

pub fn part_1(input: String) -> Solution {
    let input = parse(input);
    let mut nodes = vec![
        vec![
            Node {
                visited: false,
                distance: usize::MAX,
                passable: true
            };
            71
        ];
        71
    ];
    nodes[0][0] = Node {
        visited: false,
        distance: 0,
        passable: true,
    };
    for i in &input[0..1024] {
        nodes[i.1][i.0].passable = false;
    }
    while nodes[70][70].visited == false {
        let mut min_loc = (0, 0);
        let mut min = usize::MAX;
        for (y, line) in nodes.iter().enumerate() {
            for (x, node) in line.iter().enumerate() {
                if node.passable == true && node.visited == false && node.distance < min {
                    min = node.distance;
                    min_loc = (x, y);
                }
            }
        }
        let node = nodes[min_loc.1][min_loc.0];
        nodes[min_loc.1][min_loc.0].visited = true;
        let to_visit = [
            (min_loc.0 + 1, min_loc.1),
            (min_loc.0.overflowing_sub(1).0, min_loc.1),
            (min_loc.0, min_loc.1 + 1),
            (min_loc.0, min_loc.1.overflowing_sub(1).0),
        ];
        for loc in to_visit {
            let Some(line) = nodes.get_mut(loc.1) else {
                continue;
            };
            let Some(next_node) = line.get_mut(loc.0) else {
                continue;
            };
            if next_node.visited == false
                && next_node.passable == true
                && node.distance + 1 < next_node.distance
            {
                next_node.distance = node.distance + 1;
            }
        }
        //do dijkestras
    }

    let sol = nodes[70][70].distance;
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let input = parse(input);
    let mut initial_nodes = vec![
        vec![
            Node {
                visited: false,
                distance: usize::MAX,
                passable: true
            };
            71
        ];
        71
    ];
    initial_nodes[0][0] = Node {
        visited: false,
        distance: 0,
        passable: true,
    };
    let mut i = 0;
    'outer: loop {
        initial_nodes[input[i].1][input[i].0].passable = false;
        let mut nodes = initial_nodes.clone();
        while nodes[70][70].visited == false {
            let mut min_loc = (0, 0);
            let mut min = usize::MAX;
            for (y, line) in nodes.iter().enumerate() {
                for (x, node) in line.iter().enumerate() {
                    if node.passable == true && node.visited == false && node.distance < min {
                        min = node.distance;
                        min_loc = (x, y);
                    }
                }
            }
            if min == usize::MAX {
                break 'outer;
            }

            let node = nodes[min_loc.1][min_loc.0];
            nodes[min_loc.1][min_loc.0].visited = true;
            let to_visit = [
                (min_loc.0 + 1, min_loc.1),
                (min_loc.0.overflowing_sub(1).0, min_loc.1),
                (min_loc.0, min_loc.1 + 1),
                (min_loc.0, min_loc.1.overflowing_sub(1).0),
            ];
            for loc in to_visit {
                let Some(line) = nodes.get_mut(loc.1) else {
                    continue;
                };
                let Some(next_node) = line.get_mut(loc.0) else {
                    continue;
                };
                if next_node.visited == false
                    && next_node.passable == true
                    && node.distance + 1 < next_node.distance
                {
                    next_node.distance = node.distance + 1;
                }
            }
        }
        i += 1;
    }
    let sol = format!("{},{}", input[i].0, input[i].1);
    Solution::from(sol)
}
