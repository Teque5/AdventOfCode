#[path = "common.rs"]
mod common;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

/// are these char vectors equal?
fn are_equal(alpha: &Vec<char>, omega: &Vec<char>) -> bool {
    for idx in 0..alpha.len() {
        if (alpha[idx] != omega[idx]) && ('?' != omega[idx]) {
            return false;
        }
    }
    return true;
}

/// determine how dots can be distributed between groups
fn generate_combinations(
    n_spots: usize,
    r_dots: usize,
    current_combination: &mut Vec<usize>,
    all_combinations: &mut Vec<Vec<usize>>,
    current_spot: usize,
) {
    if current_spot == r_dots {
        // If we have filled all spots and the sum is equal to n, add the current combination to the result
        if current_combination.iter().sum::<usize>() == n_spots {
            all_combinations.push(current_combination.clone());
        }
    } else {
        // Place the ball in the current spot and proceed to the next spot
        for dots_in_current_spot in 0..=n_spots {
            current_combination.push(dots_in_current_spot);
            generate_combinations(
                n_spots,
                r_dots,
                current_combination,
                all_combinations,
                current_spot + 1,
            );
            current_combination.pop();
        }
    }
}

// Broken Hot Springs
fn part(filename: &str, is_part1: bool) -> usize {
    let arrangements = AtomicUsize::new(0);
    // parse info
    let lines = common::read_lines(filename);
    lines.par_iter().for_each(|line| {
        let mut record_and_groups = line.split_whitespace();
        let mut record = record_and_groups
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>();
        let mut groups = common::split_str_as::<usize>(record_and_groups.next().unwrap(), ',');

        if !is_part1 {
            // 5x everything; insert extra ? between records
            let mut new_groups: Vec<usize> = Vec::new();
            let mut new_record: Vec<char> = Vec::new();
            for _ in 0..5 {
                new_groups.extend(groups.clone());
                new_record.extend(record.clone());
                new_record.push('?');
            }
            record = new_record;
            groups = new_groups;
            // println!("x {:?}", record.clone().into_iter().collect::<String>());
            // println!("x {:?}", groups);
        }

        // determine possible arrangements
        let min_len = groups.iter().sum::<usize>() + groups.len() - 1;

        // construct a possibility and then fuck with it
        let mut possible: Vec<char> = Vec::new();

        // figure out where blank space could be distributed
        let mut all_combinations = Vec::new();
        let mut current_combination = Vec::new();
        generate_combinations(
            record.len() - min_len,
            groups.len() + 1,
            &mut current_combination,
            &mut all_combinations,
            0,
        );
        println!("{} combos possible", all_combinations.len());
        for combo in all_combinations.iter() {
            possible.clear();

            if combo[0] != 0 {
                possible.extend(vec!['.'; combo[0]]);
            }

            for (cdx, count) in groups.iter().enumerate() {
                if cdx != 0 {
                    let mut dotcount = 1usize;
                    if combo[cdx] != 0 {
                        dotcount += combo[cdx];
                    }
                    possible.extend(vec!['.'; dotcount]);
                }
                possible.extend(vec!['#'; *count]);
            }
            if combo[groups.len()] != 0 {
                possible.extend(vec!['.'; combo[groups.len()]]);
            }
            let is_ok = are_equal(&possible, &record);

            if is_ok {
                arrangements.fetch_add(1, Ordering::Relaxed);
                println!(
                    "{:?} {:?} {}",
                    combo,
                    possible.clone().into_iter().collect::<String>(),
                    is_ok
                );
            }
        }
    });

    return arrangements.load(Ordering::Relaxed);
}

pub fn solve() {
    let day: usize = 12;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), false),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    // );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: 120+ minutes");
}
