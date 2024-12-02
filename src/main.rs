mod days;
pub mod solution;

use days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};
use solution::Solution;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    let days: Vec<u8> = args[1..]
        .iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();

    let mut runtime = 0.0;

    for day in days {
        let p1 = get_p1_solver(day);
        let p2 = get_p2_solver(day);

        let file_input = std::fs::read_to_string(format!("input/day{day}"))
            .expect("you havent given me an input file silly");
        let start = Instant::now();
        let p1 = p1(file_input.clone());
        let mid = Instant::now();
        let p2 = p2(file_input.clone());
        let end = Instant::now();
        let p1_dur = mid.duration_since(start).as_nanos() as f64 / 1_000_000.0;
        let p2_dur = end.duration_since(mid).as_nanos() as f64 / 1_000_000.0;
        let tot_dur = end.duration_since(start).as_nanos() as f64 / 1_000_000.0;

        println!("\n=== Day {:02} ===", day);
        println!("  · Part 1: {}", p1);
        println!("  · Part 2: {}", p2);
        println!("\n  · Part 1 Elapsed: {:.4} ms", p1_dur);
        println!("  · Part 2 Elapsed: {:.4} ms", p2_dur);
        println!("  · Total Elapsed: {:.4} ms", tot_dur);

        runtime += tot_dur;
    }
    println!("Total runtime: {:.4} ms", runtime);
}

fn get_p1_solver(day: u8) -> fn(String) -> Solution {
    match day {
        1 => day01::part_1,
        2 => day02::part_1,
        3 => day03::part_1,
        4 => day04::part_1,
        5 => day05::part_1,
        6 => day06::part_1,
        7 => day07::part_1,
        8 => day08::part_1,
        9 => day09::part_1,
        10 => day10::part_1,
        11 => day11::part_1,
        12 => day12::part_1,
        13 => day13::part_1,
        14 => day14::part_1,
        15 => day15::part_1,
        16 => day16::part_1,
        17 => day17::part_1,
        18 => day18::part_1,
        19 => day19::part_1,
        20 => day20::part_1,
        21 => day21::part_1,
        22 => day22::part_1,
        23 => day23::part_1,
        24 => day24::part_1,
        25 => day25::part_1,
        _ => unimplemented!(),
    }
}
fn get_p2_solver(day: u8) -> fn(String) -> Solution {
    match day {
        1 => day01::part_2,
        2 => day02::part_2,
        3 => day03::part_2,
        4 => day04::part_2,
        5 => day05::part_2,
        6 => day06::part_2,
        7 => day07::part_2,
        8 => day08::part_2,
        9 => day09::part_2,
        10 => day10::part_2,
        11 => day11::part_2,
        12 => day12::part_2,
        13 => day13::part_2,
        14 => day14::part_2,
        15 => day15::part_2,
        16 => day16::part_2,
        17 => day17::part_2,
        18 => day18::part_2,
        19 => day19::part_2,
        20 => day20::part_2,
        21 => day21::part_2,
        22 => day22::part_2,
        23 => day23::part_2,
        24 => day24::part_2,
        25 => day25::part_2,
        _ => unimplemented!(),
    }
}
