/// Top-level dispatcher for Advent Of Code 2024 solutions
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
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

// List of functions, one for each day's problem
lazy_static! {
    static ref SOLUTIONS: HashMap<usize, fn(usize)> = vec![
        (1, day01::solve as _),
        (2, day02::solve as _),
        (3, day03::solve as _),
        (4, day04::solve as _),
        (5, day05::solve as _),
        (6, day06::solve as _),
        (7, day07::solve as _),
        (8, day08::solve as _),
        (9, day09::solve as _),
        (10, day10::solve as _),
        (11, day11::solve as _),
        (12, day12::solve as _),
        (13, day13::solve as _),
        // (14, day14::solve as _),
        // (15, day15::solve as _),
        // (16, day16::solve as _),
        // (17, day17::solve as _),
        // (18, day18::solve as _),
        // (19, day19::solve as _),
        // (20, day20::solve as _),
        // (21, day21::solve as _),
        // (22, day22::solve as _),
        // (23, day23::solve as _),
        // (24, day24::solve as _),
        // (25, day25::solve as _),
    ].into_iter().collect();

    static ref EMPTY_STRING: String = String::from("");
}

/// Wrapper that adds elapsed-time telemetry.
fn solve_timer(day: &usize) -> usize {
    if let Some(solver) = SOLUTIONS.get(day) {
        println!("Day {}", day);
        let timer = time::Instant::now();
        solver(*day); // Run designated function
        let elapsed = timer.elapsed().as_micros();
        println!("Timed: {} µs", elapsed);
        println!("");
        elapsed as usize
    } else {
        eprintln!("No solution for Day {}.", day);
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
        for day in keys.iter() {
            fetch::get_data(&year, &day);
            elapsed += solve_timer(&day);
        }

        println!(
            "Advent of Code {} Runtime = {:.3} s",
            year,
            elapsed as f32 / 1e6
        );
    } else if let Result::Ok(day) = arg.parse::<usize>() {
        fetch::get_data(&year, &day);
        solve_timer(&day);
    } else {
        eprintln!("Usage: 'cargo run [day#]' or 'cargo run all'");
        eprintln!("  Where day# is the problem to be solved (1-25)");
    }
}
