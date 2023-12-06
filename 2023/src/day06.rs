#[path = "common.rs"]
mod common;

fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 1usize;
    let mut times: Vec<usize> = Vec::new();
    let mut dists: Vec<usize> = Vec::new();

    // parse the boat race info
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        if line.contains("Time") {
            if is_part1 {
                times = common::parse_numbers(line);
            } else {
                let newline: String = line.split_whitespace().collect();
                times = common::parse_numbers(&newline);
            }
        } else if line.contains("Distance") {
            if is_part1 {
                dists = common::parse_numbers(line);
            } else {
                let newline: String = line.split_whitespace().collect();
                dists = common::parse_numbers(&newline);
            }
        } else {
            continue;
        }
    }
    // solve number of ways to beat the record in each race
    for idx in 0..times.len() {
        let a_time = times[idx];
        let a_dist = dists[idx];
        let mut ways = 0usize;
        for speed in 0..=a_time {
            if speed * (a_time - speed) > a_dist {
                ways += 1;
            }
        }
        // println!(">>>>>> {} {} {}", times[idx], dists[idx], ways);
        acc *= ways;
    }

    return acc;
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part("input/06_train", true),
        common::read_lines_as::<usize>("input/06_val1")[0]
    );
    println!("Part1: {}", part("input/06_test", true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part("input/06_train", false),
        common::read_lines_as::<usize>("input/06_val2")[0]
    );
    println!("Part2: {}", part("input/06_test", false));
    println!("33 minutes");
}
