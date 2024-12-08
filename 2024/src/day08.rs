#[path = "common.rs"]
mod common;
use std::collections::HashMap;

/// is position within map bounds?
#[rustfmt::skip]
fn is_in_map(rows: &usize, cols: &usize, position: &(isize, isize)) -> bool {
    position.0 >= 0
    && position.1 >= 0
    && position.0 < *rows as isize
    && position.1 < *cols as isize
}

/// Resonant Collinearity
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0;
    // parse info
    let (map, rows, cols) = common::read_2d_chars(filename);
    // create lookup table for antennas
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut nodes: Vec<(isize, isize)> = Vec::new();
    for rdx in 0..rows {
        for cdx in 0..cols {
            let key = map[(rdx, cdx)];
            if key != '.' {
                antennas
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push((rdx as isize, cdx as isize));
            }
        }
    }
    // find antinodes
    for (_key, positions) in antennas {
        // println!("{:?} {:?}", _key, positions);
        for idx in 0..positions.len() {
            for jdx in idx + 1..positions.len() {
                let i_position = positions[idx];
                let j_position = positions[jdx];
                // compute antinode
                let mut positions: Vec<(isize, isize)> = Vec::new();
                let row_distance = i_position.0 - j_position.0;
                let col_distance = i_position.1 - j_position.1;
                if is_part1 {
                    // nodes at +/- 1 distance
                    positions.push((i_position.0 - row_distance, i_position.1 - col_distance));
                    positions.push((i_position.0 + row_distance, i_position.1 + col_distance));
                    positions.push((j_position.0 - row_distance, j_position.1 - col_distance));
                    positions.push((j_position.0 + row_distance, j_position.1 + col_distance));
                } else {
                    // nodes at +/- n distance
                    for multiple in [-1, 1].iter() {
                        let mut count = 0isize;
                        loop {
                            let position = (
                                i_position.0 + row_distance * count,
                                i_position.1 + col_distance * count,
                            );
                            if is_in_map(&rows, &cols, &position) {
                                positions.push(position);
                            } else {
                                break;
                            }
                            count += multiple;
                        }
                    }
                }
                for position in positions.into_iter() {
                    if is_part1 {
                        // for part 1 we don't allow nodes to be on top of antennas
                        if position == i_position || position == j_position {
                            continue;
                        }
                    }
                    if is_in_map(&rows, &cols, &position) {
                        if nodes.contains(&position) {
                            // already counted this node
                        } else {
                            // keep track of how many unique nodes
                            nodes.push(position);
                            acc += 1
                        }
                    }
                }
            }
        }
    }
    return acc;
}

pub fn solve() {
    let day: usize = 8;
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
    println!("Coded: 90 Minutes");
}
