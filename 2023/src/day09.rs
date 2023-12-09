#[path = "common.rs"]
mod common;

fn diff(ray: Vec<isize>) -> Vec<isize> {
    ray.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

/// Oasis And Sand Instability Sensor
fn part(filename: &str, is_part1: bool) -> isize {
    let mut acc = 0isize;

    // parse info
    let lines = common::read_lines(filename);
    for line in lines {
        let mut triangle_row = common::parse_numbers_isize(&line);
        let mut alpha_points: Vec<isize> = Vec::new();
        let mut omega_points: Vec<isize> = Vec::new();
        let mut current_len = triangle_row.len() - 1;
        // println!("start {:?}", vals);
        loop {
            omega_points.push(triangle_row[current_len]);
            alpha_points.push(triangle_row[0]);
            triangle_row = diff(triangle_row);
            // println!("somed {:?}", triangle_row);
            current_len -= 1;
            if triangle_row.iter().sum::<isize>() == 0 {
                break;
            }
        }
        if is_part1 {
            // add up the right side of the triangle
            acc += omega_points.iter().sum::<isize>();
        } else {
            // add, subtract sequentially from left side of the triangle
            for idx in 0..alpha_points.len() {
                let sign = if idx % 2 == 0 { 1isize } else { -1isize };
                acc += alpha_points[idx] * sign;
            }
        }
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
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 118 minutes");
}
