#[path = "common.rs"]
mod common;
use ndarray::{Array1, Array2};

/// ignore text and just return single or multi-digit numbers
fn parse_numbers(line: &str) -> Vec<isize> {
    line.chars()
        .filter(|c| c.is_digit(10) || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}


/// Claw Contraption
/// use matrix solve Ax = B
/// TODO: for part2 dispense with matrix math and use simultaneous equations; remember to add 10 trillion also
fn part(filename: &str, is_part1: bool) -> usize {
    let mut tokens = 0usize;
    // parse info
    let lines = common::read_lines(filename);
    let zeros = Array1::zeros(2);
    let mut aaa: Vec<isize> = Vec::new();
    for line in lines {
        // println!("{:?}", line);
        let numbers = parse_numbers(&line);
        if line.starts_with("Button") {
            aaa.extend(numbers);
        } else if line.starts_with("Prize") {
            // Compute tokens required to solve problem
            let coeffs: Array2<isize> = Array2::from_shape_vec((2, 2), aaa.clone()).expect("?");
            let result: Array1<isize> = Array1::from(numbers);

            // println!("{:?} {:?}", coeffs, result);
            // println!("start!");
            let mut best_cost = isize::MAX;
            let mut has_solution = false;
            for adx in 0..100 {
                for bdx in 0..100 {
                    let guess: Array1<isize> = Array1::from(vec![adx, bdx]);
                    if (result.clone() - guess.dot(&coeffs)) == zeros {
                        // good solution
                        let cost = adx * 3 + bdx;
                        if cost < best_cost {
                            best_cost = cost;
                            // println!("a={} b={} oK! ${}", adx, bdx, best_cost);
                        }
                        has_solution = true;
                    }
                    // let bla = ;
                    // if is_solution(coeffs, result.clone(), guess) {
                    //     println!("ok!")
                    // }
                }
            }
            if has_solution {
                tokens += best_cost as usize;
            }

            // reset for next puzzle
            aaa.clear();
            // break;
        }
    }
    return tokens;
}

pub fn solve() {
    let day: usize = 13;
    // Test part-1 solver, then apply to real input.
    assert_eq!(part(&format!("input/{:02}_train", day), true), 480);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    // assert_eq!(part(&format!("input/{:02}_train", day), false), 480);
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: 50+ Minutes"); //started 918pm
}
