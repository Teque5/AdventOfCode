#[path = "common.rs"]
mod common;
use std::collections::HashMap;

fn part(filename: &str, is_part1: bool) -> usize {
    let mut path: Vec<usize> = Vec::new();
    // each node has a corresponding uchar value - nevermind
    // let mut nodes: HashMap<String, usize> = HashMap::new()
    let mut routes: HashMap<String, (String, String)> = HashMap::new();

    // parse the info
    let lines = common::read_lines(filename);
    for line in lines {
        if line.contains("=") {
            // parse network like "AAA = (BBB, CCC)"
            let loc_key = &line[0..3];
            let loc_left = &line[7..10];
            let loc_right = &line[12..15];
            routes.insert(loc_key.to_string(), (loc_left.to_string(), loc_right.to_string()));
        } else if line.chars().count() > 0 {
            // parse instructions
            for char in line.chars() {
                if char == 'L' {
                    path.push(0);
                } else if char == 'R' {
                    path.push(1);
                }
            }

        }
    }
    let mut loc = "AAA";
    let mut at_zzz = false;
    let mut steps = 0;

    while !at_zzz {
        let next_loc_tuple = routes.get(loc).unwrap();
        match path[steps % path.len()] {
            0 => loc = &next_loc_tuple.0,
            1 => loc = &next_loc_tuple.1,
            _ => println!("fuck"),
        }
        steps += 1;
        if loc == "ZZZ" {
            at_zzz = true;
        }
    }
    // println!("{:?}", routes);
    return steps;
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part("input/08_train1", true),
        common::read_lines_as::<usize>("input/08_val1")[0]
    );
    assert_eq!(
        part("input/08_train2", true),
        common::read_lines_as::<usize>("input/08_val2")[0]
    );
    println!("Part1: {}", part("input/08_test", true));

    // // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part("input/08_train", false),
    //     common::read_lines_as::<usize>("input/08_val2")[0]
    // );
    // println!("Part2: {}", part("input/08_test", false));
    // println!("xxx minutes");
}
