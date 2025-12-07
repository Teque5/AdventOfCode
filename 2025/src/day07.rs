use aoc;

/// ???
fn part1(filename: &str) -> usize {
    let mut acc = 0usize;
    // parse info
    let lines = aoc::read_lines(filename);
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part1(&format!("input/{:02}_train", day)), 123);
    println!("Part1: {}", part1(&format!("input/{:02}_test", day)));

    // assert_eq!(part2(&format!("input/{:02}_train", day)), 123);
    // println!("Part2: {}", part2(&format!("input/{:02}_test", day)));

    println!("Coded: XX minutes.");
}
