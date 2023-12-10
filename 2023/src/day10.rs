#[path = "common.rs"]
mod common;
use ndarray::Array2;

/// given two positions and a previous, return only the new one
fn check_prev(
    pos_poss: ((usize, usize), (usize, usize)),
    pos_prev: (usize, usize),
) -> (usize, usize) {
    if pos_poss.0 == pos_prev {
        return pos_poss.1;
    } else {
        return pos_poss.0;
    }
}

/// return the next position, given current and prior
fn step(tiles: &Array2<char>, pos: (usize, usize), pos_prev: (usize, usize)) -> (usize, usize) {
    let this_char = tiles[pos];
    match this_char {
        '|' => {
            let pos_poss = ((pos.0 - 1, pos.1), (pos.0 + 1, pos.1));
            return check_prev(pos_poss, pos_prev);
        }
        '-' => {
            let pos_poss = ((pos.0, pos.1 - 1), (pos.0, pos.1 + 1));
            return check_prev(pos_poss, pos_prev);
        }
        'J' => {
            let pos_poss = ((pos.0 - 1, pos.1), (pos.0, pos.1 - 1));
            return check_prev(pos_poss, pos_prev);
        }
        'L' => {
            let pos_poss = ((pos.0 - 1, pos.1), (pos.0, pos.1 + 1));
            return check_prev(pos_poss, pos_prev);
        }
        '7' => {
            let pos_poss = ((pos.0, pos.1 - 1), (pos.0 + 1, pos.1));
            return check_prev(pos_poss, pos_prev);
        }
        'F' => {
            let pos_poss = ((pos.0, pos.1 + 1), (pos.0 + 1, pos.1));
            return check_prev(pos_poss, pos_prev);
        }
        _ => panic!("not possible {}", this_char),
    }
}

fn possible_step(
    tiles: &Array2<char>,
    rows: usize,
    cols: usize,
    pos: (usize, usize),
    pos_prev: (usize, usize),
) -> (usize, usize) {
    if pos.1 > 0 {
        // check left
        let possible = (pos.0, pos.1 - 1);
        if possible != pos_prev {
            if ['-', 'F', 'L'].contains(&tiles[possible]) {
                return possible;
            }
        }
    }
    if pos.0 > 0 {
        // check top
        let possible = (pos.0 - 1, pos.1);
        if possible != pos_prev {
            if ['|', '7', 'F'].contains(&tiles[possible]) {
                return possible;
            }
        }
    }
    if pos.1 < cols - 1 {
        // check right
        let possible = (pos.0, pos.1 + 1);
        if possible != pos_prev {
            if ['-', '7', 'J'].contains(&tiles[possible]) {
                return possible;
            }
        }
    }
    if pos.0 < rows - 1 {
        // check bottom
        let possible = (pos.0 + 1, pos.1);
        if possible != pos_prev {
            if ['|', 'J', 'L'].contains(&tiles[possible]) {
                return possible;
            }
        }
    }
    return (0, 0);
}

/// hot springs pipes
fn part1(filename: &str) -> usize {
    // parse info
    let (tiles, rows, cols) = common::read_2d_chars(filename);

    // find s_position
    let mut pos = (0usize, 0usize);
    for rdx in 0..rows {
        for cdx in 0..cols {
            if tiles[(rdx, cdx)] == 'S' {
                pos = (rdx, cdx);
            }
        }
    }
    let pos_start = pos.clone();
    // println!("0 {:?}", pos_start);
    let mut pos_prev = pos_start.clone();
    // look around until we find the pipe
    pos = possible_step(&tiles, rows, cols, pos, pos_prev);
    let mut acc = 1usize;
    // println!("1 {:?}", pos);

    loop {
        let next_pos = step(&tiles, pos, pos_prev);
        pos_prev = pos;
        pos = next_pos;
        acc += 1;
        // println!("{} {:?}", acc, pos);
        if pos == pos_start {
            break;
        }
    }

    return acc / 2;
}

/// waterfill to find inner/outer
fn waterfill(containment: &mut Array2<char>, rows: usize, cols: usize) -> bool {
    // set filled if any pixels were filled
    let mut filled = false;
    for rdx in 0..rows {
        for cdx in 0..cols {
            let pos = (rdx, cdx);
            if containment[pos] == '.' {
                if (pos.1 == 0) || (pos.0 == 0) || (pos.0 == rows - 1) || (pos.1 == cols - 1) {
                    // on edge
                    containment[pos] = 'o';
                    filled = true;
                } else {
                    // TOP RIGHT BOTTOM LEFT
                    let trbl = vec![
                        containment[(rdx - 1, cdx)],
                        containment[(rdx, cdx + 1)],
                        containment[(rdx + 1, cdx)],
                        containment[(rdx, cdx - 1)],
                    ];
                    for char in trbl {
                        if char == 'o' {
                            containment[pos] = 'o';
                            filled = true;
                        }
                    }
                }
            }
        }
    }

    return filled;
}

/// fill out interpolated containment
fn mark_contained(tiles: &Array2<char>, containment: &mut Array2<char>, pos: (usize, usize)) {
    containment[(pos.0 * 3 + 1, pos.1 * 3 + 1)] = 'x';
    match tiles[pos] {
        'S' => {
            containment[(pos.0 * 3, pos.1 * 3 + 1)] = 'x';
            containment[(pos.0 * 3 + 1, pos.1 * 3)] = 'x';
            containment[(pos.0 * 3 + 1, pos.1 * 3 + 1)] = 'x';
            containment[(pos.0 * 3 + 1, pos.1 * 3 + 2)] = 'x';
            containment[(pos.0 * 3 + 2, pos.1 * 3 + 1)] = 'x';
        }
        'J' => {
            containment[(pos.0 * 3, pos.1 * 3 + 1)] = 'x';
            containment[(pos.0 * 3 + 1, pos.1 * 3)] = 'x';
        }
        'L' => {
            containment[(pos.0 * 3, pos.1 * 3 + 1)] = 'x';
            containment[(pos.0 * 3 + 1, pos.1 * 3 + 2)] = 'x';
        }
        'F' => {
            containment[(pos.0 * 3 + 1, pos.1 * 3 + 2)] = 'x';
            containment[(pos.0 * 3 + 2, pos.1 * 3 + 1)] = 'x';
        }
        '7' => {
            containment[(pos.0 * 3 + 1, pos.1 * 3)] = 'x';
            containment[(pos.0 * 3 + 2, pos.1 * 3 + 1)] = 'x';
        }
        '-' => {
            containment[(pos.0 * 3 + 1, pos.1 * 3)] = 'x';
            containment[(pos.0 * 3 + 1, pos.1 * 3 + 2)] = 'x';
        }
        '|' => {
            containment[(pos.0 * 3, pos.1 * 3 + 1)] = 'x';
            containment[(pos.0 * 3 + 2, pos.1 * 3 + 1)] = 'x';
        }
        _ => panic!("not possible {}", tiles[pos]),
    }
}

/// hot springs pipes, but now interpolated by 3x
fn part2(filename: &str) -> usize {
    // parse info
    let (tiles, rows, cols) = common::read_2d_chars(filename);
    let mut containment = Array2::from_elem((rows * 3, cols * 3), '.');

    // find s_position
    let mut pos = (0usize, 0usize);
    for rdx in 0..rows {
        for cdx in 0..cols {
            if tiles[(rdx, cdx)] == 'S' {
                pos = (rdx, cdx);
            }
        }
    }
    mark_contained(&tiles, &mut containment, pos);
    let pos_start = pos.clone();

    let mut pos_prev = pos_start.clone();
    // look around until we find the pipe
    pos = possible_step(&tiles, rows, cols, pos, pos_prev);
    mark_contained(&tiles, &mut containment, pos);

    loop {
        let next_pos = step(&tiles, pos, pos_prev);

        pos_prev = pos;
        pos = next_pos;

        mark_contained(&tiles, &mut containment, pos);

        if pos == pos_start {
            break;
        }
    }

    let mut filled = true;
    while filled {
        // keep calling this until waterfill is complete
        filled = waterfill(&mut containment, rows * 3, cols * 3);
    }

    // for rdx in 0..rows*3 {
    //     let some_string: String = containment.slice(s![rdx, ..]).into_iter().collect();
    //     println!("{:}", some_string)
    // }

    // count the center points
    let mut acc = 0usize;
    for rdx in 0..rows {
        for cdx in 0..cols {
            if containment[(rdx * 3 + 1, cdx * 3 + 1)] == '.' {
                acc += 1;
            }
        }
    }
    return acc;
}

pub fn solve() {
    let day: usize = 10;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part1(&format!("input/{:02}_train1", day)),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part1(&format!("input/{:02}_test", day)));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part2(&format!("input/{:02}_train2", day)),
        common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    );
    assert_eq!(
        part2(&format!("input/{:02}_train3", day)),
        common::read_lines_as::<usize>(&format!("input/{:02}_val3", day))[0]
    );
    println!("Part2: {}", part2(&format!("input/{:02}_test", day)));
    // println!("Coded: xxx minutes");
}
