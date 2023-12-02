#[path = "common.rs"] mod common;

fn part1(filename: &str) -> u64 {
    let (mut square, dim) = common::read_lines_2d(filename);
    let dim = 3usize;
    // square[(0, 0)] = 7;
    for rdx in 0..dim {
        for cdx in 0..dim {
            // println!("a");
            println!("{}", square[(rdx, cdx)]);
        }
    }
    return 1;
}

// fn part2(filename: &str) -> u64 {

// }

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part1("input/02_train"),
        common::read_lines_as::<u64>("input/02_val1")[0]
    );
    // println!("Part1: {}", part1("input/02_test"));

    // // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part2("input/02_train"),
    //     common::read_lines_as::<u64>("input/02_val2")[0]
    // );
    // println!("Part2: {}", part2("input/02_test"));
}
