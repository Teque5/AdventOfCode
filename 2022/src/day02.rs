#[path = "common.rs"] mod common;

fn part1(filename: &str) -> i32 {
    let mut acc = 0i32;
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        let game = common::split_str_as::<char>(line, ' ');
        // shape points
        if game[1] == 'X' {
            acc += 1;
        } else if game[1] == 'Y' {
            acc += 2;
        } else {
            acc += 3;
        }
        // winning points
        if game[0] == 'A' {
            if game[1] == 'X' {
                //draw
                acc += 3;
            } else if game[1] == 'Y' {
                acc += 6;
            }
        } else if game[0] == 'B' {
            if game[1] == 'Y' {
                acc += 3;
            } else if game[1] == 'Z' {
                acc += 6;
            }
        } else {
            if game[1] == 'Z' {
                acc += 3;
            } else if game[1] == 'X' {
                acc += 6;
            }
        }
        //println!("{} {} {}", game[0], game[1], acc);
    }
    return acc;
    
}

fn part2(filename: &str) -> i32 {
    let mut acc = 0i32;
    let mut action = '_';
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        let game = common::split_str_as::<char>(line, ' ');
        // what to do
        if game[0] == 'A' {
            if game[1] == 'Y' {
                // draw
                action = 'X';
            } else if game[1] == 'X' {
                // lose
                action = 'Z';
            } else {
                // win
                action = 'Y';
            }
        } else if game[0] == 'B' {
            if game[1] == 'Y' {
                // draw
                action = 'Y';
            } else if game[1] == 'X' {
                // lose
                action = 'X';
            } else {
                //win
                action = 'Z';
            }
        } else {
            // they play Scissors
            if game[1] == 'Y' {
                // draw
                action = 'Z';
            } else if game[1] == 'X' {
                // lose
                action = 'Y';
            } else {
                // win
                action = 'X';
            }
        }

        // winning points
        if game[0] == 'A' {
            if action == 'X' {
                //draw
                acc += 3;
            } else if action == 'Y' {
                acc += 6;
            }
        } else if game[0] == 'B' {
            if action == 'Y' {
                acc += 3;
            } else if action == 'Z' {
                acc += 6;
            }
        } else {
            if action == 'Z' {
                acc += 3;
            } else if action == 'X' {
                acc += 6;
            }
        }
        // shape points
        if action == 'X' {
            acc += 1;
        } else if action == 'Y' {
            acc += 2;
        } else {
            acc += 3;
        }
        //println!("{} {} {}", game[0], game[1], acc);
    }
    return acc;
    
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(part1("input/02_train"), common::read_lines_as::<i32>("input/02_val1")[0]);
    println!();
    println!("Part1: {}", part1("input/02_test"));

    // Test part-2 solver, then apply to real input.
    assert_eq!(part2("input/02_train"), common::read_lines_as::<i32>("input/02_val2")[0]);
    println!("Part2: {}", part2("input/02_test"));

}
