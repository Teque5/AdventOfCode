#[path = "common.rs"]
mod common;

/// Cosmic Expansion
fn part(filename: &str, is_part1: bool) -> usize {
    let mut newrdx: Vec<usize> = Vec::new();
    let mut newcdx: Vec<usize> = Vec::new();

    // parse info
    let (universe, rows, cols) = common::read_2d_chars(filename);

    // figure out where row expansion occurs
    for rdx in 0..rows {
        let mut empty = true;
        for cdx in 0..cols {
            if universe[(rdx, cdx)] == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            newrdx.push(rdx);
        }
    }
    // figure out where col expansion occurs
    for cdx in 0..cols {
        let mut empty = true;
        for rdx in 0..rows {
            if universe[(rdx, cdx)] == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            newcdx.push(cdx);
        }
    }

    // create universe expanded 2d vector
    // turns out this wasn't necessary to solve the puzzle
    // let mut expanded = Array2::from_elem((rows + newrdx.len(), cols + newcdx.len()), '.');
    let mut galaxy_positions: Vec<(usize, usize)> = Vec::new();
    let mut r_extra = 0;
    for rdx in 0..rows {
        if newrdx.contains(&rdx) {
            if is_part1 {
                r_extra += 1;
            } else {
                r_extra += 1_000_000 - 1;
            }
        }
        let mut c_extra = 0;
        for cdx in 0..cols {
            if newcdx.contains(&cdx) {
                if is_part1 {
                    c_extra += 1;
                } else {
                    c_extra += 1_000_000 - 1;
                }
            }

            if universe[(rdx, cdx)] == '#' {
                let pos = (rdx + r_extra, cdx + c_extra);
                // expanded[pos] = universe[(rdx, cdx)];
                galaxy_positions.push(pos);
            }
        }
    }

    let mut acc = 0usize;
    let combos = combinations(galaxy_positions);
    for (pos1, pos2) in combos.iter() {
        acc += (pos1.0 as i32 - pos2.0 as i32).abs() as usize;
        acc += (pos1.1 as i32 - pos2.1 as i32).abs() as usize;
    }

    // common::print_2d_chars(&universe);

    return acc;
}

/// n-choose-2 combinations given a vec of tuples
fn combinations<T: Clone>(items: Vec<T>) -> Vec<(T, T)> {
    let mut combinations = Vec::new();

    for i in 0..items.len() {
        for j in i + 1..items.len() {
            combinations.push((items[i].clone(), items[j].clone()));
        }
    }

    return combinations;
}

pub fn solve() {
    let day: usize = 11;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 62 minutes");
}
