use average::Variance;
use nalgebra::{SVector, Vector2};

use crate::Solution;

const MAP_SIZE: SVector<isize, 2> = Vector2::new(101, 103);

#[derive(Debug)]
pub struct Robot {
    pos: SVector<isize, 2>,
    dir: SVector<isize, 2>
}

impl Robot {
    pub fn move_bot(&mut self) {
        self.pos += self.dir;
        self.pos[0] %= MAP_SIZE[0];
        self.pos[1] %= MAP_SIZE[1];
        while self.pos[0] < 0 {
            self.pos[0] += MAP_SIZE[0]
        }
        while self.pos[1] < 0 {
            self.pos[1] += MAP_SIZE[1]
        }
    }
}

fn parse (input: String) -> Vec<Robot> {
    let mut robots = vec![];
    for line in input.lines(){
        let parts: Vec<Vec<isize>> = line.split_whitespace().map(|vec| vec.split("=").last().unwrap().split(",").map(|num|num.parse::<isize>().unwrap()).collect()).collect();
        robots.push(Robot{
            pos:Vector2::new(parts[0][0], parts[0][1]),
            dir: Vector2::new(parts[1][0], parts[1][1])
        });
    }
    robots
}

pub fn part_1(input: String) -> Solution {
    let mut input = parse(input);
    for _ in 0..100 {
        for robot in  &mut input {
            robot.move_bot();
        }
    }
    let mut quardrants = [0;4];
    for robot in input {
        if robot.pos[0] < MAP_SIZE[0]/2 && robot.pos[1] < MAP_SIZE[1]/2 {
            quardrants[0] +=1;
        } else if robot.pos[0] < MAP_SIZE[0]/2 && robot.pos[1] > MAP_SIZE[1]/2  {

            quardrants[1] +=1;
        } else if robot.pos[0] > MAP_SIZE[0]/2 && robot.pos[1] < MAP_SIZE[1]/2 {
            quardrants[2] +=1;
        } else if robot.pos[0] > MAP_SIZE[0]/2 && robot.pos[1] > MAP_SIZE[1]/2  {
            quardrants[3] +=1;

        } else {
        }
    }
    let sol = quardrants.into_iter().reduce(|acc, e| acc * e).unwrap();
    Solution::from(sol)
}

pub fn part_2(input: String) -> Solution {
    let mut input = parse(input);
    for i  in 0..usize::MAX {
        let x_var = input.iter().map(|robot|robot.pos[0] as f64).collect::<Variance>().sample_variance();
        let y_var = input.iter().map(|robot|robot.pos[1] as f64).collect::<Variance>().sample_variance();

        if(x_var< 700.0 && y_var < 700.0) {
            return Solution::from(i)
        }
        for robot in  &mut input {
            robot.move_bot();
        }
    }
    panic!("not found")
}
