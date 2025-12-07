use aoc;
use ndarray::Array2;

/// Lobby
/// Ostensibly pick highest joltage
/// Actually find highest digits by seeking backwards
fn part(filename: &str, is_part1: bool) -> usize {
    let mut joltage = 0usize;
    // parse info
    let (banks, rows, cols) = aoc::read_2d_as::<usize>(filename);
    let num_digits = if is_part1 { 2usize } else { 12usize };
    // let mut img = aoc::Image::new(rows + 1, cols + 1 + num_digits);
    // img.set_alpha(0.2);
    // img.set_scale(8);
    // img.draw_chars(&banks);
    // img.render_frame();
    let mut buffer = Array2::<usize>::zeros((rows, num_digits));
    for rdx in 0..rows {
        // left edge will scan right
        let mut left_edge = 0usize;
        for digit in 0..num_digits {
            let mut maxval = 0usize;
            let mut maxcdx;
            let right_edge = cols - (num_digits - digit);
            // iterate backwards from right edge-12 to left edge
            for cdx in (left_edge..=right_edge).rev() {
                // find largest digit
                if banks[[rdx, cdx]] >= maxval {
                    maxval = banks[[rdx, cdx]];
                    maxcdx = cdx;
                    buffer[[rdx, digit]] = maxval;
                    left_edge = maxcdx + 1;
                }
                // for ((row, col), value) in buffer.indexed_iter() {
                //     if *value != 0 {
                //         img.draw_text(
                //             row,
                //             cols + col + 1,
                //             &char::from_u32(0x30 + *value as u32).unwrap().to_string(),
                //         );
                //     }
                // }
                // img.draw_chars(&banks);
                // img.draw_bool(rdx, cdx, true);
                // img.draw_text(rows, 0, &format!("Joltage={}", joltage));
                // img.render_frame();
                // img.fade();
            }
        }
        // convert buffer to number
        let mut number = 0usize;
        for digit in 0..num_digits {
            number = number * 10 + buffer[[rdx, digit]];
        }
        joltage += number;
    }
    // for _ in 0..30 {
    //     img.draw_text(rows, 0, &format!("Joltage={}", joltage));
    //     img.render_frame();
    //     img.fade();
    // }
    // img.render_gif("img/day03.gif");
    return joltage;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 357);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        3121910778619
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 44 minutes.");
}
