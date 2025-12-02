use aoc;

/// Dial Password, Count Zeros on Circular Dial
fn part(filename: &str, is_part1: bool, render: bool) -> usize {
    let mut acc = 0usize; // zero count
    let mut dial = 50isize; // dial position
    let mut turn: isize; // change amount
    let mut direction: isize; // change direction
    let mut img = aoc::Image::new(1, 100);
    img.set_frameskip(5);

    if render {
        // fancy plot for part 2
        img.draw_bool(0, dial as usize, true);
        img.draw_text(0, 0, &format!("Zeros: {}", acc));
        img.render_frame();
        img.info();
    }

    // parse info
    let lines = aoc::read_lines(filename);
    for line in lines {
        turn = aoc::parse_numbers::<isize>(&line)[0];
        if line.chars().next().unwrap() == 'L' {
            direction = -1;
        } else {
            direction = 1;
        }
        if is_part1 {
            dial += turn * direction;
            while dial < 0 {
                dial += 100;
            }
            while dial >= 100 {
                dial -= 100;
            }
            if dial == 0 {
                acc += 1;
            }
        } else {
            for _ in 0..turn {
                dial += direction;
                if dial < 0 {
                    dial = 99;
                } else if dial >= 100 {
                    dial = 0;
                }
                if dial == 0 {
                    acc += 1;
                }
                if render {
                    img.fade();
                    img.draw_bool(0, dial as usize, true);
                    img.draw_text(0, 0, &format!("Zeros: {}", acc));
                    img.render_frame();
                }
            }
        }
    }

    if render {
        for _ in 0..5 * 15 {
            img.fade();
            img.draw_text(0, 0, &format!("Zeros: {}", acc));
            img.render_frame();
        }
        img.render_webp(&"img/day01.webp");
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true, false), 3);
    println!(
        "Part1: {}",
        part(&format!("input/{:02}_test", day), true, false)
    );

    assert_eq!(part(&format!("input/{:02}_train", day), false, true), 6);
    println!(
        "Part2: {}",
        part(&format!("input/{:02}_test", day), false, false)
    );

    println!("Coded: 88 minutes; out of practice.");
}
