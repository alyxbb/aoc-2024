use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    usize,
};

use crate::Solution;
use nalgebra::Vector2;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, EnumIter, Hash, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn get_move_vector(&self) -> Vector2<isize> {
        match self {
            Direction::North => Vector2::new(0, -1),
            Direction::East => Vector2::new(1, 0),
            Direction::South => Vector2::new(0, 1),
            Direction::West => Vector2::new(-1, 0),
        }
    }
    pub fn get_adj_dirs(&self) -> [Self; 2] {
        match self {
            Direction::North => [Direction::East, Direction::West],
            Direction::East => [Direction::North, Direction::South],
            Direction::South => [Direction::East, Direction::West],
            Direction::West => [Direction::North, Direction::South],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Node {
    distance: usize,
    visited: bool,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.visited == other.visited {
            return self.distance.cmp(&other.distance);
        } else {
            if self.visited {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
    }
}

fn parse(input: String) -> (HashMap<(Vector2<isize>, Direction), Node>, Vector2<isize>) {
    let mut map = HashMap::new();
    let mut end: Vector2<isize> = Vector2::new(0, 0);

    for (y, line) in input.lines().enumerate() {
        let y = y as isize;
        for (x, char) in line.chars().enumerate() {
            let x = x as isize;
            match char {
                'S' => {
                    for dir in Direction::iter() {
                        if dir == Direction::East {
                            map.insert(
                                (Vector2::new(x, y), dir),
                                Node {
                                    distance: 0,
                                    visited: false,
                                },
                            );
                            continue;
                        }
                        map.insert(
                            (Vector2::new(x, y), dir),
                            Node {
                                distance: usize::MAX,
                                visited: false,
                            },
                        );
                    }
                }
                'E' => {
                    end = Vector2::new(x, y);
                    for dir in Direction::iter() {
                        map.insert(
                            (Vector2::new(x, y), dir),
                            Node {
                                distance: usize::MAX,
                                visited: false,
                            },
                        );
                    }
                }
                '.' => {
                    for dir in Direction::iter() {
                        map.insert(
                            (Vector2::new(x, y), dir),
                            Node {
                                distance: usize::MAX,
                                visited: false,
                            },
                        );
                    }
                }
                '#' => continue,
                _ => panic!(),
            }
        }
    }

    (map, end)
}

pub fn part_1(input: String) -> Solution {
    let (mut map, end) = parse(input);

    'outer: loop {
        let mut nodes: Vec<_> = map.clone().into_iter().collect();
        println!(
            "{}/{}",
            nodes
                .iter()
                .filter(|node| node.1.visited)
                .collect::<Vec<_>>()
                .len(),
            nodes.len()
        );
        nodes.sort_by_key(|k| k.1);

        let current = &nodes[0];
        if current.1.visited == true {
            panic!()
        }

        map.get_mut(&current.0).unwrap().visited = true;

        if let Some(moved_point) =
            map.get_mut(&(current.0 .0 + current.0 .1.get_move_vector(), current.0 .1))
        {
            moved_point.distance = moved_point.distance.min(current.1.distance + 1);
        }

        for dir in current.0 .1.get_adj_dirs() {
            let rotated_point = map.get_mut(&(current.0 .0, dir)).unwrap();
            rotated_point.distance = rotated_point.distance.min(current.1.distance + 1000)
        }

        for dir in Direction::iter() {
            if map[&(end, dir)].visited {
                break 'outer;
            }
        }
    }

    let mut sol = usize::MAX;
    for dir in Direction::iter() {
        sol = sol.min(map[&(end, dir)].distance)
    }

    Solution::from(sol)
}

fn get_route_points(
    map: &HashMap<(Vector2<isize>, Direction), Node>,
    current: (Vector2<isize>, Direction),
) -> HashSet<Vector2<isize>> {
    let mut res = HashSet::new();
    res.insert(current.0);
    let current_node = map[&current];
    let to_check: Vec<((Vector2<isize>, Direction), usize)> = vec![
        (
            (current.0 + (current.1.get_move_vector() * -1), current.1),
            1,
        ),
        ((current.0, current.1.get_adj_dirs()[0]), 1000),
        ((current.0, current.1.get_adj_dirs()[1]), 1000),
    ];
    for (node_loc, dist) in to_check {
        if let Some(node) = map.get(&node_loc) {
            if node.distance == current_node.distance.overflowing_sub(dist).0 {
                res.extend(get_route_points(map, node_loc));
            }
        }
    }
    res
}

pub fn part_2(input: String) -> Solution {
    let (mut map, end) = parse(input);

    loop {
        let mut nodes: Vec<_> = map.clone().into_iter().collect();
        println!(
            "{}/{}",
            nodes
                .iter()
                .filter(|node| node.1.visited)
                .collect::<Vec<_>>()
                .len(),
            nodes.len()
        );
        nodes.sort_by_key(|k| k.1);

        let current = &nodes[0];
        if current.1.visited == true {
            break;
        }

        map.get_mut(&current.0).unwrap().visited = true;

        if let Some(moved_point) =
            map.get_mut(&(current.0 .0 + current.0 .1.get_move_vector(), current.0 .1))
        {
            moved_point.distance = moved_point.distance.min(current.1.distance + 1);
        }

        for dir in current.0 .1.get_adj_dirs() {
            let rotated_point = map.get_mut(&(current.0 .0, dir)).unwrap();
            rotated_point.distance = rotated_point.distance.min(current.1.distance + 1000)
        }
    }

    let mut distance = usize::MAX;
    let mut end_dir = Direction::North;
    for dir in Direction::iter() {
        if map[&(end, dir)].distance < distance {
            distance = map[&(end, dir)].distance;
            end_dir = dir;
        }
    }
    let sol = get_route_points(&map, (end, end_dir)).len();

    Solution::from(sol)
}
