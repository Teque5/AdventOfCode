#[path = "common.rs"]
mod common;
use indicatif::{ProgressBar, ProgressStyle};

/// given some positions and some operators, return all possible combinations
fn combinations(num_positions: usize, num_operators: usize) -> Vec<Vec<usize>> {
    let mut combinations: Vec<Vec<usize>> = Vec::new();
    for idx in 0..(num_operators.pow(num_positions as u32)) {
        let mut combination = Vec::new();
        for jdx in 0..num_positions {
            let value = (idx / num_operators.pow(jdx as u32)) % num_operators;
            combination.push(value);
        }
        combinations.push(combination);
    }
    return combinations;
}

/// Bridge Repair
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0;
    // parse info
    let lines = common::read_lines(filename);
    // fancy progress bar
    let progress = ProgressBar::new(lines.len() as u64)
        .with_style(ProgressStyle::with_template(common::STYLE).unwrap())
        .with_message("Computing Combinations");
    for line in lines {
        let result = common::parse_delimited::<usize>(&line, ':')[0];
        let numbers = common::parse_delimited::<usize>(&line, ' ');
        let num_operators = if is_part1 { 2 } else { 3 };
        let combos = combinations(numbers.len() - 1, num_operators);
        for combo in combos.iter() {
            let mut buffer: usize = numbers[0];
            // let mut
            for ndx in 1..(numbers.len()) {
                match combo[ndx - 1] {
                    0 => buffer += numbers[ndx], // add
                    1 => buffer *= numbers[ndx], // multiply
                    2 => {
                        // concatenate
                        let num_digits = numbers[ndx].to_string().len();
                        buffer = buffer * 10_usize.pow(num_digits as u32) + numbers[ndx];
                    }
                    _ => {}
                }
                if buffer > result {
                    // cannot be correct
                    break;
                }
            }
            if buffer == result {
                // this is what we want
                acc += result;
                break;
            }
        }
        progress.inc(1)
    }
    return acc;
}

pub fn solve() {
    let day: usize = 7;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // // Test part-2 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 63 Minutes");
}
