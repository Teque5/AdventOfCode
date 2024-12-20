/// Top-level dispatcher for Advent Of Code 2023 solutions
/// https://adventofcode.com/
/// To run a given day's solution: cargo run [day#]
/// To run all day's solutions: cargo run all

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::env;
use std::time;
mod fetch;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

// List of functions, one for each day's problem
lazy_static! {
    static ref SOLUTIONS: HashMap<usize, fn()> = vec![
        (1,  day01::solve as fn()),
        (2,  day02::solve as fn()),
        (3,  day03::solve as fn()),
        (4,  day04::solve as fn()),
        (5,  day05::solve as fn()),
        (6,  day06::solve as fn()),
        (7,  day07::solve as fn()),
        (8,  day08::solve as fn()),
        (9,  day09::solve as fn()),
        (10, day10::solve as fn()),
        (11, day11::solve as fn()),
        (12, day12::solve as fn()),
        (13, day13::solve as fn()),
        (14, day14::solve as fn()),
        (15, day15::solve as fn()),
        (16, day16::solve as fn()),
        (17, day17::solve as fn()),
        (18, day18::solve as fn()),
        (19, day19::solve as fn()),
        (20, day20::solve as fn()),
        // (21, day21::solve as fn()),
        // (22, day22::solve as fn()),
        // (23, day23::solve as fn()),
        // (24, day24::solve as fn()),
        // (25, day25::solve as fn()),
    ].into_iter().collect();

    static ref EMPTY_STRING: String = String::from("");
}

/// Wrapper that adds elapsed-time telemetry.
fn solve_timer(idx: &usize) -> usize {
    if let Some(solver) = SOLUTIONS.get(idx) {
        println!("Day {}", idx);
        let timer = time::Instant::now();
        solver(); // Run designated function
        let elapsed = timer.elapsed().as_micros();
        println!("Timed: {} µs", elapsed);
        println!("");
        elapsed as usize
    } else {
        eprintln!("No solution for Day {}.", idx);
        0
    }
}

/// Main entry point looks at command-line arguments.
fn main() {
    let year: usize = 2024;
    let args: Vec<String> = env::args().collect();
    let arg = args.last().unwrap_or(&EMPTY_STRING);

    if arg == "all" {
        // Solve every problem in the list.
        let mut elapsed = 0usize;
        let mut keys: Vec<&usize> = SOLUTIONS.keys().collect();
        keys.sort(); // Print in order...
        for idx in keys.iter() {
            fetch::get_data(year, **idx);
            elapsed += solve_timer(idx);
        }

        println!(
            "Advent of Code {} Runtime = {:.3} s",
            year,
            elapsed as f32 / 1e6
        );
    } else if let Result::Ok(idx) = arg.parse::<usize>() {
        fetch::get_data(year, idx);
        solve_timer(&idx);
    } else {
        eprintln!("Usage: 'cargo run [day#]' or 'cargo run all'");
        eprintln!("  Where day# is the problem to be solved (1-25)");
    }
}
