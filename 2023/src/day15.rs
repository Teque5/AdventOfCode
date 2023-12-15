#[path = "common.rs"]
mod common;

fn hash_string(some_string: &String) -> usize {
    let mut hash = 0usize;
    for some_char in some_string.chars() {
        hash += some_char as usize;
        hash *= 17;
        hash %= 256;
    }
    return hash;
}

/// Lens Library
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;

    // parse info
    let line = common::read_lines(filename)[0].clone();

    let split_line = common::split_str_as::<String>(&line, ',');

    for step in split_line.iter() {
        acc += hash_string(step);

    }


    return acc;
}


/// Lens Library
// fn part1(filename: &str, is_part1: bool) -> usize {
//     let mut acc = 0usize;

//     // parse info
//     let lines = common::read_lines(filename);
//     for line in lines {
//         // println!("line {}", line);
//         let split_line = common::split_str_as::<String>(&line, ',');
//         // println!("enter! {:?}", split_line);
//         for step in split_line.iter() {
//             let mut charval = 0usize;
//             for c in step.chars() {
//                 hash_char(c, &mut charval);
//             }
//             acc += charval;
//             // println!("{} {}", step, charval)
//         }
//     }

//     return acc;
// }

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
    //     part2(&format!("input/{:02}_train", day), false),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    // );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 20+ minutes");
}
