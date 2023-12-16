#[path = "common.rs"]
mod common;
use ndarray::Array2;
use std::collections::HashMap;

/// recursively spawn the next step of the beam
fn follow_beam(
    grid: &Array2<char>,
    energized: &mut Array2<bool>,
    cache: &mut HashMap<(usize, usize, char), ()>,
    rows: usize,
    cols: usize,
    laser_row: usize,
    laser_col: usize,
    laser_dir: char,
) {
    // note these max values are due to possible overflow
    if laser_row == usize::MAX || laser_row == rows {
        // out of bounds left/right, laser ends
        return;
    }
    if laser_col == usize::MAX || laser_col == cols {
        // out of bounds top/bottom, laser ends
        return;
    }
    if cache.contains_key(&(laser_row, laser_col, laser_dir)) {
        // we've already done this one
        return;
    } else {
        cache.insert((laser_row, laser_col, laser_dir), ());
    }
    // energize this position
    energized[(laser_row, laser_col)] = true;

    let grid_char = grid[(laser_row, laser_col)];
    // determine how the laser moves next
    match laser_dir {
        '🡲' | '🡰' => {
            match grid_char {
                '.' | '-' => {
                    // empty
                    let next_col = if laser_dir=='🡲' {laser_col+1} else {laser_col.wrapping_sub(1)};
                    follow_beam(&grid, energized, cache, rows, cols, laser_row, next_col, laser_dir);
                }
                '/' | '\\' => {
                    // go up/down
                    match (laser_dir, grid_char) {
                        ('🡰', '\\') | ('🡲', '/') => {
                            // go up
                            follow_beam(&grid, energized, cache, rows, cols, laser_row.wrapping_sub(1), laser_col, '🡱');
                        }
                        ('🡰', '/') | ('🡲', '\\') => {
                            // go down
                            follow_beam(&grid, energized, cache, rows, cols, laser_row+1, laser_col, '🡳')
                        }
                        _ => panic!("no!"),
                    }
                }
                '|' => {
                    //split
                    follow_beam(&grid, energized, cache, rows, cols, laser_row.wrapping_sub(1), laser_col, '🡱');
                    follow_beam(&grid, energized, cache, rows, cols, laser_row + 1, laser_col, '🡳');
                }
                _ => panic!("no!"),
            }
        }
        '🡱' | '🡳' => {
            match grid_char {
                '.' | '|' => {
                    // empty
                    let next_row = if laser_dir=='🡳' {laser_row+1} else {laser_row.wrapping_sub(1)};
                    follow_beam(&grid, energized, cache, rows, cols, next_row, laser_col, laser_dir);
                }
                '/' | '\\' => {
                    // go left/right
                    match (laser_dir, grid_char) {
                        ('🡱', '/') | ('🡳', '\\') => {
                            // go right
                            follow_beam(&grid, energized, cache, rows, cols, laser_row, laser_col+1, '🡲');
                        }
                        ('🡱', '\\') | ('🡳', '/') => {
                            // go left
                            follow_beam(&grid, energized, cache, rows, cols, laser_row, laser_col.wrapping_sub(1), '🡰')
                        }
                        _ => panic!("no!"),
                    }
                }
                '-' => {
                    //split
                    follow_beam(&grid, energized, cache, rows, cols, laser_row, laser_col.wrapping_sub(1), '🡰');
                    follow_beam(&grid, energized, cache, rows, cols, laser_row, laser_col + 1, '🡲');
                }
                _ => panic!("no!"),
            }
        }
        _ => panic!("no!"),
    }
}

/// before each run, need to reset the energizer map
fn reset(
    energized: &mut Array2<bool>,
    cache: &mut HashMap<(usize, usize, char), ()>,
    rows: usize,
    cols: usize,
) {
    cache.clear();
    *energized = Array2::from_elem((rows, cols), false);
}

/// Beam Splitter
fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let (grid, rows, cols) = common::read_2d_chars(filename);
    let mut energized: Array2<bool> = Array2::from_elem((rows, cols), false);
    let mut cache: HashMap<(usize, usize, char), ()> = HashMap::new();
    if is_part1 {
        follow_beam(&grid, &mut energized, &mut cache, rows, cols, 0, 0, '🡲');
        // for row in energized.axis_iter(Axis(0)) {
        //     let some_string: String = row.iter().map(|&b| if b {'#'} else {'.'}).collect();
        //     println!("{:}", some_string);
        // }
        // count true values in energized
        let energized_spaces = energized.iter().filter(|&&x| x).count();
        return energized_spaces;
    }
    // part2 starts here
    let mut best = (0usize, 0usize, 0usize);
    let mut energized: Array2<bool> = Array2::from_elem((rows, cols), false);

    // top row
    for cdx in 0..cols {
        reset(&mut energized, &mut cache, rows, cols);
        follow_beam(&grid, &mut energized, &mut cache, rows, cols, 0, cdx, '🡳');
        let energized_spaces = energized.iter().filter(|&&x| x).count();
        if energized_spaces > best.0 {
            best = (energized_spaces, 0, cdx);
        }
    }
    // bottom row
    for cdx in 0..cols {
        reset(&mut energized, &mut cache, rows, cols);
        follow_beam(&grid, &mut energized, &mut cache, rows, cols, rows-1, cdx, '🡱');
        let energized_spaces = energized.iter().filter(|&&x| x).count();
        if energized_spaces > best.0 {
            best = (energized_spaces, rows - 1, cdx);
        }
    }
    // left col
    for rdx in 0..rows {
        reset(&mut energized, &mut cache, rows, cols);
        follow_beam(&grid, &mut energized, &mut cache, rows, cols, rdx, 0, '🡲');
        let energized_spaces = energized.iter().filter(|&&x| x).count();
        if energized_spaces > best.0 {
            best = (energized_spaces, rdx, 0);
        }
    }
    // right col
    for rdx in 0..rows {
        reset(&mut energized, &mut cache, rows, cols);
        follow_beam(&grid, &mut energized, &mut cache, rows, cols, rdx, cols-1, '🡰');
        let energized_spaces = energized.iter().filter(|&&x| x).count();
        if energized_spaces > best.0 {
            best = (energized_spaces, rdx, cols - 1);
        }
    }
    return best.0;
}

pub fn solve() {
    let day: usize = 16;
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
    println!("Coded: 94 minutes");
}
