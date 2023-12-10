#[path = "common.rs"]
mod common;
use ndarray::s;
use ndarray::Array2;

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

fn step(
    tiles: &Array2<char>,
    rows: usize,
    cols: usize,
    pos: (usize, usize),
    pos_prev: (usize, usize),
) -> (usize, usize) {
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

    // the program should never get here
    // return (0, 0);
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
        let next_pos = step(&tiles, rows, cols, pos, pos_prev);
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

// scrub to find inner/outer
fn scrub(containment: &mut Array2<char>, rows: usize, cols: usize) {
    for rdx in 0..rows {
        for cdx in 0..cols {
            // println!("going? {} {}", rdx, cdx);
            let pos = (rdx, cdx);
            if containment[pos] == '.' {
                // println!("d");
                if (pos.1 == 0) || (pos.0 == 0) || (pos.0 == rows - 1) || (pos.1 == cols - 1) {
                    // on edge
                    containment[pos] = 'o';
                    // println!("e{}", containment[pos])
                } else {
                    let trbl = vec![
                        containment[(rdx - 1, cdx)],
                        containment[(rdx, cdx + 1)],
                        containment[(rdx + 1, cdx)],
                        containment[(rdx, cdx - 1)],
                    ];
                    // println!("{:?}", trbl);
                    for char in trbl {
                        if char == 'o' {
                            containment[pos] = 'o';
                            // println!("o{}", containment[pos])
                        }
                    }
                }
            }
        }
    }

    return;
}

/// hot springs pipes
fn part2(filename: &str) -> usize {
    // parse info
    let (tiles, rows, cols) = common::read_2d_chars(filename);
    let mut containment = Array2::from_elem((rows, cols), '.');

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
    containment[pos_start] = 'x';
    let mut pos_prev = pos_start.clone();
    // look around until we find the pipe
    pos = possible_step(&tiles, rows, cols, pos, pos_prev);
    containment[pos] = 'x';

    // println!("1 {:?}", pos);

    loop {
        let next_pos = step(&tiles, rows, cols, pos, pos_prev);
        pos_prev = pos;
        pos = next_pos;
        containment[pos] = 'x';
        // println!("{} {:?}", 8, pos);
        if pos == pos_start {
            break;
        }
    }
    // let is_contained = false;
    for _ in 0..100 {
        scrub(&mut containment, rows, cols);
    }

    println!("dbug {} {}", rows, cols);
    let mut acc = 0usize;
    for rdx in 0..rows {
        for cdx in 0..cols {
            if containment[(rdx, cdx)] == '.' {
                acc += 1;
            }
        }
    }
    for rdx in 0..rows {
        let some_string: String = containment.slice(s![rdx, ..]).into_iter().collect();
        println!("{:}", some_string)
    }
    // 558 is too high
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
    // assert_eq!(
    //     part2(&format!("input/{:02}_train2", day)),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    // );
    // assert_eq!(
    //     part2(&format!("input/{:02}_train3", day)),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val3", day))[0]
    // );
    println!("Part2: {}", part2(&format!("input/{:02}_test", day)));
    // println!("Coded: xxx minutes");
}
