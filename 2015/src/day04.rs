use aoc;
use md5::compute;

/// five leading zeros
fn check_five(seed: &str, acc: usize) -> bool {
    let digest = compute(format!("{}{}", seed, acc));
    digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0
}

/// six leading zeros
fn check_six(seed: &str, acc: usize) -> bool {
    let digest = compute(format!("{}{}", seed, acc));
    digest[0] == 0 && digest[1] == 0 && digest[2] == 0
}

/// The Ideal Stocking Stuffer
/// MD5 hashing akin to bitcoin mining
fn part(filename: &str, is_part1: bool) -> usize {
    let seed = aoc::read_lines(filename)[0].clone();
    let check = if is_part1 { check_five } else { check_six };
    let mut acc = 0usize;
    while !check(&seed, acc) {
        acc += 1;
    }
    acc
}

pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 609043);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 6742839);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 60 minutes.");
}
