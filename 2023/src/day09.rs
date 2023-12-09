#[path = "common.rs"]
mod common;

fn diff(ray: Vec<isize>) -> Vec<isize> {
    ray.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

/// Oasis And Sand Instability Sensor
fn part(filename: &str, is_part1: bool) -> isize {
    let mut acc = 0isize;

    // parse info
    let mut vals: Vec<isize> = Vec::new();
    let lines = common::read_lines(filename);
    for line in lines {
        vals = common::parse_numbers_isize(&line);
        let mut depth = 0usize;
        let mut some_diff = vals.clone();
        let mut steps: Vec<isize> = Vec::new();
        let mut endpoints: Vec<isize> = Vec::new();
        let mut current_len = some_diff.len() - 1;
        println!("start {:?}", vals);
        loop {
            // println!("pushing {}", some_diff[current_len]);
            endpoints.push(some_diff[current_len]);
            steps.push(some_diff[current_len] - some_diff[current_len - 1]);
            some_diff = diff(some_diff);
            println!("somed {:?}", some_diff);
            // println!("steps {:?}", steps);
            depth += 1;
            current_len -= 1;
            if some_diff.iter().sum::<isize>() == 0 {
                break;
            }
        }
        acc += endpoints.iter().sum::<isize>();
        println!(
            "acc={}   d{:?} >>> {}",
            acc,
            depth,
            endpoints.iter().sum::<isize>()
        );
    }

    return acc;
}

pub fn solve() {
    let day: usize = 9;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<isize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        common::read_lines_as::<isize>(&format!("input/{:02}_val2", day))[0]
    );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: xxx minutes");
}
