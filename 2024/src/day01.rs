use aoc;

/// List Distance
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;
    let mut left: Vec<isize> = Vec::new();
    let mut right: Vec<isize> = Vec::new();

    // parse info
    let lines = aoc::read_lines(filename);
    for line in lines {
        let row = aoc::parse_numbers::<isize>(&line);
        left.push(row[0]);
        right.push(row[1]);
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

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 11);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 31);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 45 minutes");
}
