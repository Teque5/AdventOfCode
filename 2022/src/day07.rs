#[path = "common.rs"] mod common;

use std::fs;
//use itertools::Itertools; // allows use of combinations / permutations
use std::path::PathBuf; // kinda like pathlib
use std::collections::HashMap; // it's a dictionary yo

/// (n choose k) unique values from front of string
fn part(filename: &str) -> i32 {
    let commands = common::read_lines(filename);
//.unwrap().chars().collect();
    let mut folders: HashMap<String, i32> = HashMap::new();
//    let mut path = "/".to_string();
    let mut path = PathBuf::new();
    // assemble the hashmap
    for command in commands {
//        println!("start {}", command);
        if command == "$ cd /" {
            continue;
        } else if command == "$ ls" {
            continue;
        } else if command.starts_with("dir") {
            continue;
        } else if command.starts_with("$ cd") {
            // increment or decrement hash key
            if command.ends_with("..") {
                // go up a level
//                println!("-");
                path.pop();
            } else {
                path = path.join(&command[5..]);
            }
//            println!("@{}", path.display());
//            command[5..]
        } else {
            let parts: Vec<&str> = command.split(" ").collect();
            let size: i32 = parts[0].parse().unwrap();
            if folders.contains_key(&path.display().to_string()) {
                // add to key
                //let mut val = folders.get_mut(path.display().to_string());
                *folders.get_mut(&path.display().to_string()).unwrap() += size;
            } else {
                // insert new key
                folders.insert(path.display().to_string(), size);
            }
//            let fname: String = parts[1].parse().unwrap();
//            println!("{} {}", size, size)
        }

    }
    // find smaller dirs (part 1)
    let mut acc = 0i32;
    for (key, value) in folders {
        println!{"{} = {}", key, value}
        if value <= 100000 {
            acc += value;
        }
    }

    return acc;
}


pub fn solve() {
    let pdx = 7;

    // validate each solver, then apply to test 
    assert_eq!(
        part(&format!("input/{:02}_train", pdx)),
        common::read_lines_as::<i32>(&format!("input/{:02}_val1", pdx))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", pdx)));
/*
    // part 2
    assert_eq!(
        part("input/06_train", 14),
        common::read_lines_as::<i32>(&format!("input/{:02}_val2", pdx))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", pdx)));
*/

}
