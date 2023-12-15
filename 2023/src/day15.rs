#[path = "common.rs"]
mod common;

fn hash_char(some_char: char, current_value: &mut usize) {
    *current_value += some_char as usize;
    *current_value *= 17;
    *current_value %= 256;
}

/// Lens Library
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;

    // parse info
    let lines = common::read_lines(filename);
    for line in lines {
        // println!("line {}", line);
        let split_line = common::split_str_as::<String>(&line, ',');
        // println!("enter! {:?}", split_line);
        for step in split_line.iter() {
            let mut charval = 0usize;
            for c in step.chars() {
                hash_char(c, &mut charval);
            }
            acc += charval;
            // println!("{} {}", step, charval)
        }
    }

    return acc;
}

pub fn solve() {
    let day: usize = 15;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), false),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    // );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: xxx minutes");
}
