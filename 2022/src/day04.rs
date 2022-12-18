#[path = "common.rs"] mod common;

fn part1(filename: &str) -> i32 {
    let mut acc = 0i32;
    // read each pair
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        let elves = common::split_str_as::<String>(line, ',');
        let left = common::split_str_as::<i32>(&elves[0], '-');
        let right = common::split_str_as::<i32>(&elves[1], '-');
        //println!("L {} {}", left[0], left[1]);
        //println!("R {} {}\n", right[0], right[1]);
        if (left[0] >= right[0]) & (left[1] <= right[1]) {
            // left in right?
            acc += 1;
        } else if (right[0] >= left[0]) & (right[1] <= left[1]) {
            // right in left?
            acc += 1
        }
    }
    return acc;
    
}

fn part2(filename: &str) -> i32 {
    let mut acc = 0i32;
    // read each pair
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        let elves = common::split_str_as::<String>(line, ',');
        let left = common::split_str_as::<i32>(&elves[0], '-');
        let right = common::split_str_as::<i32>(&elves[1], '-');
        //println!("L {} {}", left[0], left[1]);
        //println!("R {} {}\n", right[0], right[1]);
        // left in right?
        if left[0] >= right[0] && left[0] <= right[1] {
            acc += 1;
        } else if left[1] >= right[0] && left[1] <= right[1] {
            acc += 1;
        } else if right[0] >= left[0] && right[0] <= left[1] {
            acc += 1;
        } else if right[1] >= left[0] && right[1] <= left[1] {
            acc += 1;
        }

    }
    return acc;
    
}

pub fn solve() {
    // validate each solver, then apply to test 
    assert_eq!(part1("input/04_train"), common::read_lines_as::<i32>("input/04_val1")[0]);
    println!("Part1: {}", part1("input/04_test"));

    assert_eq!(part2("input/04_train"), common::read_lines_as::<i32>("input/04_val2")[0]);
    println!("Part2: {}", part2("input/04_test"));

}
