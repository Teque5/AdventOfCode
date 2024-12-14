use ab_glyph::{FontRef, PxScale};
use aoc;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use std::process::Command;

/// print robot positions to console
#[allow(dead_code)]
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

/// Update image img with new robot positions, write to file
fn draw_robots(img: &mut RgbImage, robots: &Vec<(i64, i64, i64, i64)>) {
    for pixel in img.pixels_mut() {
        pixel[0] = (pixel[0] as f32 * 0.95) as u8;
        // pixel[1] = (pixel[0] as f32 / 4.0 + pixel[1] as f32 * 0.6) as u8;
        pixel[1] = 0;
        pixel[2] = 0;
    }
    // let mut max_intensity = 0f32;
    for robot in robots {
        let pixel = img.get_pixel_mut(robot.1 as u32, robot.0 as u32);
        let intensity =
            (((robot.3 as f32).powf(2.0) + (robot.2 as f32).powf(2.0)).sqrt() * 2.0) as u8;
        pixel[0] = intensity;
        pixel[1] = intensity;
        pixel[2] = intensity;
    }
}

/// Compute safety factor
fn safety_factor(robots: &Vec<(i64, i64, i64, i64)>, rows: i64, cols: i64) -> u64 {
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
        let numbers = aoc::parse_numbers::<i64>(&line);
        // row_pos, col_pos, row_vel, col_vel
        robots.push((numbers[1], numbers[0], numbers[3], numbers[2]));
    }
    let is_training = robots.len() == 12;
    let rows: i64 = if is_training { 7 } else { 103 };
    let cols: i64 = if is_training { 11 } else { 101 };
    // image related vars

    let font = FontRef::try_from_slice(include_bytes!("../SNESesque.ttf")).unwrap();
    let height = 13f32;
    let scale = PxScale {
        x: height,
        y: height,
    };

    let mut img = RgbImage::new(cols as u32 + 1, rows as u32 + 1);
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
        if second == 99 && is_part1 {
            // 99 is the 100th second!
            return safety_factor(&robots, rows, cols);
        }
        // save frames for GIF
        if second % 101 == 97 {
            let mut filename = format!("day14_{:05}.png", second);
            let safety = safety_factor(&robots, rows, cols);
            let txt = &format!("safety={}", safety);
            let txt_size = text_size(scale, &font, txt);
            draw_robots(&mut img, &robots);
            draw_filled_rect_mut(
                &mut img,
                Rect::at(0, 2).of_size(txt_size.0, txt_size.1),
                Rgb([0, 0, 0]),
            );
            draw_text_mut(&mut img, Rgb([255, 255, 255]), 0, 0, scale, &font, txt);
            img.save(filename).expect("save failed");
            // print_robots(&robots, rows, cols);
            if second == 7773 {
                for idx in second + 1..second + 30 {
                    filename = format!("day14_{:05}.png", idx);
                    draw_robots(&mut img, &robots);
                    draw_text_mut(&mut img, Rgb([255, 255, 255]), 0, 0, scale, &font, txt);
                    img.save(filename).expect("save failed");
                }
            }
        }
    }
    // compile frames to GIF
    Command::new("ffmpeg")
        .args([
            "-y",
            "-pattern_type",
            "glob",
            "-i",
            "day14*.png",
            "img/day14.gif",
        ])
        .output()
        .expect("ffmpeg merge failed");
    // delete raw frames
    Command::new("sh")
        .args(["-c", "rm day14*.png"])
        .output()
        .expect("error during rm");

    return 0;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 12);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    // in this case part2 generates the animated gif
    _ = part(&format!("input/{:02}_test", day), false);
    println!("Part2: 7773 (found experimentally)");
    println!("Coded: 83 Minutes");
}
