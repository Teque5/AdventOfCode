use aoc;

/// Monkey Psudorandom Sequence
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;
    let mut operand: usize;

    // parse codes
    let initial_numbers = aoc::read_lines_as::<usize>(filename);
    for idx in 0..initial_numbers.len() {
        let mut number = initial_numbers[idx];
        for _ in 0..2000 {
            operand = number * 64;
            number = prune(mix(number, operand));
            operand = number / 32;
            number = prune(mix(number, operand));
            operand = number * 2048;
            number = prune(mix(number, operand));
            let price = number % 10; // also # of bananas
        }
        println!("{:?}", number);
        acc += number;
    }
    // for number in &initial_numbers {
    // }
    return acc;
}

/// prune secret number
fn prune(secret: usize) -> usize {
    secret % 16777216
}

/// mix into secret number
fn mix(secret: usize, operator: usize) -> usize {
    secret ^ operator
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 37327623);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: 45+ Minutes");
}
