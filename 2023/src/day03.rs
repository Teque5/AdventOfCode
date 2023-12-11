#[path = "common.rs"]
mod common;
use ndarray::Array2;
use std::cmp;

/// Gear Ratios
fn part1(filename: &str) -> usize {
    let mut acc = 0usize;
    let (ray, rows, cols) = common::read_2d_chars(filename);
    // find the part mask first
    let mut mask: Array2<bool> = Array2::from_elem((rows, cols), false);
    for rdx in 0..rows {
        for cdx in 0..cols {
            let foo = ray[(rdx, cdx)];
            if !(foo.is_digit(10) || foo == '.') {
                for subrdx in cmp::max(rdx - 1, 0)..cmp::min(rdx + 2, rows) {
                    for subcdx in cmp::max(cdx - 1, 0)..cmp::min(cdx + 2, cols) {
                        // above, at position, & below
                        mask[(subrdx, subcdx)] = true;
                    }
                }
            }
        }
    }
    // look for numbers overlapping the mask
    let mut keep = false;
    let mut temp: String = String::new();
    for rdx in 0..rows {
        for cdx in 0..cols {
            let foo = ray[(rdx, cdx)];

            if foo.is_digit(10) {
                temp.push(foo);
                if keep || mask[(rdx, cdx)] {
                    keep = true;
                }
            } else if !foo.is_digit(10) {
                if let Ok(parsed) = temp.parse::<usize>() {
                    // println!("{} {}", parsed, keep);
                    if keep {
                        acc += parsed;
                    }
                    // reset things
                    temp.clear();
                    keep = false;
                }
            }
        }
    }
    return acc;
}

/// from (rdx, cdx) position eat the number and return value
fn eat(ray: &Array2<char>, rdx: usize, cdx: usize) -> usize {
    // FIXME: This is not well written :(
    let mut value = 0usize;
    let mut temp: String = String::new();
    for col in cdx..ray.ncols() {
        let foo = ray[(rdx, col)];
        if foo.is_digit(10) {
            temp.push(foo);
            if col == ray.ncols() - 1 {
                if let Ok(parsed) = temp.parse::<usize>() {
                    value = parsed;
                    break;
                }
            }
        } else if !foo.is_digit(10) {
            if let Ok(parsed) = temp.parse::<usize>() {
                value = parsed;
                break;
            }
        }
    }
    return value;
}

/// Gear Ratios *
fn part2(filename: &str) -> usize {
    let mut acc = 0usize;
    let (ray, rows, cols) = common::read_2d_chars(filename);
    // find the part mask first
    for rdx in 0..rows {
        for cdx in 0..cols {
            if ray[(rdx, cdx)] == '*' {
                let mut adjacents: Vec<usize> = Vec::new();
                // find the start index (alpha) of adjacent numbers
                for subrdx in cmp::max(rdx - 1, 0)..cmp::min(rdx + 2, rows) {
                    // for each row, find the leftmost number position
                    let mut alpha = 99999;
                    let mut last_alpha = 99999;
                    // println!("\nrow {}", subrdx);
                    for subcdx in cmp::max(cdx - 1, 0)..cmp::min(cdx + 2, cols) {
                        // above, at position, & below
                        if ray[(subrdx, subcdx)].is_digit(10) {
                            // try to walk left
                            for leftwalk in (0..subcdx + 1).rev() {
                                if ray[(subrdx, leftwalk)].is_digit(10) {
                                    // println!("left {} {}", leftwalk)
                                    alpha = leftwalk;
                                } else {
                                    break;
                                }
                            }
                            if alpha != last_alpha {
                                // println!("{} walked {}", subrdx, eat(&ray, subrdx, alpha));
                                adjacents.push(eat(&ray, subrdx, alpha));
                                last_alpha = alpha;
                            }
                        }
                    }
                }
                if adjacents.len() == 2 {
                    let mut subtotal = 1usize;
                    // print!("gear ({:3} {:3}) >>> ", rdx, cdx);
                    for item in adjacents {
                        // print!("{:3} ", item);
                        subtotal *= item;
                    }
                    // println!("");
                    acc += subtotal;
                }
            }
        }
    }
    return acc;
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part1("input/03_train"),
        common::read_lines_as::<usize>("input/03_val1")[0]
    );
    println!("Part1: {}", part1("input/03_test"));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part2("input/03_train"),
        common::read_lines_as::<usize>("input/03_val2")[0]
    );
    println!("Part2: {}", part2("input/03_test"));
    println!("Coded: 84 minutes");
}
