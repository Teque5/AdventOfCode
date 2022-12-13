#[path = "common.rs"] mod common;

use std::fs;
use itertools::Itertools; // allows use of combinations / permutations

/// (n choose k) unique values from front of string
fn part(filename: &str, k: usize) -> i32 {
    let mut indent = 0i32;
    // read char vector
    let tx: Vec<_> = fs::read_to_string(filename).unwrap().chars().collect();

    for idx in 0..tx.len()-k {
        let mut ok = true;
        let sub = &tx[idx..idx+k];
        for (left, right) in sub.iter().tuple_combinations() {
            //println!("{} LR {} {} {}", idx, left, right, left==right);

            if left == right {
                ok = false;
            }
        }
        if ok {
            indent = idx as i32 + k as i32;
            break;
        }
    }
//    println!("ok");
    return indent;
}


pub fn solve() {
    let pdx = "06";
    // validate each solver, then apply to test 
    // compare 4 values
    assert_eq!(part("input/06_train", 4), common::read_lines_as::<i32>("input/06_val1")[0]);
    println!();
    println!("Part1: {}", part("input/06_test", 4));

    // compare 14 values
    assert_eq!(part("input/06_train", 14), common::read_lines_as::<i32>("input/06_val2")[0]);
    println!("Part2: {}", part("input/06_test", 14));

}
