#[path = "common.rs"] mod common;

use std::fs;

fn part(filename: &str, is_9001: bool) -> String {
    let raw = fs::read_to_string(filename).unwrap();
    let parts: Vec<&str> = raw.split("\n\n").collect();
    let mut topstack = Vec::new();
    // first part is stack, second part is instructions
    for (ldx, line) in parts[0].lines().rev().skip(1).enumerate() {
        if ldx == 0 {
            // construct stack vectors
            //let stack_count = (line.len()+1) / 4;
            for (vdx, val) in line.chars().enumerate() {
                if vdx % 2 != 0 && val != ' ' {
                    let mut stack = Vec::new();
                    stack.push(val);
                    topstack.push(stack);
                }
            }
        } else {
            for (vdx, val) in line.chars().enumerate() {
                if vdx % 2 != 0 && val != ' ' {
                    topstack[(vdx-1)/4].push(val);
                }
            }
        }
    }
    // parse instructions
    for line in parts[1].lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let count = tokens[1].parse::<usize>().unwrap();
        let move_from = tokens[3].parse::<usize>().unwrap();
        let move_to = tokens[5].parse::<usize>().unwrap();
//        println!("dbug {} {} {}", count, move_from, move_to);
        if is_9001 {
            // move crates as a group
            let mut buffer = Vec::new();
            for _ in 0..count {
                buffer.push(topstack[move_from-1].pop().unwrap());
            }
            buffer.reverse();
            topstack[move_to-1].extend(buffer);
            
        } else {
            // move crates one at a time
            for _ in 0..count {
                let val = topstack[move_from-1].pop().unwrap();
                topstack[move_to-1].push(val);
            }
        }
    }
    let mut out = "".to_string();

// verify stack
    for stack in topstack {
        out.push(*stack.last().unwrap());
        /*
        println!("s");
        for item in stack {
            print!("{} ", item);
        }
        */
    }

    return out;
}


pub fn solve() {
    // validate each solver, then apply to test 
    assert_eq!(part("input/05_train", false), common::read_lines("input/05_val1")[0]);
    println!();
    println!("Part1: {}", part("input/05_test", false));

    assert_eq!(
        part("input/05_train", true),
        common::read_lines("input/05_val2")[0]
    );
    println!("Part2: {}", part("input/05_test", true));

}
