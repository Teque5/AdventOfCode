use aoc;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressStyle;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

// /// given some positions and some operators, return all possible combinations
// fn combinations(num_positions: usize, num_operators: usize) -> Vec<Vec<usize>> {
//     let mut combinations: Vec<Vec<usize>> = Vec::new();
//     for idx in 0..(num_operators.pow(num_positions as u32)) {
//         let mut combination = Vec::new();
//         for jdx in 0..num_positions {
//             let value = (idx / num_operators.pow(jdx as u32)) % num_operators;
//             combination.push(value);
//         }
//         combinations.push(combination);
//     }
//     return combinations;
// }

struct Combinations {
    num_positions: usize,
    num_operators: usize,
    current: usize,            // Tracks the current index in the combinations space
    total_combinations: usize, // Total number of combinations
}

impl Combinations {
    fn new(num_positions: usize, num_operators: usize) -> Self {
        let total_combinations = num_operators.pow(num_positions as u32);
        Combinations {
            num_positions,
            num_operators,
            current: 0,
            total_combinations,
        }
    }
}
/// given some positions and some operators, return all possible combinations
impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.total_combinations {
            return None; // No more combinations
        }

        let mut combination = Vec::new();

        for jdx in 0..self.num_positions {
            let value = (self.current / self.num_operators.pow(jdx as u32)) % self.num_operators;
            combination.push(value);
        }

        self.current += 1;
        Some(combination)
    }
}

/// Bridge Repair
/// chain possible operators and check if matches expected result
fn part(filename: &str, is_part1: bool) -> usize {
    let acc = AtomicUsize::new(0);
    // parse info
    let lines = aoc::read_lines(filename);
    let style = ProgressStyle::with_template(aoc::STYLE).unwrap();
    lines
        .par_iter()
        .progress_with_style(style)
        .for_each(|line| {
            let result = aoc::parse_delimited::<usize>(&line, ':')[0];
            let numbers = aoc::parse_delimited::<usize>(&line, ' ');
            let num_operators = if is_part1 { 2 } else { 3 };
            let combos = Combinations::new(numbers.len() - 1, num_operators);
            // let combos = combinations(numbers.len() - 1, num_operators);
            for combo in combos {
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
                    acc.fetch_add(result, Ordering::Relaxed);
                    break;
                }
            }
        });
    return acc.load(Ordering::Relaxed);
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 3749);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 11387);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 63 Minutes");
}
