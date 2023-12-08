#[path = "common.rs"]
mod common;
use std::collections::HashMap;

fn part1(filename: &str) -> usize {
    let mut path: Vec<usize> = Vec::new();
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


/// Part 2 with the ghosts is pretty different
fn part2(filename: &str) -> usize {
    let mut path: Vec<usize> = Vec::new();
    let mut routes: HashMap<String, (String, String)> = HashMap::new();
    let mut locs: Vec<String> = Vec::new();
    let mut is_z: HashMap<String, bool> = HashMap::new();

    // parse the info
    let lines = common::read_lines(filename);
    for line in lines {
        if line.contains("=") {
            // parse network like "AAA = (BBB, CCC)"
            let loc_key = &line[0..3];
            let loc_left = &line[7..10];
            let loc_right = &line[12..15];
            routes.insert(loc_key.to_string(), (loc_left.to_string(), loc_right.to_string()));

            // check for A and Z
            if loc_key.to_string().ends_with("A") {
                locs.push(loc_key.to_string());
            }
            if loc_key.to_string().ends_with("Z") {
                is_z.insert(loc_key.to_string(), true);
            } else {
                is_z.insert(loc_key.to_string(), false);
            }
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

    let mut all_at_z = false;
    let mut steps = 0;
    println!("{:?}", locs);
    // println!("{:?} {}", path, path.len());
    while !all_at_z {
        let mut round_at_z = true;
        for ldx in 0..locs.len() {
            let next_loc_tuple = routes.get(&locs[ldx]).unwrap();
            // println!("dbug {} to {:?}", locs[ldx], next_loc_tuple);
            match path[steps % path.len()] {
                0 => locs[ldx] = next_loc_tuple.0.clone(),
                1 => locs[ldx] = next_loc_tuple.1.clone(),
                _ => println!("fuck"),
            }
            if !is_z.get(&locs[ldx]).unwrap() {
                // if any location is not at z, this round has failed
                round_at_z = false;
            }
        }
        // println!("{:?}", locs);
        all_at_z = round_at_z;
        steps += 1;
        if steps % 10_000_000 == 0 {
            println!("{}M {:?}", (steps as f32)/1e6, locs);
        }
        // if steps > 1_000_000_000 {
        //     println!("fail");
        //     break;
        // }
        // println!("end of step");
        // if steps > 3 {
        //     break;
        // }
    }
    // println!("{:?}", is_z);
    return steps;
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part1("input/08_train1"),
        common::read_lines_as::<usize>("input/08_val1")[0]
    );
    assert_eq!(
        part1("input/08_train2"),
        common::read_lines_as::<usize>("input/08_val2")[0]
    );
    println!("Part1: {}", part1("input/08_test"));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part2("input/08_train3"),
        common::read_lines_as::<usize>("input/08_val3")[0]
    );
    println!("Part2: {}", part2("input/08_test"));
    println!("60 minutes");
}
