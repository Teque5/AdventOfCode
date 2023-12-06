#[path = "common.rs"]
mod common;

fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 1usize;
    let mut times: Vec<usize> = Vec::new();
    let mut dists: Vec<usize> = Vec::new();

    // parse instructions
    let lines = common::read_lines(filename);

    return acc;
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part("input/07_train", true),
        common::read_lines_as::<usize>("input/07_val1")[0]
    );
    // println!("Part1: {}", part("input/07_test", true));

    // // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part("input/07_train", false),
    //     common::read_lines_as::<usize>("input/07_val2")[0]
    // );
    // println!("Part2: {}", part("input/07_test", false));
    // println!("xxx minutes");
}
