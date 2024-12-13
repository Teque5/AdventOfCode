use aoc;

/// Determine if a row is safe
fn is_safe(row: &Vec<isize>) -> bool {
    let mut sign;
    let mut safe = true;
    let mut prev = -1isize;
    let mut prevsign = true;
    for (idx, &val) in row.iter().enumerate() {
        if prev == -1 {
            prev = val;
            continue;
        } else {
            sign = (prev - val) < 0;
            let diff = (prev - val).abs();
            // print!("new={} diff={} pos={}", val, diff, sign);
            if idx > 1 {
                // check sign
                if sign != prevsign {
                    safe = false;
                    break;
                }
            }
            // check diff
            if diff == 0 || diff > 3 {
                safe = false;
                break;
            }
            prev = val;
            prevsign = sign;
        }
    }
    return safe;
}

/// Count Safe Reports
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;

    // parse info
    let lines = aoc::read_lines(filename);
    for line in lines {
        let row = aoc::parse_numbers(&line);
        if is_safe(&row) {
            acc += 1;
        } else if !is_part1 {
            // problem dampener makes it safe if we remove any one value
            let rowlen = row.len();
            for idx in 0..rowlen {
                let newrow = [&row[..idx], &row[idx + 1..rowlen]].concat();
                if is_safe(&newrow) {
                    acc += 1;
                    break;
                }
            }
        }
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 2);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 4);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 35 minutes");
}
