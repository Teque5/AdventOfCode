#[path = "common.rs"]
mod common;
use std::cmp;

fn part1(filename: &str) -> usize {
    let mut acc = 0usize;

    let lines = common::read_lines(filename);
    for line in lines.iter() {
        let mut fair_game = true;
        let mut id = 0usize;
        let mut position = 0usize;
        // retrieve the game ID
        for (idx, c) in line.chars().enumerate() {
            if let Some(n) = c.to_digit(10) {
                id = id * 10 + n as usize;
                position = idx + 2;
            } else if id > 0 {
                // non-digit character found
                break;
            }
        }
        let pulls = line[position..].split(";").collect::<Vec<_>>();
        for pull in pulls {
            // for each pull, cannot exceed this quantity
            let mut red = 12isize;
            let mut blue = 14isize;
            let mut green = 13isize;
            let grabs = pull.split(",").collect::<Vec<_>>();
            for grab in grabs {
                let mut val_and_color = grab.trim().split_whitespace();
                if let Some(number_str) = val_and_color.next() {
                    if let Ok(number) = number_str.parse::<isize>() {
                        if let Some(color) = val_and_color.next() {
                            match color {
                                "red" => red -= number,
                                "green" => green -= number,
                                "blue" => blue -= number,
                                _ => println!("fuck"),
                            }
                        }
                    }
                }
            }
            if red < 0 || green < 0 || blue < 0 {
                // println!("{}: {} {} {}", id, red, green, blue);
                fair_game = false;
            }
        }
        if fair_game {
            // println!("{}", id);
            acc += id;
        }
    }
    return acc;
}

fn part2(filename: &str) -> isize {
    let mut acc = 0isize;

    let lines = common::read_lines(filename);
    for line in lines.iter() {
        let mut id = 0u64;
        let mut position = 0usize;
        // retrieve the game ID
        for (idx, c) in line.chars().enumerate() {
            if let Some(n) = c.to_digit(10) {
                id = id * 10 + n as u64;
                position = idx + 2;
            } else if id > 0 {
                // non-digit character found
                break;
            }
        }
        let mut red = 0isize;
        let mut green = 0isize;
        let mut blue = 0isize;

        let pulls = line[position..].split(";").collect::<Vec<_>>();
        for pull in pulls {
            // for each pull, cannot exceed this quantity
            let grabs = pull.split(",").collect::<Vec<_>>();
            for grab in grabs {
                let mut val_and_color = grab.trim().split_whitespace();
                if let Some(number_str) = val_and_color.next() {
                    if let Ok(number) = number_str.parse::<isize>() {
                        if let Some(color) = val_and_color.next() {
                            match color {
                                "red" => red = cmp::max(red, number),
                                "green" => green = cmp::max(green, number),
                                "blue" => blue = cmp::max(blue, number),
                                _ => println!("fuck"),
                            }
                        }
                    }
                }
            }
        }
        // println!("{} power {}", id, red * green * blue);
        acc += red * green * blue;
    }

    return acc;
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part1("input/02_train"),
        common::read_lines_as::<usize>("input/02_val1")[0]
    );
    println!("Part1: {}", part1("input/02_test"));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part2("input/02_train"),
        common::read_lines_as::<isize>("input/02_val2")[0]
    );
    println!("Part2: {}", part2("input/02_test"));
}
