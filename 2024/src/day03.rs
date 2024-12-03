#[path = "common.rs"]
mod common;

use regex::Regex;

/// Mull It Over (corrupted memory parsing)
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;

    let pattern = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
    // parse info
    let lines = common::read_lines(filename);
    let mut enable = true;
    for line in lines {
        // println!("line: {}", line);
        let mut results = vec![];

        for mat in pattern.captures_iter(&line) {
            let idx = mat.get(0).unwrap().start();
            let x = mat[1].parse::<usize>().unwrap();
            let y = mat[2].parse::<usize>().unwrap();
            results.push((idx, x, y))
        }
        if is_part1 {
            for (_, x, y) in results.iter() {
                acc += x * y;
            }
        } else {
            // additional search for do() and don't()
            let pattern_do = Regex::new(r"do\(\)").unwrap();
            let pattern_dont = Regex::new(r"don[\'’]t\(\)").unwrap();
            for mat in pattern_do.find_iter(&line) {
                let idx = mat.start();
                results.push((idx, 1_000_000, 1_000_000));
            }
            for mat in pattern_dont.find_iter(&line) {
                let idx = mat.start();
                results.push((idx, 2_000_000, 2_000_000));
            }
            // luckily sort sorts by the first value!
            results.sort();
            for (_, x, y) in results.iter() {
                if *x == 1_000_000 {
                    enable = true;
                    continue;
                } else if *x == 2_000_000 {
                    enable = false;
                    continue;
                }
                if enable {
                    acc += x * y;
                    // println!("{} {} {} {}", idx, x, y, enable);
                }
            }
        }
    }
    return acc;
}

pub fn solve() {
    let day: usize = 3;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train1", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train2", day), false),
        common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 55 Minutes");
}
