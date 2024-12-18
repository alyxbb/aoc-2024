use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::Solution;

fn parse(input: String) -> Computer {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| line.contains(": "))
        .map(|line| line.split_once(": ").unwrap().1)
        .collect();
    let reg_a = lines[0].parse().unwrap();
    let reg_b = lines[1].parse().unwrap();
    let reg_c = lines[2].parse().unwrap();

    let instructions = lines[3]
        .split(",")
        .map(|val| val.parse::<u8>().unwrap().try_into().unwrap())
        .collect();
    Computer {
        pc: 0,
        reg_a,
        reg_b,
        reg_c,
        instructions,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Clone)]
struct Computer {
    pc: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instructions: Vec<Opcode>,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(()),
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::Adv => 0,
            Opcode::Bxl => 1,
            Opcode::Bst => 2,
            Opcode::Jnz => 3,
            Opcode::Bxc => 4,
            Opcode::Out => 5,
            Opcode::Bdv => 6,
            Opcode::Cdv => 7,
        }
    }
}

impl Computer {
    pub fn run(&mut self) -> Vec<usize> {
        let mut output = vec![];
        while self.pc < self.instructions.len() {
            // println!("pc: {} regs:{} {} {}",self.pc, self.reg_a, self.reg_b, self.reg_c);
            let opcode = self.instructions[self.pc];
            let operand: u8 = self.instructions[self.pc + 1].into();
            // println!("{:?} {}",opcode, operand);
            self.pc += 2;

            match opcode {
                Opcode::Adv => {
                    self.reg_a = self.reg_a
                        / (2_usize.pow(self.get_combo_operand(operand).try_into().unwrap()))
                }
                Opcode::Bxl => self.reg_b = self.reg_b ^ operand as usize,
                Opcode::Bst => self.reg_b = self.get_combo_operand(operand) % 8,
                Opcode::Jnz => {
                    if self.reg_a != 0 {
                        self.pc = operand as usize
                    }
                }
                Opcode::Bxc => self.reg_b = self.reg_b ^ self.reg_c,
                Opcode::Out => output.push(self.get_combo_operand(operand) % 8),
                Opcode::Bdv => {
                    self.reg_b = self.reg_a
                        / (2_usize.pow(self.get_combo_operand(operand).try_into().unwrap()))
                }
                Opcode::Cdv => {
                    self.reg_c = self.reg_a
                        / (2_usize.pow(self.get_combo_operand(operand).try_into().unwrap()))
                }
            }
        }
        output
    }

    fn get_combo_operand(&self, operand: u8) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!(),
        }
    }
}

pub fn part_1(input: String) -> Solution {
    let mut computer = parse(input);
    let output = computer.run();
    let sol = output
        .into_iter()
        .map(|val| val.to_string())
        .collect::<Vec<String>>()
        .join(",");
    Solution::from(sol)
}

fn parse2(input: String) -> Vec<u64> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| line.contains(": "))
        .map(|line| line.split_once(": ").unwrap().1)
        .collect();

    let instructions = lines[3]
        .split(",")
        .map(|val| val.parse::<u8>().unwrap().try_into().unwrap())
        .collect();
    instructions
}
fn test_validity(num: u64, goal: &[u64]) -> Vec<u64> {
    let mut res = vec![];
    for i in 0..8 {
        let a = (num >> 3) + (i << 7);
        let mut b = a % 8;
        b = b ^ 1;
        let c = a / 2_u64.pow(b.try_into().unwrap());
        b = b ^ c;
        b = b ^ 4;
        if b % 8 == goal[0] {
            if goal.len() == 1 {
                res.push(a);
            } else {
                let valid = test_validity(a, &goal[1..]);
                res.extend(
                    valid
                        .iter()
                        .map(|val| (val << 3) + (a % 8))
                        .collect::<Vec<u64>>(),
                );
            }
        }
    }
    res
}

pub fn part_2(input: String) -> Solution {
    let goal = parse2(input);
    let mut valid_sols = vec![];
    for i in 0..(2 << 9) {
        let mut b: u64 = i % 8;
        b = b ^ 1;
        let c = i / (2_u64.pow(b.try_into().unwrap()));
        b = b ^ c;
        b = b ^ 4;
        if b % 8 == goal[0] {
            valid_sols.push(i);
        }
    }
    let mut res = vec![];
    for num in valid_sols {
        res.extend(
            test_validity(num, &goal[1..])
                .iter()
                .map(|val| (val << 3) + (num % 8))
                .collect::<Vec<u64>>(),
        );
    }
    Solution::from(*res.iter().min().unwrap())
}
