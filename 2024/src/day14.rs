use aoc;
use image::{Rgb, RgbImage};

/// print robot positions
fn print_robots(robots: &Vec<(i64, i64, i64, i64)>, rows: i64, cols: i64) {
    let mut buffer: Vec<Vec<char>> = vec![vec![' '; cols as usize]; rows as usize];
    for robot in robots {
        buffer[robot.0 as usize][robot.1 as usize] = '█';
    }
    for row in &buffer {
        let row_str: String = row.iter().collect();
        println!("{}", row_str);
    }
}

#[allow(dead_code)]
fn save_robots(robots: &Vec<(i64, i64, i64, i64)>, rows: i64, cols: i64, filename: String) {
    // let mut buffer: Vec<Vec<char>> = vec![vec![' '; cols as usize]; rows as usize];
    let mut buffer = RgbImage::new(cols as u32 + 1, rows as u32 + 1);

    for robot in robots {
        buffer.put_pixel(robot.1 as u32, robot.0 as u32, Rgb([255, 0, 0]));
    }
    let _ = buffer.save(filename);
}

/// Count robots in quadrants
fn count_robots(robots: &Vec<(i64, i64, i64, i64)>, rows: i64, cols: i64) -> u64 {
    let mut quads = (0u64, 0u64, 0u64, 0u64);
    for robot in robots {
        if robot.0 == rows / 2 || robot.1 == cols / 2 {
            // robot straddling quadrants, do nothing
        } else if robot.0 < rows / 2 && robot.1 < cols / 2 {
            // top left
            quads.0 += 1;
        } else if robot.0 < rows / 2 && robot.1 > cols / 2 {
            // top right
            quads.1 += 1;
        } else if robot.0 > rows / 2 && robot.1 < cols / 2 {
            // bottom left
            quads.2 += 1;
        } else {
            quads.3 += 1;
        }
    }
    return quads.0 * quads.1 * quads.2 * quads.3;
}

/// Restroom Redoubt
///
/// ┌────► x=col
/// │
/// │
/// ▼
/// y=row
///
/// Keep track of robot positions
/// Use (row, col) instead of (x ,y)
fn part(filename: &str, is_part1: bool) -> u64 {
    // parse robots
    let lines = aoc::read_lines(filename);
    let mut robots: Vec<(i64, i64, i64, i64)> = Vec::new();
    for line in lines {
        let numbers = aoc::parse_numbers(&line);
        // row_pos, col_pos, row_vel, col_vel
        robots.push((
            numbers[1] as i64,
            numbers[0] as i64,
            numbers[3] as i64,
            numbers[2] as i64,
        ));
    }
    let is_training = robots.len() == 12;
    let rows: i64 = if is_training { 7 } else { 103 };
    let cols: i64 = if is_training { 11 } else { 101 };
    // simulate robots
    for second in 0..7777 {
        for rdx in 0..robots.len() {
            // row = (row + row_vel) % rows
            robots[rdx].0 = (robots[rdx].0 + robots[rdx].2) % rows;
            // col = (col + col_vel) % cols
            robots[rdx].1 = (robots[rdx].1 + robots[rdx].3) % cols;
            if robots[rdx].0 < 0 {
                robots[rdx].0 += rows;
            }
            if robots[rdx].1 < 0 {
                robots[rdx].1 += cols;
            }
        }
        // 7773 7773 7773
        if second == 100 && is_part1 {
            return count_robots(&robots, rows, cols);
        }
        if second % 101 == 97 {
            // enable to save frames for GIF
            // save_robots(&robots, rows, cols, format!("day14_{:05}.png", second));
        }
        if second == 7773 {
            print_robots(&robots, rows, cols);
        }
    }

    return 0;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 12);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    _ = part(&format!("input/{:02}_test", day), false);
    println!("Part2: 7773 (found experimentally)");
    println!("Coded: 83 Minutes");
}
