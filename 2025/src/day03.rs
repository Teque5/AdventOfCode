use aoc;

/// Lobby
/// Pick highest joltage by pairing numbers from each row
fn part(filename: &str, is_part1: bool) -> usize {
    let mut joltage = 0usize;
    // parse info
    let (banks, rows, cols) = aoc::read_2d_as::<usize>(filename);
    // aoc::print_2d(&banks);

    let num_digits = if is_part1 { 2usize } else { 12usize };
    for rdx in 0..rows {
        let mut buffer: Vec<usize> = vec![0; num_digits];
        // left edge will scan right
        let mut left_edge = 0usize;
        for digit in 0..num_digits {
            let mut maxval = 0usize;
            let mut maxcdx;
            let right_edge = cols - (num_digits - digit);
            // iterate backwards from right edge-12 to left edge
            for cdx in (left_edge..=right_edge).rev() {
                // find largest digit
                if banks[[rdx, cdx]] >= maxval {
                    maxval = banks[[rdx, cdx]];
                    maxcdx = cdx;
                    buffer[digit] = maxval;
                    left_edge = maxcdx + 1;
                }
            }
        }
        // convert buffer to number
        let mut number = 0usize;
        for digit in 0..num_digits {
            number = number * 10 + buffer[digit];
        }
        joltage += number;
    }
    return joltage;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 357);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        3121910778619
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 44 minutes.");
}
