use aoc;

/// Not Quite Lisp
/// Keep track of floor and position, and count when we first enter the basement
fn part(filename: &str, is_part1: bool) -> isize {
    let mut floor = 0isize;
    let mut position = 0isize;

    // parse info
    let lines = aoc::read_lines(filename);
    for line in lines {
        for c in line.chars() {
            if c == '(' {
                floor += 1;
            } else if c == ')' {
                floor -= 1;
            }
            position += 1;
            if floor == -1 && !is_part1 {
                return position;
            }
        }
    }
    return floor;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), -1);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 5);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 5 minutes.");
}
