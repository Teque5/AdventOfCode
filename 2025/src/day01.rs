use aoc;

/// Dial Password, Count Zeros on Circular Dial
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;
    let mut dial = 50isize;
    let mut prev: isize;

    // parse info
    let lines = aoc::read_lines(filename);
    for line in lines {
        let row = aoc::parse_numbers::<isize>(&line);
        // set left true if first char is L
        let left = line.chars().next().unwrap() == 'L';
        // println!("row: {:?} left: {}", row, left);
        prev = dial;
        if left {
            dial -= row[0];
        } else {
            dial += row[0];
        }
        if is_part1 {
            while dial < 0 {
                dial += 100;
            }
            while dial >= 100 {
                dial -= 100;
            }
            // println!("{:<3}  dial: {}", line, dial);
            if dial == 0 {
                acc += 1;
            }
        } else {
            loop {
                // println!(" prev: {}", prev);

                if dial == 0 {
                    // tick at zero
                    acc += 1;
                    // print!("*");
                    break;
                } else if dial > 0 && dial < 100 {
                    // ok
                    break;
                } else if dial < -100 {
                    // must cross zero once
                    dial += 100;
                    acc += 1;
                    // print!("-");
                } else if dial < 0 {
                    dial += 100;
                    if prev != 0 {
                        acc += 1;
                        // print!("-");
                    }
                } else if dial > 100 {
                    // must cross zero once
                    dial -= 100;
                    acc += 1;
                    // print!("+");
                } else if dial == 100 {
                    dial -= 100;
                }
            }
        }
        // println!("{:<3}  dial: {}", line, dial);
    }
    // not 6392, 6291, 5396 too low, not 5828
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 3);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 6);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 88 minutes; out of practice.");
}
