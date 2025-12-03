use aoc;

/// Lobby
/// Pick highest joltage by pairing numbers from each row
fn part(filename: &str, is_part1: bool) -> usize {
    let mut joltage = 0usize;
    // parse info
    let (banks, rows, cols) = aoc::read_2d_as::<usize>(filename);
    aoc::print_2d(&banks);

    if is_part1 {
        for rdx in 0..rows {
            let mut maxval = 0usize;
            let mut maxcdx = 0usize;
            // in each row find the largest value
            for cdx in 0..cols {
                let val = banks[[rdx, cdx]];
                if val > maxval {
                    maxval = val;
                    maxcdx = cdx;
                }
                if maxval == 9 {
                    break;
                }
            }
            // now from maxcdx find next largest value left and right
            // left first
            let mut left_max = 0usize;
            let mut left_ok = false;
            for cdx in (0..maxcdx).rev() {
                let val = banks[[rdx, cdx]];
                if val > left_max {
                    left_max = val;
                    left_ok = true;
                }
                if val == maxval {
                    break;
                }
            }
            // right next
            let mut right_max = 0usize;
            let mut right_ok = false;
            for cdx in (maxcdx + 1)..cols {
                let val = banks[[rdx, cdx]];
                if val > right_max {
                    right_max = val;
                    right_ok = true;
                }
                if val == maxval {
                    break;
                }
            }

            let mut leftpair = left_max * 10 + maxval;
            let mut rightpair = maxval * 10 + right_max;
            if !left_ok {
                leftpair = 0;
            }
            if !right_ok {
                rightpair = 0;
            }

            if leftpair >= rightpair {
                joltage += leftpair;
                println!("Chose left pair.");
            } else {
                joltage += rightpair;
                println!("Chose right pair.");
            }
            println!(
                "Row {}: leftpair={}, rightpair={}.",
                rdx, leftpair, rightpair
            );
        }
    } else {
        // part 2 we use 12 digits instead of 2
        // create 12 digit buffer array

        for rdx in 0..rows {
            let mut buffer: Vec<usize> = vec![0; 12];
            // left edge will move rightwards
            let mut left_edge = 0usize;
            for digit in 0..12 {
                let mut maxval = 0usize;
                let mut maxcdx = usize::MAX;
                let right_edge = cols - (12 - digit);
                // iterate backwards from right edge-12 to left edge
                for cdx in (left_edge..=right_edge).rev() {
                    // find largest digit
                    if banks[[rdx, cdx]] >= maxval {
                        maxval = banks[[rdx, cdx]];
                        maxcdx = cdx;
                        buffer[digit] = maxval;
                        left_edge = maxcdx + 1;
                    }
                }
                // println!("Row {}: maxval={} at cdx={}.", rdx, maxval, maxcdx);
            }
            // convert buffer to number
            let mut number = 0usize;
            for digit in 0..12 {
                number = number * 10 + buffer[digit];
            }
            joltage += number;
        }

    }

    return joltage;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 357);
    println!("  Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        3121910778619
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 44 minutes.");
}
