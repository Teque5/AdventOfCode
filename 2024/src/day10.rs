use aoc;
use ndarray::Array2;

#[rustfmt::skip]
fn is_in_map(rows: usize, cols: usize, position: (isize, isize)) -> bool {
    position.0 >= 0
    && position.1 >= 0
    && position.0 < rows as isize
    && position.1 < cols as isize
}

fn is_sloped(
    topo: &Array2<char>,
    first_position: (usize, usize),
    second_position: (isize, isize),
    is_part1: bool,
) -> bool {
    let first_height = topo[first_position].to_digit(10).unwrap();
    let second_height = topo[(second_position.0 as usize, second_position.1 as usize)]
        .to_digit(10)
        .unwrap();
    let difference = second_height as isize - first_height as isize;
    if is_part1 {
        return difference == 1;
    } else {
        return difference == -1;
    }
}

/// walk & count paths either up or down
/// when counting paths up (part1), do not overlap prior routes
/// when counting paths down (part2), allow traversing same positions
fn walk(
    topo: &Array2<char>,
    traversed: &mut Array2<bool>,
    rows: usize,
    cols: usize,
    position: (usize, usize),
    is_part1: bool,
) -> usize {
    // set this location as traversed
    traversed[position] = true;
    let mut rating = 0usize;
    let up = (position.0 as isize - 1, position.1 as isize);
    let down = (position.0 as isize + 1, position.1 as isize);
    let left = (position.0 as isize, position.1 as isize - 1);
    let right = (position.0 as isize, position.1 as isize + 1);
    for direction in [up, down, left, right].iter() {
        if is_in_map(rows, cols, *direction) && is_sloped(&topo, position, *direction, is_part1) {
            let new_position = (direction.0 as usize, direction.1 as usize);
            if !traversed[new_position] || !is_part1 {
                rating += walk(&topo, traversed, rows, cols, new_position, is_part1);
            }
        }
    }
    if is_part1 {
        // are we at a 9?
        if topo[position] == '9' {
            rating += 1;
        }
    } else {
        if topo[position] == '0' {
            rating += 1
        }
    }
    return rating;
}

/// Hoof It
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0;
    // parse info
    let (topo, rows, cols) = aoc::read_2d_chars(filename);
    // hold previously traversed routes
    for rdx in 0..rows {
        for cdx in 0..cols {
            let mut traversed: Array2<bool> = Array2::from_elem((rows, cols), false);
            let target = if is_part1 { '0' } else { '9' };
            if topo[(rdx, cdx)] == target {
                // these are all the start positions
                let position = (rdx, cdx);
                let rating = walk(&topo, &mut traversed, rows, cols, position, is_part1);
                // println!("start=({},{}), rating={}", rdx, cdx, rating);
                acc += rating
            }
        }
    }

    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 36);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 81);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 104 Minutes");
}
