use aoc;

/// Trash Compactor
/// Probably the least fun AoC puzzle ever
fn part1(filename: &str) -> usize {
    let mut acc = 0usize;
    // parse info
    let lines = aoc::read_lines(filename);
    let mut grid: Vec<Vec<usize>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    for line in lines {
        // if line contains digits, push to grid
        if line.chars().any(|c| c.is_digit(10)) {
            let numbers = aoc::parse_numbers::<usize>(&line);
            grid.push(numbers);
        } else {
            // save chars separated by spaces into operators
            for c in line.chars() {
                if c != ' ' {
                    operators.push(c);
                }
            }
        }
    }
    for cdx in 0..operators.len() {
        match operators[cdx] {
            '+' => {
                for row in &grid {
                    acc += row[cdx];
                }
            }
            '*' => {
                let mut subtotal = 1;
                for row in &grid {
                    subtotal *= row[cdx];
                }
                acc += subtotal;
            }
            _ => {}
        }
    }
    return acc;
}

/// Cephalapod math entirely diffent, no reuse
/// Scan last row backwards for operators, then parse MSB to LSB numbers in columns above
/// Whoever made this puzzle should feel bad.
fn part2(filename: &str) -> usize {
    let mut acc = 0usize;
    let (grid, rows, cols) = aoc::read_2d_as::<char>(filename);
    // aoc::print_2d(&grid);

    // scan backwards over last row of grid for operators
    let mut prev_cdx = cols;
    for cdx in (0..cols).rev() {
        match grid[[rows - 1, cdx]] {
            '+' => {
                for subcdx in (cdx..prev_cdx).rev() {
                    let mut digits = Vec::new();
                    for rdx in 0..(rows - 1) {
                        // check if digit or space
                        if grid[[rdx, subcdx]] != ' ' {
                            let digit = grid[[rdx, subcdx]].to_digit(10).unwrap() as usize;
                            digits.push(digit);
                        }
                    }
                    let mut subtotal = 0;
                    for (ddx, digit) in digits.iter().enumerate() {
                        subtotal += digit * 10usize.pow((digits.len() - 1 - ddx) as u32);
                    }
                    // println!("+++ for col {}: {}.", subcdx, subtotal);
                    prev_cdx = cdx;
                    acc += subtotal;
                }
            }
            '*' => {
                let mut product = 1;
                for subcdx in (cdx..prev_cdx).rev() {
                    let mut digits = Vec::new();
                    for rdx in 0..(rows - 1) {
                        // check if digit or space
                        if grid[[rdx, subcdx]] != ' ' {
                            let digit = grid[[rdx, subcdx]].to_digit(10).unwrap() as usize;
                            digits.push(digit);
                        }
                    }
                    let mut subtotal = 0;
                    for (ddx, digit) in digits.iter().enumerate() {
                        subtotal += digit * 10usize.pow((digits.len() - 1 - ddx) as u32);
                    }
                    // println!("||| for col {}: {}.", subcdx, subtotal);
                    prev_cdx = cdx;
                    if digits.len() > 0 {
                        product *= subtotal;
                    }
                }
                acc += product;
            }
            _ => {}
        }
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part1(&format!("input/{:02}_train", day)), 4277556);
    println!("Part1: {}", part1(&format!("input/{:02}_test", day)));

    assert_eq!(part2(&format!("input/{:02}_train", day)), 3263827);
    println!("Part2: {}", part2(&format!("input/{:02}_test", day)));

    println!("Coded: 62 minutes.");
}
