use nalgebra::{Matrix2, Matrix2x1, MatrixXx2, SVector, Vector2};
use regex::Regex;

use crate::Solution;

pub struct Button {}

pub struct Machine {
    a: SVector<f64, 2>,
    b: SVector<f64, 2>,
    prize: SVector<f64, 2>,
}

fn parse(input: String) -> Vec<Machine> {
    let machines_string: Vec<_> = input.split("\n\n").collect();
    let regex = Regex::new(r"X[+=](\d*), Y[+=](\d*)").unwrap();
    let mut machines = vec![];

    for machine in machines_string {
        let captures = regex.captures_iter(machine);
        let mut machine_parts = vec![];
        for capture in captures {
            machine_parts.push(Vector2::new(
                capture.get(1).unwrap().as_str().parse().unwrap(),
                capture.get(2).unwrap().as_str().parse().unwrap(),
            ))
        }
        machines.push(Machine {
            a: machine_parts[0],
            b: machine_parts[1],
            prize: machine_parts[2],
        });
    }
    machines
}

pub fn part_1(input: String) -> Solution {
    let input = parse(input);
    let mut sol = 0;

    for machine in input {
        let mtx = Matrix2::new(machine.a.x, machine.b.x, machine.a.y, machine.b.y);
        let inv = mtx.try_inverse().unwrap();
        let prize_mtx = Matrix2x1::new(machine.prize.x, machine.prize.y);
        let ans = inv * prize_mtx;
        let a_presses = ans.get(0).unwrap();
        let b_presses = ans.get(1).unwrap();

        if (a_presses.round() - a_presses).abs() > 0.0000001
            || (b_presses.round() - b_presses).abs() > 0.0000001
        {
            continue;
        }
        sol += (3.0 * a_presses.round() + b_presses.round()) as usize;
    }
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let input = parse(input);
    let mut sol = 0;

    for machine in input {
        let mtx = Matrix2::new(machine.a.x, machine.b.x, machine.a.y, machine.b.y);
        let inv = mtx.try_inverse().unwrap();
        let prize_mtx = Matrix2x1::new(
            machine.prize.x + 10000000000000.0,
            machine.prize.y + 10000000000000.0,
        );
        let ans = inv * prize_mtx;
        let a_presses = ans.get(0).unwrap();
        let b_presses = ans.get(1).unwrap();

        if (a_presses.round() - a_presses).abs() > 0.0001
            || (b_presses.round() - b_presses).abs() > 0.0001
        {
            continue;
        }
        sol += (3.0 * a_presses.round() + b_presses.round()) as usize;
    }
    Solution::from(sol)
}
