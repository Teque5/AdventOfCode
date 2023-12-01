#[path = "common.rs"]
mod common;

fn part1(filename: &str) -> u64 {
    let mut acc = 0u64;

    let lines = common::read_lines(filename);
    for line in lines.iter() {
        // get all numbers in each line
        let items = common::split_numeric(line);
        let mut alpha = 0u64;
        let mut omega = 0u64;
        for &val in items.iter() {
            if alpha == 0 {
                // set to first found number
                alpha = val;
            }
            // set to last found number
            omega = val;
        }
        // println!("endrow {}{}", alpha, omega);
        acc += alpha * 10 + omega
    }
    return acc;
}

// This time we will first replace the words with numbers
fn part2(filename: &str) -> u64 {
    let mut acc = 0u64;

    let lines = common::read_lines(filename);
    for line in lines.iter() {
        // get all numbers in each line
        // let mut cleanline = line;
        let cleanline = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        let items = common::split_numeric(&cleanline);
        let mut alpha = 0u64;
        let mut omega = 0u64;
        for &val in items.iter() {
            if alpha == 0 {
                // set to first found number
                alpha = val;
            }
            // set to last found number
            omega = val;
        }
        // println!("endrow {}{}", alpha, omega);
        acc += alpha * 10 + omega
    }
    return acc;
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part1("input/01_train1"),
        common::read_lines_as::<u64>("input/01_val1")[0]
    );
    println!("Part1: {}", part1("input/01_test"));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part2("input/01_train2"),
        common::read_lines_as::<u64>("input/01_val2")[0]
    );
    println!("Part2: {}", part2("input/01_test"));
}
