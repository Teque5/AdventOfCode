#[path = "common.rs"] mod common;

fn part1(filename: &str) -> i32 {
    let mut idx = 1i32;
    let mut acc = 0i32;
    let mut max = (0i32, 0i32); // idx, val
    let x = common::read_ints(filename);
    for &val in x.iter() {
        if val >= 0 {
            acc += val;
        } else {
            if max.1 < acc {
                max = (idx, acc);
                //println!("{} {}", max.0, max.1);
            }
            acc = 0;
            idx += 1;
        }
    }
    return max.1;
}

fn part2(filename: &str) -> i32 {
    let mut sums = Vec::new();
    let mut acc = 0i32;

    let x = common::read_ints(filename);
    for &val in x.iter() {
        if val >= 0 {
            acc += val;
        } else {
            sums.push(acc);
//            println!("{}", acc);
            acc = 0;
        }
    }
    if acc != 0 {
        sums.push(acc);
    }
    sums.sort_unstable();
//    for sum in sums.iter() {
//        println!("{}", sum)
//    }
    let total = sums.iter().rev().take(3).sum();
    return total
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(part1("input/01_train"), common::read_lines_as::<i32>("input/01_val1")[0]);
    println!();
    println!("Part1: {}", part1("input/01_test"));

    // Test part-2 solver, then apply to real input.
    assert_eq!(part2("input/01_train"), common::read_lines_as::<i32>("input/01_val2")[0]);
    println!("Part2: {}", part2("input/01_test"));

}
