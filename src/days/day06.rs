use std::{collections::HashSet, vec};

use rayon::prelude::*;

use crate::Solution;

#[derive(PartialEq, Eq, Clone, Copy)]

enum GridSquare {
    Empty,
    Full,
    StartPos,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Guard {
    pos: (usize, usize),
    dir: Direction,
}

impl Guard {
    fn get_next_move(&mut self) -> (usize, usize) {
        match self.dir {
            Direction::North => (self.pos.0, self.pos.1.overflowing_sub(1).0),
            Direction::South => (self.pos.0, self.pos.1 + 1),
            Direction::East => (self.pos.0 + 1, self.pos.1),
            Direction::West => (self.pos.0.overflowing_sub(1).0, self.pos.1),
        }
    }
}

fn parse(input: String) -> (Guard, Vec<Vec<GridSquare>>) {
    let mut map: Vec<Vec<GridSquare>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|cha| match cha {
                    '^' => GridSquare::StartPos,
                    '.' => GridSquare::Empty,
                    '#' => GridSquare::Full,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let mut start_pos = (0, 0);

    for (i, line) in map.clone().iter().enumerate() {
        let Some(j) = line.iter().position(|x| *x == GridSquare::StartPos) else {
            continue;
        };
        start_pos = (j, i);
        map[i][j] = GridSquare::Empty;
    }
    let guard = Guard {
        pos: start_pos,
        dir: Direction::North,
    };

    (guard, map)
}

fn check_loop(guard: &Guard, map: &[Vec<GridSquare>], pos: (usize, usize)) -> bool {
    let mut map = map.to_owned();
    let mut guard = *guard;
    let mut history = vec![];
    map[pos.1][pos.0] = GridSquare::Full;
    history.push(guard);

    loop {
        let next_pos = guard.get_next_move();
        let Some(row) = map.get(next_pos.1) else {
            return false;
        };
        let Some(square) = row.get(next_pos.0) else {
            return false;
        };
        match square {
            GridSquare::Empty => {
                guard.pos = next_pos;
            }
            GridSquare::Full => {
                guard.dir = guard.dir.rotate();
            }
            GridSquare::StartPos => unreachable!(),
        }
        if history.contains(&guard) {
            return true;
        }
        history.push(guard);
    }
}

pub fn part_1(input: String) -> Solution {
    let (mut guard, map) = parse(input);
    let mut visited_squares = vec![];
    visited_squares.push(guard.pos);

    loop {
        let next_pos = guard.get_next_move();
        let Some(row) = map.get(next_pos.1) else {
            break;
        };
        let Some(square) = row.get(next_pos.0) else {
            break;
        };
        match square {
            GridSquare::Empty => {
                guard.pos = next_pos;
                visited_squares.push(next_pos);
            }
            GridSquare::Full => guard.dir = guard.dir.rotate(),
            GridSquare::StartPos => unreachable!(),
        }
    }

    let unique_squares: HashSet<_> = visited_squares.into_iter().collect();

    Solution::from(unique_squares.len())
}

pub fn part_2(input: String) -> Solution {
    let (original_guard, map) = parse(input);
    let mut visited_squares = HashSet::new();
    let mut guard = original_guard;
    loop {
        let next_pos = guard.get_next_move();
        let Some(row) = map.get(next_pos.1) else {
            break;
        };
        let Some(square) = row.get(next_pos.0) else {
            break;
        };
        match square {
            GridSquare::Empty => {
                visited_squares.insert(next_pos);
                guard.pos = next_pos;
            }
            GridSquare::Full => {
                guard.dir = guard.dir.rotate();
            }
            GridSquare::StartPos => unreachable!(),
        }
    }

    visited_squares.remove(&original_guard.pos);
    let sol = visited_squares
        .into_par_iter()
        .filter(|square| check_loop(&original_guard, &map, *square))
        .count();

    Solution::from(sol)
}
