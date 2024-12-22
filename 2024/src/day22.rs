use aoc;
use std::cmp::max;
use indicatif::{ProgressBar, ProgressStyle};

/// Monkey Psudorandom Sequence
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;
    let mut operand: usize;
    let mut prices: Vec<Vec<i8>> = Vec::new();
    let mut diffs: Vec<Vec<i8>> = Vec::new();

    // parse codes
    let initial_numbers = aoc::read_lines_as::<usize>(filename);

    // iterate over bananas & compute prices
    for idx in 0..initial_numbers.len() {
        let mut prior = 0i8;
        let mut number = initial_numbers[idx];
        let mut vendor: Vec<i8> = Vec::new();
        let mut diff: Vec<i8> = Vec::new();
        for jdx in 0..2000 {
            operand = number * 64;
            number = prune(mix(number, operand));
            operand = number / 32;
            number = prune(mix(number, operand));
            operand = number * 2048;
            number = prune(mix(number, operand));
            // store things
            let price = (number % 10) as i8;
            vendor.push(price);
            if jdx > 0 {
                diff.push(price - prior);
            }
            prior = price;
        }
        if is_part1 {
            acc += number;
        } else {
            prices.push(vendor);
            diffs.push(diff);
        }
    }
    if !is_part1 {
        acc = 0;
        let progress = ProgressBar::new((19 * 19 * 19 * 19) as u64);
        progress.set_style(ProgressStyle::with_template(aoc::STYLE).unwrap());
        progress.set_message("Counting Bananas...");
        for (cdx, combo) in generate_combinations().iter().enumerate() {
            let mut bananas = 0usize;
            for vdx in 0..initial_numbers.len() {
                if let Some(offset) = find_offset(&diffs[vdx], &combo) {
                    bananas += prices[vdx][offset+4] as usize;
                    // println!("{} {:?} {}", vdx, offset, prices[vdx][offset+4]);
                }
            }
            progress.set_position(cdx as u64);
            acc = max(acc, bananas);
        }
        let sequence: Vec<i8> = vec![-2,1,-1,3];
        // println!("price {:?}",prices);
        // println!("diffs {:?}",diffs);
    }
    return acc;
}

/// create all possible combinations for price changes
fn generate_combinations() -> Vec<Vec<i8>> {
    let mut combinations = Vec::new();
    for d1 in -9..=9 {
        for d2 in -9..=9 {
            for d3 in -9..=9 {
                for d4 in -9..=9 {
                    combinations.push(vec![d1, d2, d3, d4]);
                }
            }
        }
    }
    combinations
}

fn find_offset(haystack: &Vec<i8>, needle: &Vec<i8>) -> Option<usize> {
    haystack.windows(needle.len())
        .position(|window| window == needle.as_slice())
}

/// prune secret number
fn prune(secret: usize) -> usize {
    secret % 16777216
}

/// mix into secret number
fn mix(secret: usize, operator: usize) -> usize {
    secret ^ operator
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    // assert_eq!(part(&format!("input/{:02}_train0", day), true), 37327623);
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    assert_eq!(part(&format!("input/{:02}_train1", day), false), 23);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: 45+ Minutes");
}
