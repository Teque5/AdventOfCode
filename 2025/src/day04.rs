use aoc;

/// ??
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;
    // parse info
    let (banks, rows, cols) = aoc::read_2d_as::<usize>(filename);

    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 123);
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // assert_eq!(part(&format!("input/{:02}_train", day), false), 123);
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 44 minutes.");
}
