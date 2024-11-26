#[path = "common.rs"]
mod common;

/// Trebuchet
// fn part1(filename: &str) -> u64 {
//     let mut acc = 0u64;

//     let lines = common::read_lines(filename);
//     for line in lines.iter() {
//         // get all numbers in each line
//         let items = common::split_numeric(line);
//         let mut alpha = 0u64;
//         let mut omega = 0u64;
//         for &val in items.iter() {
//             if alpha == 0 {
//                 // set to first found number
//                 alpha = val;
//             }
//             // set to last found number
//             omega = val;
//         }
//         // println!("endrow {}{}", alpha, omega);
//         acc += alpha * 10 + omega
//     }
//     return acc;
// }

// /// This time we will first replace the words with numbers
// fn part2(filename: &str) -> u64 {
//     let mut acc = 0u64;
//     let lines = common::read_lines(filename);
//     for line in lines.iter() {
//         // get all numbers in each line
//         // due to some trickyness I keep the first and last letters of
//         // each number in this replace since they can overlap with adjacent
//         let cleanline = line
//             .replace("one", "o1e")
//             .replace("two", "t2o")
//             .replace("three", "t3e")
//             .replace("four", "f4r")
//             .replace("five", "f5e")
//             .replace("six", "s6x")
//             .replace("seven", "s7n")
//             .replace("eight", "e8t")
//             .replace("nine", "n9e");
//         let items = common::split_numeric(&cleanline);
//         let mut alpha = 0u64;
//         let mut omega = 0u64;
//         for &val in items.iter() {
//             if alpha == 0 {
//                 // set to first found number
//                 alpha = val;
//             }
//             // set to last found number
//             omega = val;
//         }
//         // println!("endrow {}{}", alpha, omega);
//         acc += alpha * 10 + omega
//     }
//     return acc;
// }

pub fn solve() {
    let day: usize = 1;
    println!("{}", day)
    // Test part-1 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), true),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    // );
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), false),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    // );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: xxx minutes");
}
