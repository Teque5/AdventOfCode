#[path = "common.rs"]
mod common;

/// List Distance
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;
    let mut left: Vec<isize> = Vec::new();
    let mut right: Vec<isize> = Vec::new();

    // parse info
    let lines = common::read_lines(filename);
    for line in lines {
        let row = common::parse_numbers_isize(&line);
        left.push(row[0] as isize);
        right.push(row[1] as isize);
    }
    if is_part1 {
        // between left and right, sum distances between lists
        // sort lists
        left.sort();
        right.sort();
        // sum up distances
        for idx in 0..left.len() {
            acc += (left[idx] - right[idx]).abs() as usize;
          }
    } else {
        // count how many times left number appears in right list; do multiply later
        for &lval in left.iter() {
            let mut subcount = 0usize;
            for &rval in right.iter() {
                if lval == rval {
                    subcount += 1;
                }
            }
            acc += subcount * (lval as usize);
        }
    }
    return acc;
}

pub fn solve() {
    let day: usize = 1;
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
    println!("Coded: 45 minutes");
}
