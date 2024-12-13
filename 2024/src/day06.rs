use aoc;
use indicatif::{ProgressBar, ProgressStyle};
use ndarray::Array2;
use ndarray::Array3;

/// Move the Guard, return false if outside the map
fn walk(
    map: &Array2<char>,
    rows: &usize,
    cols: &usize,
    position: &mut (usize, usize),
    direction: &mut (isize, isize),
) -> bool {
    let next_position = (
        position.0 as isize + direction.0,
        position.1 as isize + direction.1,
    );
    // check for map edge
    if next_position.0 == -1
        || next_position.0 == *rows as isize
        || next_position.1 == -1
        || next_position.1 == *cols as isize
    {
        return false;
    }
    let new_position = (next_position.0 as usize, next_position.1 as usize);
    // check for obstacles
    if map[new_position] == '#' {
        // rotate 90 and be done
        match direction {
            (-1, 0) => *direction = (0, 1),  // up -> right
            (0, 1) => *direction = (1, 0),   // right -> down
            (1, 0) => *direction = (0, -1),  // down -> left
            (0, -1) => *direction = (-1, 0), // left -> up
            _ => {}
        }
    } else {
        // move guard to new position
        *position = new_position;
    }
    // println!("{:?} {:?}", position, direction);
    return true;
}

/// Guard Gallivant
fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let (map, rows, cols) = aoc::read_2d_chars(filename);
    // aoc::print_2d_chars(&map);

    // find start position
    let mut position: (usize, usize) = (0, 0);
    let mut direction: (isize, isize) = (-1, 0); // start facing up (row, col)
    let mut ok = false;
    for rdx in 0..rows {
        for cdx in 0..cols {
            if map[(rdx, cdx)] == '^' {
                position = (rdx, cdx);
                ok = true;
                break;
            }
        }
        if ok {
            break;
        }
    }
    let original_position = position;
    let original_direction = direction;
    let mut patrol: Array2<usize> = Array2::zeros((rows, cols));
    // walk the map
    while ok {
        patrol[position] = 1;
        ok = walk(&map, &rows, &cols, &mut position, &mut direction);
    }
    if is_part1 {
        return patrol.sum();
    }
    // part 2 down here
    // find location for new obstruction that causes infinite loop
    let mut acc = 0usize;
    let mut ddx = 0usize;
    let mut map2 = map.clone();

    // walked 3rd dimension: up, right, down, left
    let mut walked: Array3<bool> = Array3::from_elem((rows, cols, 4), false);
    // fancy progress bar
    let progress = ProgressBar::new((rows * cols) as u64);
    progress.set_style(ProgressStyle::with_template(aoc::STYLE).unwrap());
    progress.set_message("Simulating Obstacles");
    // add obstruction
    for rdx in 0..rows {
        for cdx in 0..cols {
            if map2[(rdx, cdx)] == '#' {
                // cannot add obstacle where one exists
                continue;
            } else if patrol[(rdx, cdx)] == 0 {
                // will not intersect obstacle
                continue;
            } else {
                progress.set_position((rdx * cols + cdx) as u64);
                // insert obstacle
                map2[(rdx, cdx)] = '#';
                ok = true;
                position = original_position;
                direction = original_direction;
                // walk the map
                while ok {
                    match direction {
                        (-1, 0) => ddx = 0, // up
                        (0, 1) => ddx = 1,  // right
                        (1, 0) => ddx = 2,  // down
                        (0, -1) => ddx = 3, // left
                        _ => {}
                    }
                    if walked[(position.0, position.1, ddx)] == true {
                        // we've walked exactly like this before
                        acc += 1;
                        ok = false;
                    } else {
                        walked[(position.0, position.1, ddx)] = true;
                        ok = walk(&map2, &rows, &cols, &mut position, &mut direction);
                    }
                }
                // reset obstacle
                map2[(rdx, cdx)] = '.';
                // reset walk path
                walked.fill(false);
            }
        }
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 41);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 6);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 86 Minutes");
}
