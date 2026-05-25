use aoc;
use std::cmp;

/// I Was Told There Would Be No Math
/// calculate required wrapping paper
fn part(filename: &str, is_part1: bool) -> isize {
    let mut acc = 0isize;

    // parse info
    let lines = aoc::read_lines(filename);
    for line in lines {
        let dims = aoc::parse_numbers::<isize>(&line);
        let (length, width, height) = (dims[0], dims[1], dims[2]);
        if is_part1 {
            // calculate paper area
            acc += 2 * length * width + 2 * width * height + 2 * height * length;
            // calculate sides for slack
            let s0 = length * width;
            let s1 = width * height;
            let s2 = height * length;
            let mut slack = s0;
            if s1 < slack {
                slack = s1;
            }
            if s2 < slack {
                slack = s2;
            }
            acc += slack;
        } else {
            // calculate ribbon
            let mut short = length + width + height - cmp::max(length, cmp::max(width, height));
            // length is double shortest two sides plus volume for bow
            acc += short * 2 + length * width * height;
        }
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 58);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 34);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 17 minutes.");
}
