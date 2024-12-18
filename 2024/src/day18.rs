use aoc;

/// ???
fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let lines = aoc::read_lines(filename);
    return 0;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train0", day), true), 123);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    assert_eq!(part(&format!("input/{:02}_train1", day), false), 123);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: xx Minutes");
}
