use aoc;
use std::collections::HashSet;

fn step(pos: &mut (isize, isize), direction: char) {
    match direction {
        '^' => pos.1 -= 1,
        '>' => pos.0 += 1,
        'v' => pos.1 += 1,
        '<' => pos.0 -= 1,
        _ => panic!("invalid input"),
    }
}

/// Perfectly Spherical Houses in a Vacuum
fn part(filename: &str, is_part1: bool) -> usize {
    let mut santa = (0isize, 0isize);
    let mut robot = (0isize, 0isize);
    let mut visited = HashSet::<(isize, isize)>::new();
    visited.insert(santa);

    let map = &aoc::read_lines(filename)[0];

    for (ddx, direction) in map.chars().enumerate() {
        let mover = if is_part1 || ddx % 2 == 0 { &mut santa } else { &mut robot };
        step(mover, direction);
        visited.insert(*mover);
    }
    visited.len()
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 4);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 3);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 60 minutes.");
}
