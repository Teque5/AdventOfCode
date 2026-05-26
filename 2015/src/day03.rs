use aoc;
use std::collections::HashSet;

/// Perfectly Spherical Houses in a Vacuum
/// could be a more elegant
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 1usize;
    let mut santa = (0isize, 0isize);
    let mut robot = (0isize, 0isize);
    let mut visited = HashSet::<(isize, isize)>::new();
    visited.insert(santa);

    let map = &aoc::read_lines(filename)[0];

    if is_part1 {
        for direction in map.chars() {
            match direction {
                '^' => {
                    santa.1 -= 1;
                }
                '>' => {
                    santa.0 += 1;
                }
                'v' => {
                    santa.1 += 1;
                }
                '<' => {
                    santa.0 -= 1;
                }
                _ => {
                    panic!("invalid input")
                }
            }
            if !visited.contains(&santa) {
                acc += 1;
                visited.insert(santa);
            }
        }
    } else {
        // part 2
        for (ddx, direction) in map.chars().enumerate() {
            match direction {
                '^' => {
                    if ddx % 2 == 0 {
                        santa.1 -= 1;
                    } else {
                        robot.1 -= 1;
                    }
                }
                '>' => {
                    if ddx % 2 == 0 {
                        santa.0 += 1;
                    } else {
                        robot.0 += 1;
                    }
                }
                'v' => {
                    if ddx % 2 == 0 {
                        santa.1 += 1;
                    } else {
                        robot.1 += 1;
                    }
                }
                '<' => {
                    if ddx % 2 == 0 {
                        santa.0 -= 1;
                    } else {
                        robot.0 -= 1;
                    }
                }
                _ => {
                    panic!("invalid input")
                }
            }
            if !visited.contains(&santa) {
                acc += 1;
                visited.insert(santa);
            } else if !visited.contains(&robot) {
                acc += 1;
                visited.insert(robot);
            }
        }
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 4);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 3);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 60 minutes.");
}
