use aoc;

/// Claw Contraption
/// The "minimum cost" part of the problem is a red herring.
/// Directly compute the exact number of pushes; valid if integer value.
fn part(filename: &str, is_part1: bool) -> usize {
    let mut tokens = 0usize;
    // parse info
    let lines = aoc::read_lines(filename);
    let mut coeffs: Vec<i64> = Vec::new();
    let bonus = if is_part1 { 0 } else { 10000000000000 };
    for line in lines {
        // println!("{:?}", line);
        let numbers = aoc::parse_numbers::<i64>(&line);
        if line.starts_with("Button") {
            coeffs.extend(numbers);
        } else if line.starts_with("Prize") {
            // Compute tokens required to solve problem
            let p0 = numbers[0] + bonus;
            let p1 = numbers[1] + bonus;
            let x0 = coeffs[0];
            let y0 = coeffs[1];
            let x1 = coeffs[2];
            let y1 = coeffs[3];
            // compute exact intercept point directly
            let a_push = (p0 * y1 - p1 * x1) as f64 / (x0 * y1 - x1 * y0) as f64;
            let b_push = (p0 * y0 - p1 * x0) as f64 / (x1 * y0 - y1 * x0) as f64;
            if a_push.fract() == 0f64 && b_push.fract() == 0f64 {
                // exact integer solution is correct
                tokens += a_push as usize * 3 + b_push as usize;
            }

            // reset for next puzzle
            coeffs.clear();
        }
    }
    return tokens;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 480);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // no training solution was given, but I added one
    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        875318608908
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 65 Minutes");
}
