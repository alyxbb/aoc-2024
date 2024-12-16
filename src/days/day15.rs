use nalgebra::Vector2;
use rayon::iter::Map;

use crate::Solution;

fn parse(input: String) -> (Vec<Vec<MapContent>>, Vec<Direction>, Vector2<i32>) {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let map_vec: Vec<Vec<MapContent>> = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' | '@' => MapContent::Empty,
                    '#' => MapContent::Immovable,
                    'O' => MapContent::Movable,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let mut coords = Vector2::new(0, 0);
    for (y, line) in map.lines().enumerate() {
        let Some(x) = line.find('@') else {
            continue;
        };
        coords = Vector2::new(x as i32, y as i32);
        break;
    }

    let mut instructions_vec = vec![];
    for instruction in instructions.chars() {
        instructions_vec.push(match instruction {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            '\n' => continue,
            x => panic!(),
        });
    }
    (map_vec, instructions_vec, coords)
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn get_move_vector(&self) -> Vector2<i32> {
        match self {
            Direction::North => Vector2::new(0, -1),
            Direction::East => Vector2::new(1, 0),
            Direction::South => Vector2::new(0, 1),
            Direction::West => Vector2::new(-1, 0),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum MapContent {
    Empty,
    Movable,
    Immovable,
}

fn test_move(location: Vector2<i32>, dir: Vector2<i32>, map: &Vec<Vec<MapContent>>) -> bool {
    match map[location.y as usize][location.x as usize] {
        MapContent::Empty => true,
        MapContent::Movable => test_move(location + dir, dir, map),
        MapContent::Immovable => false,
    }
}

fn do_move(
    location: Vector2<i32>,
    dir: Vector2<i32>,
    map: &mut Vec<Vec<MapContent>>,
    first: bool,
) -> bool {
    if first {
        if !test_move(location, dir, map) {
            return false;
        }
    }

    let loc = (location.x as usize, location.y as usize);
    match map[loc.1][loc.0] {
        MapContent::Empty => {
            if !first {
                map[loc.1][loc.0] = MapContent::Movable
            }
        }
        MapContent::Movable => {
            if first {
                map[loc.1][loc.0] = MapContent::Empty;
            }

            do_move(location + dir, dir, map, false);
        }
        _ => panic!(),
    }
    return true;
}

fn print_map(map: &Vec<Vec<MapContent>>, bot: &Vector2<i32>) {
    for (y, line) in map.iter().enumerate() {
        for (x, item) in line.iter().enumerate() {
            if bot.x == x.try_into().unwrap() && bot.y == y.try_into().unwrap() {
                print!("@");
                continue;
            }
            let to_print = match item {
                MapContent::Empty => '.',
                MapContent::Movable => 'O',
                MapContent::Immovable => '#',
            };
            print!("{to_print}");
        }
        println!();
    }
}

pub fn part_1(input: String) -> Solution {
    let (mut map, instructions, mut bot) = parse(input);
    for instruction in instructions {
        let move_vec: Vector2<_> = instruction.get_move_vector();
        let next_pos = bot + move_vec;
        let success = do_move(next_pos, move_vec, &mut map, true);
        if success {
            bot = next_pos;
        }
    }

    let mut sol = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, content) in row.iter().enumerate() {
            if *content == MapContent::Movable {
                sol += 100 * y + x;
            }
        }
    }
    Solution::from(sol)
}

pub fn part_2(_input: String) -> Solution {
    let sol = 0;
    Solution::from(sol)
}
