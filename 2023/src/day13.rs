#[path = "common.rs"]
mod common;
use ndarray::{Array2, Dim};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Lava Mirrors
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;

    // parse blocks
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();

    let mut current_block: Vec<Vec<char>> = Vec::new();
    for line_result in lines {
        let line = line_result.expect("line is fucked");
        // println!("line {:?}", line);
        if !line.trim().is_empty() {
            current_block.push(line.chars().collect());
        } else {
            // process this block
            acc += process_block(&current_block, is_part1);
            // reset block
            current_block.clear();
        }
    }
    // process remainder block
    acc += process_block(&current_block, is_part1);

    // 20779 is too low
    return acc;
}

/// count differences between rows
fn count_row_diff(ray: &Array2<char>, rdx1: usize, rdx2: usize) -> usize {
    // Get the rows using row indices
    let row1 = ray.row(rdx1);
    let row2 = ray.row(rdx2);

    // Zip the rows and count the number of different values
    let count = row1
        .iter()
        .zip(row2.iter())
        .filter(|&(val1, val2)| val1 != val2)
        .count();

    return count;
}

/// count differences between columns
fn count_col_diff(ray: &Array2<char>, cdx1: usize, cdx2: usize) -> usize {
    // Get the rows using row indices
    let col1 = ray.column(cdx1);
    let col2 = ray.column(cdx2);

    // Zip the rows and count the number of different values
    let count = col1
        .iter()
        .zip(col2.iter())
        .filter(|&(val1, val2)| val1 != val2)
        .count();

    return count;
}

/// process a single pattern
#[allow(unreachable_code)]
fn process_block(pattern: &Vec<Vec<char>>, is_part1: bool) -> usize {
    // read into Array2
    let rows = pattern.len();
    let cols = pattern[0].len();

    let ray: Array2<char> = Array2::from_shape_vec(
        Dim([rows, cols]),
        pattern.clone().into_iter().flatten().collect(),
    )
    .unwrap();

    // println!("");
    // common::print_2d_chars(&ray);

    // process block
    for rdx in 1..rows {
        let mut smudge_count = 0usize;
        // let mut reflection_found = true;
        for mdx in 0..rdx {
            // check during upper half
            if rdx + mdx >= rows {
                continue;
            }
            if is_part1 {
                if ray.row(rdx + mdx) != ray.row(rdx - mdx - 1) {
                    // mirror breaks
                    smudge_count = 7;
                    break;
                }
            } else {
                smudge_count += count_row_diff(&ray, rdx + mdx, rdx - mdx - 1);
                if smudge_count > 1 {
                    break;
                }
            }
        }
        if !is_part1 && smudge_count == 1 {
            // part2: mirrors have 1 error
            return 100 * rdx;
        }
        if is_part1 && smudge_count == 0 {
            // part1: mirrors have 0 error
            return 100 * rdx;
        }
    }
    for cdx in 1..cols {
        let mut smudge_count = 0usize;
        // let mut reflection_found = true;
        for mdx in 0..cdx {
            // check during upper half
            if cdx + mdx >= cols {
                continue;
            }
            if is_part1 {
                if ray.column(cdx + mdx) != ray.column(cdx - mdx - 1) {
                    // mirror breaks
                    smudge_count = 7;
                    break;
                }
            } else {
                smudge_count += count_col_diff(&ray, cdx + mdx, cdx - mdx - 1);
                if smudge_count > 1 {
                    break;
                }
            }
        }
        if !is_part1 && smudge_count == 1 {
            // part2: mirrors have 1 error
            return cdx;
        }
        if is_part1 && smudge_count == 0 {
            // part1: mirrors have 0 error
            return cdx;
        }
    }

    // program should never get here
    panic!("this cannot happen");
    return 0;
}

pub fn solve() {
    let day: usize = 13;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 88 minutes");
}
