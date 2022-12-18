#[path = "common.rs"] mod common;

//use itertools::Itertools; // allows use of combinations / permutations
use std::collections::HashMap;
use std::path::PathBuf; // kinda like pathlib // it's a dictionary yo

/// create a hashmap of directories w/total size of all subdirectories
fn part(filename: &str, part2: bool) -> i32 {
    let commands = common::read_lines(filename);
    let mut folders: HashMap<String, i32> = HashMap::new();
    let mut path = PathBuf::new();
    // assemble the hashmap
    for command in commands {
        //println!("\t{}", command);
        if command == "" {
            continue;
        } else if command == "$ cd /" {
            continue;
        } else if command == "$ ls" {
            continue;
        } else if command.starts_with("dir") {
            continue;
        } else if command.starts_with("$ cd") {
            // increment or decrement path
            if command.ends_with("..") {
                // go up a level
                path.pop();
            } else {
                // go down a level
                path = path.join(&command[5..]);
            }
        } else {
            // add file to hashmap
            let parts: Vec<&str> = command.split(" ").collect();
            let size: i32 = parts[0].parse().unwrap();
            //let fname: String = parts[1].parse().unwrap();
            if folders.contains_key(&path.display().to_string()) {
                // add to key
                //let mut val = folders.get_mut(path.display().to_string());
                *folders.get_mut(&path.display().to_string()).unwrap() += size;
            } else {
                // insert new key
                folders.insert(path.display().to_string(), size);
            }
            // add to parents
            for ancestor in path.ancestors().skip(1) {
                if folders.contains_key(&ancestor.display().to_string()) {
                    *folders.get_mut(&ancestor.display().to_string()).unwrap() += size;
                } else {
                    folders.insert(ancestor.display().to_string(), size);
                }
            }
        }
    }

    if !part2 {
        let mut acc = 0i32;
        // find smaller dirs (part 1)
        for (_key, value) in folders {
            if value <= 100000 {
                acc += value;
            }
        }
        return acc;
    } else {
        // find smallest dir capable of freeing up 30M
        let required = folders.get(&"".to_string()).unwrap() - 40000000;
        let mut best_value: i32 = i32::MAX; // max possible
        for (_key, value) in folders {
            let diff = value - required;
            if (diff >= 0) && (value < best_value) {
                best_value = value;
            }
        }
        return best_value;
    }
}

pub fn solve() {
    let pdx = 7;

    // validate each solver, then apply to test
    assert_eq!(
        part(&format!("input/{:02}_train", pdx), false),
        common::read_lines_as::<i32>(&format!("input/{:02}_val1", pdx))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", pdx), false));

    // part 2
    assert_eq!(
        part(&format!("input/{:02}_train", pdx), true),
        common::read_lines_as::<i32>(&format!("input/{:02}_val2", pdx))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", pdx), true));
}
