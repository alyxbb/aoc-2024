use std::collections::{HashMap, HashSet};

use crate::Solution;

#[derive(Clone)]
pub struct Plot {
    plant: char,
    visited: bool,
}

fn parse(input: String) -> HashMap<(usize, usize), Plot> {
    let mut res = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            res.insert(
                (x, y),
                Plot {
                    plant: char,
                    visited: false,
                },
            );
        }
    }
    res
}

fn get_adjacent(
    coord: &(usize, usize),
    plant: char,
    mut map: HashMap<(usize, usize), Plot>,
) -> (Vec<(usize, usize)>, HashMap<(usize, usize), Plot>) {
    let Some(plot) = map.get_mut(coord) else {
        return (vec![], map);
    };
    if plot.visited == true || plot.plant != plant {
        return (vec![], map);
    }

    let mut res = vec![*coord];
    plot.visited = true;
    let to_check = [
        (coord.0 + 1, coord.1),
        (coord.0.overflowing_sub(1).0, coord.1),
        (coord.0, coord.1 + 1),
        (coord.0, coord.1.overflowing_sub(1).0),
    ];

    for point in to_check {
        let mut ret = get_adjacent(&point, plant, map);
        map = ret.1;
        res.append(&mut ret.0);
    }
    (res, map)
}

fn get_perimeter(region: Vec<(usize, usize)>) -> usize {
    let mut res = 0;
    for coord in &region {
        let to_check = vec![
            (coord.0 + 1, coord.1),
            (coord.0.overflowing_sub(1).0, coord.1),
            (coord.0, coord.1 + 1),
            (coord.0, coord.1.overflowing_sub(1).0),
        ];
        for val in to_check {
            if !region.contains(&val) {
                res += 1;
            }
        }
    }
    res
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate_r(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn rotate_l(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

fn get_sides(region: Vec<(usize, usize)>) -> usize {
    let mut edges = HashMap::new();
    for coord in &region {
        let to_check = vec![
            ((coord.0 + 1, coord.1), Direction::East),
            ((coord.0.overflowing_sub(1).0, coord.1), Direction::West),
            ((coord.0, coord.1 + 1), Direction::South),
            ((coord.0, coord.1.overflowing_sub(1).0), Direction::North),
        ];
        for val in to_check {
            if region.contains(&val.0) {
                continue;
            }
            edges.entry(val.0).or_insert(vec![]).push(val.1);
        }
    }
    let mut checked = vec![];
    let mut sides = 0;
    for (point, dirs) in &edges {
        for dir in dirs {
            if checked.contains(&(*point, dir)) {
                continue;
            }
            sides += 1;
            let left = find_points_on_side(point, dir, &edges, &dir.rotate_l());
            let right = find_points_on_side(point, dir, &edges, &dir.rotate_r());
            for val in left {
                checked.push((val, dir));
            }
            for val in right {
                checked.push((val, dir));
            }
        }
    }
    sides
}

fn find_points_on_side(
    point: &(usize, usize),
    edge_dir: &Direction,
    edges: &HashMap<(usize, usize), Vec<Direction>>,
    move_dir: &Direction,
) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    res.insert(*point);
    let to_check = match move_dir {
        Direction::North => (point.0, point.1.overflowing_sub(1).0),
        Direction::East => (point.0 + 1, point.1),
        Direction::South => (point.0, point.1 + 1),
        Direction::West => (point.0.overflowing_sub(1).0, point.1),
    };

    let Some(dirs) = edges.get(&to_check) else {
        return res;
    };
    if dirs.contains(edge_dir) {
        res.extend(find_points_on_side(&to_check, edge_dir, edges, move_dir));
    }

    res
}

pub fn part_1(input: String) -> Solution {
    let mut input = parse(input);

    let mut res = 0;
    loop {
        let mut found = false;
        for (coords, plot) in input.clone().iter() {
            if plot.visited == true {
                continue;
            }
            found = true;
            let ret = get_adjacent(coords, plot.plant, input);
            input = ret.1;
            let region = ret.0;
            let area = region.len();
            let perimeter = get_perimeter(region);
            res += area * perimeter;
        }
        if !found {
            break;
        }
    }

    Solution::from(res)
}

pub fn part_2(input: String) -> Solution {
    let mut input = parse(input);

    let mut res = 0;
    loop {
        let mut found = false;
        for (coords, plot) in input.clone().iter() {
            if plot.visited == true {
                continue;
            }
            found = true;
            let ret = get_adjacent(coords, plot.plant, input);
            input = ret.1;
            let region = ret.0;
            let area = region.len();
            let sides = get_sides(region);
            res += area * sides;
        }
        if !found {
            break;
        }
    }

    Solution::from(res)
}
