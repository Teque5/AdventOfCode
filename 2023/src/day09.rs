#[path = "common.rs"]
mod common;

fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;

    // parse info
    let lines = common::read_lines(filename);




    return acc;
}

pub fn solve() {
    let day: usize = 9;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: xxx minutes");
}
