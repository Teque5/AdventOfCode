use aoc;

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

    // create image to draw robots upon
    let mut img = aoc::Image::new(rows as usize, cols as usize);
    // img.set_fontsize(1.0); // create very small image for github
    img.set_framerate(20);
    // simulate robots
    for second in 0..7777 {
        for rdx in 0..robots.len() {
            robots[rdx].0 = (robots[rdx].0 + robots[rdx].2) % rows;
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
        // render frames for GIF
        if second % 101 == 97 {
            let safety = safety_factor(&robots, rows, cols);
            let safety_txt = format!("safety = {}", safety);
            let frame_txt = format!("second = {}", second);
            for robot in &robots {
                img.draw_bool(robot.0 as usize, robot.1 as usize, true);
            }
            img.draw_text(0, 0, &safety_txt);
            img.draw_text(1, 0, &frame_txt);
            img.render_frame();
            // render final GIF
            if second == 7773 {
                for _ in 0..15 {
                    // render extra fade-out frames w/ robots in current position
                    for robot in &robots {
                        img.draw_bool(robot.0 as usize, robot.1 as usize, true);
                    }
                    img.render_frame();
                    // img.save_frame(&"out.png");
                    img.fade()
                }
                img.render_gif(&"img/day14.gif");
                // img.render_webp(&"img/day14.webp");
            }
            img.fade();
        }
    }
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
