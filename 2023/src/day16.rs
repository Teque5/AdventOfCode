#[path = "common.rs"]
mod common;
use ndarray::Array2;
// use ndarray::Axis;
use std::collections::HashMap;

struct Laze {
    grid: Array2<char>,
    energized: Array2<bool>,
    cache: HashMap<(usize, usize, char), ()>,
    rows: usize,
    cols: usize,
}

impl Laze {
    /// Constructor
    fn new(grid: Array2<char>, rows: usize, cols: usize) -> Self {
        Self {
            grid: grid,
            energized: Array2::from_elem((rows, cols), false),
            cache: HashMap::new(),
            rows,
            cols,
        }
    }

    /// reset the energizer map
    fn count_and_reset(&mut self) -> usize {
        let count = self.energized.iter().filter(|&&x| x).count();
        self.cache.clear();
        self.energized.fill(false);
        return count;
    }

    /// recursively spawn the next step of the beam
    fn follow_beam(&mut self, laser_row: usize, laser_col: usize, laser_dir: char) {
        // note these max values are due to possible overflow
        if laser_row == usize::MAX || laser_row == self.rows {
            // out of bounds left/right, laser ends
            return;
        }
        if laser_col == usize::MAX || laser_col == self.cols {
            // out of bounds top/bottom, laser ends
            return;
        }
        if self.cache.contains_key(&(laser_row, laser_col, laser_dir)) {
            // we've already done this one
            return;
        } else {
            self.cache.insert((laser_row, laser_col, laser_dir), ());
        }
        // energize this position
        self.energized[(laser_row, laser_col)] = true;
        // determine how the laser moves next
        let grid_char = self.grid[(laser_row, laser_col)];
        match laser_dir {
            '🡲' | '🡰' => {
                match grid_char {
                    '.' | '-' => {
                        // empty
                        let next_col = if laser_dir=='🡲' {laser_col + 1} else {laser_col.wrapping_sub(1)};
                        self.follow_beam(laser_row, next_col, laser_dir);
                    }
                    '/' | '\\' => {
                        // go up/down
                        match (laser_dir, grid_char) {
                            ('🡰', '\\') | ('🡲', '/') => {
                                // go up
                                self.follow_beam(laser_row.wrapping_sub(1), laser_col, '🡱');
                            }
                            ('🡰', '/') | ('🡲', '\\') => {
                                // go down
                                self.follow_beam(laser_row + 1, laser_col, '🡳')
                            }
                            _ => panic!("no!"),
                        }
                    }
                    '|' => {
                        //split
                        self.follow_beam(laser_row.wrapping_sub(1), laser_col, '🡱');
                        self.follow_beam(laser_row + 1, laser_col, '🡳');
                    }
                    _ => panic!("no!"),
                }
            }
            '🡱' | '🡳' => {
                match grid_char {
                    '.' | '|' => {
                        // empty
                        let next_row = if laser_dir=='🡳' {laser_row + 1} else {laser_row.wrapping_sub(1)};
                        self.follow_beam(next_row, laser_col, laser_dir);
                    }
                    '/' | '\\' => {
                        // go left/right
                        match (laser_dir, grid_char) {
                            ('🡱', '/') | ('🡳', '\\') => {
                                // go right
                                self.follow_beam(laser_row, laser_col + 1, '🡲');
                            }
                            ('🡱', '\\') | ('🡳', '/') => {
                                // go left
                                self.follow_beam(laser_row, laser_col.wrapping_sub(1), '🡰')
                            }
                            _ => panic!("no!"),
                        }
                    }
                    '-' => {
                        //split
                        self.follow_beam(laser_row, laser_col.wrapping_sub(1), '🡰');
                        self.follow_beam(laser_row, laser_col + 1, '🡲');
                    }
                    _ => panic!("no!"),
                }
            }
            _ => panic!("no!"),
        }
    }
}

/// Beam Splitter
fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let (grid, rows, cols) = common::read_2d_chars(filename);
    let mut state = Laze::new(grid, rows, cols);
    if is_part1 {
        // only one beam enters
        state.follow_beam(0, 0, '🡲');
        // for row in state.energized.axis_iter(Axis(0)) {
        //     let some_string: String = row.iter().map(|&b| if b {'#'} else {'.'}).collect();
        //     println!("{:}", some_string);
        // }
        // count true values in energized
        let energized_spaces = state.count_and_reset();
        return energized_spaces;
    }
    // part2 starts here; beams from all directions
    let mut best = 0usize;

    // top row
    for cdx in 0..cols {
        state.follow_beam(0, cdx, '🡳');
        let energized_spaces = state.count_and_reset();
        if energized_spaces > best {
            best = energized_spaces;
        }
    }
    // bottom row
    for cdx in 0..cols {
        state.follow_beam(rows - 1, cdx, '🡱');
        let energized_spaces = state.count_and_reset();
        if energized_spaces > best {
            best = energized_spaces;
        }
    }
    // left col
    for rdx in 0..rows {
        state.follow_beam(rdx, 0, '🡲');
        let energized_spaces = state.count_and_reset();
        if energized_spaces > best {
            best = energized_spaces;
        }
    }
    // right col
    for rdx in 0..rows {
        state.follow_beam(rdx, cols - 1, '🡰');
        let energized_spaces = state.count_and_reset();
        if energized_spaces > best {
            best = energized_spaces;
        }
    }
    return best;
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
