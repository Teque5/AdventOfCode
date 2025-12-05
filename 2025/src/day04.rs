use aoc;
use ndarray::Array2;

/// Count positions with '@' that have fewer than 4 adjacent '@' neighbors
fn remove_rolls(map: &mut Array2<char>, rows: usize, cols: usize) -> usize {
    let mut acc = 0usize;
    let mut map_changes = Vec::new();

    // count and collect positions to modify
    for rdx in 0..rows {
        for cdx in 0..cols {
            let position = (rdx, cdx);
            let mut rolls = 0usize;
            if map[position] == '@' {
                for direction in aoc::EIGHT_DIRECTIONS {
                    let new_position = (
                        position.0 as isize + direction.0,
                        position.1 as isize + direction.1,
                    );
                    // check if inside map
                    if aoc::is_in_map(rows, cols, new_position) {
                        let new_usize_position = (new_position.0 as usize, new_position.1 as usize);
                        if map[new_usize_position] == '@' {
                            rolls += 1;
                        }
                    }
                }
                // println!("@ {:?} {}", position, rolls);
                if rolls < 4 {
                    map_changes.push(position);
                    acc += 1;
                }
            }
        }
    }

    // apply map changes
    for pos in map_changes {
        map[pos] = ' ';
    }

    acc
}

/// Printing Department
fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let (mut map, rows, cols) = aoc::read_2d_as::<char>(filename);
    // img for part 2 test data
    // let mut img = aoc::Image::new(rows, cols);
    // img.set_scale(2);
    // img.draw_bools(&map);
    // img.render_frame();
    // img.set_frameskip(3);
    // img.set_alpha(0.01);

    if is_part1 {
        return remove_rolls(&mut map, rows, cols);
    }
    // part 2
    let mut acc = 0usize;
    let mut acc_prev = usize::MAX;
    while acc != acc_prev {
        acc_prev = acc;
        acc += remove_rolls(&mut map, rows, cols);
        // img.fade();
        // img.draw_bools(&map);
        // img.render_frame();
    }
    // for _ in 0..15 {
    //     img.fade();
    //     img.draw_bools(&map);
    //     img.render_frame();
    // }
    // img.render_gif("img/day04.gif");
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 13);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 43);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 35 minutes.");
}
