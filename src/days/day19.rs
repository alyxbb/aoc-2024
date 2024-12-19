use cached::proc_macro::cached;

use crate::Solution;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
    Black,
    White,
}

impl TryFrom<char> for Color {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'r' => Ok(Color::Red),
            'g' => Ok(Color::Green),
            'u' => Ok(Color::Blue),
            'b' => Ok(Color::Black),
            'w' => Ok(Color::White),
            _ => Err(()),
        }
    }
}

fn parse(input: String) -> (Vec<Vec<Color>>, Vec<Vec<Color>>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let towels = parts[0]
        .split(", ")
        .map(|towel| towel.chars().map(|char| char.try_into().unwrap()).collect())
        .collect();
    let designs = parts[1]
        .lines()
        .map(|line| line.chars().map(|char| char.try_into().unwrap()).collect())
        .collect();
    (towels, designs)
}

#[cached]
fn check_design_possible(design: Vec<Color>, towels: Vec<Vec<Color>>) -> bool {
    if design.is_empty() {
        return true;
    }

    for towel in &towels {
        if design.starts_with(towel) {
            let possible = check_design_possible(design[towel.len()..].to_vec(), towels.clone());
            if possible {
                return true;
            }
        }
    }
    false
}

#[cached]
fn count_design_possible(design: Vec<Color>, towels: Vec<Vec<Color>>) -> usize {
    if design.is_empty() {
        return 1;
    }
    let mut tot = 0;

    for towel in &towels {
        if design.starts_with(towel) {
            tot += count_design_possible(design[towel.len()..].to_vec(), towels.clone());
        }
    }
    tot
}

pub fn part_1(input: String) -> Solution {
    let (towels, designs) = parse(input);
    let mut sol = 0;

    for design in designs {
        let possible = check_design_possible(design, towels.clone());
        if possible {
            sol += 1;
        }
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let (towels, designs) = parse(input);
    let mut sol = 0;

    for design in designs {
        sol += count_design_possible(design, towels.clone());
    }
    Solution::from(sol)
}
