#[path = "common.rs"]
mod common;
use ndarray::Array2;

/// Dig Plan
#[allow(unused_assignments)]
fn part(filename: &str, is_part1: bool) -> isize {
    let mut gridsize_neg = (0isize, 0isize);
    let mut gridsize_pos = (0isize, 0isize);
    let mut directions: Vec<char> = Vec::new();
    let mut amounts: Vec<usize> = Vec::new();

    // parse info
    let lines = common::read_lines(filename);
    let mut position = (0isize, 0isize);
    for line in lines {
        let mut line_split = line.split_whitespace();
        let mut direction: char = 'z';
        let mut amount: usize = 0;
        if is_part1 {
            direction = line_split.next().unwrap().chars().next().unwrap();
            directions.push(direction);
            amount = common::parse_numbers(line_split.next().unwrap())[0];
            amounts.push(amount);
        } else {
            let rawhex: Vec<char> = line_split.nth(2).unwrap().chars().collect();
            match rawhex.get(rawhex.len() - 2).unwrap() {
                '0' => direction = 'R',
                '1' => direction = 'D',
                '2' => direction = 'L',
                '3' => direction = 'U',
                _ => panic!("no"),
            }
            let hexstr: String = rawhex[2..rawhex.len() - 2].iter().collect();
            amount = usize::from_str_radix(&hexstr, 16).unwrap();
            println!("{:?} {}", direction, amount);
        }

        // determine grid size
        match direction {
            'U' => position.0 -= amount as isize,
            'D' => position.0 += amount as isize,
            'L' => position.1 -= amount as isize,
            'R' => position.1 += amount as isize,
            _ => panic!("not possible"),
        }
        // println!("{:?}", position);
        // does grid need to expand?
        gridsize_neg.0 = std::cmp::min(gridsize_neg.0, position.0);
        gridsize_neg.1 = std::cmp::min(gridsize_neg.1, position.1);
        gridsize_pos.0 = std::cmp::max(gridsize_pos.0, position.0);
        gridsize_pos.1 = std::cmp::max(gridsize_pos.1, position.1);
    }

    let rows: usize = (gridsize_pos.0 - gridsize_neg.0) as usize + 1;
    let cols: usize = (gridsize_pos.1 - gridsize_neg.1) as usize + 1;

    let mut position: (usize, usize) = ((-gridsize_neg.0 as usize), (-gridsize_neg.1 as usize));
    if !is_part1 {
        panic!("part 2 not solvable yet; cannot allocate that much memory!");
    }

    // trace dig path in the grid
    let mut grid: Array2<char> = Array2::from_elem((rows, cols), '.');
    grid[position] = '#';
    for (direction, amount) in directions.iter().zip(&amounts) {
        // println!("{} p={:?} {} {}", sdx, position, direction, amount);
        for _ in 0..*amount {
            match direction {
                'U' => position.0 -= 1,
                'D' => position.0 += 1,
                'L' => position.1 -= 1,
                'R' => position.1 += 1,
                _ => panic!("not possible"),
            }
            grid[position] = '#';
        }
    }
    // common::print_2d_chars(&grid);
    // waterfill lava
    let mut filled = true;
    while filled {
        // keep calling this until waterfill is complete
        filled = waterfill(&mut grid, rows, cols);
    }
    // common::print_2d_chars(&grid);
    // count lava
    let mut lava = 0isize;
    for rdx in 0..rows {
        for cdx in 0..cols {
            if grid[(rdx, cdx)] != 'o' {
                lava += 1;
            }
        }
    }
    return lava;
}

/// waterfill to find inner/outer
fn waterfill(grid: &mut Array2<char>, rows: usize, cols: usize) -> bool {
    // set filled if any pixels were filled
    let mut filled = false;
    for rdx in 0..rows {
        for cdx in 0..cols {
            let pos = (rdx, cdx);
            if grid[pos] == '.' {
                if (pos.1 == 0) || (pos.0 == 0) || (pos.0 == rows - 1) || (pos.1 == cols - 1) {
                    // we are on the edge
                    grid[pos] = 'o';
                    filled = true;
                } else {
                    // TOP RIGHT BOTTOM LEFT
                    let trbl = vec![
                        grid[(rdx - 1, cdx)],
                        grid[(rdx, cdx + 1)],
                        grid[(rdx + 1, cdx)],
                        grid[(rdx, cdx - 1)],
                    ];
                    for char in trbl {
                        // adjacent is outside
                        if char == 'o' {
                            grid[pos] = 'o';
                            filled = true;
                        }
                    }
                }
            }
        }
    }

    return filled;
}

pub fn solve() {
    let day: usize = 18;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<isize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), false),
    //     common::read_lines_as::<isize>(&format!("input/{:02}_val2", day))[0]
    // );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // //started 2152
    // println!("Coded: xxx minutes");
}
