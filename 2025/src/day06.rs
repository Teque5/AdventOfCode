use aoc;

/// ???
fn part(filename: &str, is_part1: bool) -> usize {
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

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 4277556);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // assert_eq!(part(&format!("input/{:02}_train", day), false), 14);
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: xx minutes.");
}
