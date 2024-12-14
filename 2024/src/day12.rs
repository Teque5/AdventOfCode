use aoc;
use ndarray::Array2;

const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // Up
    (1, 0),  // Down
    (0, -1), // Left
    (0, 1),  // Right
];

#[rustfmt::skip]
fn is_in_map(rows: usize, cols: usize, position: (isize, isize)) -> bool {
    position.0 >= 0
    && position.1 >= 0
    && position.0 < rows as isize
    && position.1 < cols as isize
}

/// are a pair of positions within the same row or column?
fn same_row_or_same_column(pair: &Vec<(usize, usize)>) -> bool {
    pair[0].0 == pair[1].0 || pair[0].1 == pair[1].1
}

/// return array of valid directions from some position
fn matching_directions(
    garden: &Array2<char>,
    position: &(usize, usize),
    current: &char,
    rows: usize,
    cols: usize,
) -> Vec<(usize, usize)> {
    let mut possible: Vec<(usize, usize)> = Vec::new();
    for direction in DIRECTIONS {
        let new_position = (
            position.0 as isize + direction.0,
            position.1 as isize + direction.1,
        );
        if is_in_map(rows, cols, new_position) {
            let new_usize_position = (new_position.0 as usize, new_position.1 as usize);
            if *current == garden[new_usize_position] {
                possible.push(new_usize_position);
            }
        }
    }
    return possible;
}

// fn matching_directions_in_region(
//     position: &(usize, usize),
//     region: &Array2<bool>,
//     rows: usize,
//     cols: usize,
// ) -> Vec<(usize, usize)> {
//     let mut possible: Vec<(usize, usize)> = Vec::new();
//     for direction in DIRECTIONS {
//         let new_position = (
//             position.0 as isize + direction.0,
//             position.1 as isize + direction.1,
//         );
//         if is_in_map(rows, cols, new_position) {
//             let new_usize_position = (new_position.0 as usize, new_position.1 as usize);
//             if region[new_usize_position] {
//                 possible.push(new_usize_position);
//             }
//         }
//     }
//     return possible;
// }

/// Garden Groups
/// For part2 the trick is that num_corners == num_sides
fn part(filename: &str, is_part1: bool) -> usize {
    let mut price = 0usize;
    // parse info
    let (garden, rows, cols) = aoc::read_2d_chars(filename);
    // keep track of where we have already processed
    let mut traversed: Array2<bool> = Array2::from_elem((rows, cols), false);
    for rdx in 0..rows {
        for cdx in 0..cols {
            if traversed[(rdx, cdx)] {
                continue;
            }
            // process garden from new position
            traversed[(rdx, cdx)] = true;
            let region_name = garden[(rdx, cdx)];
            // keep track of this particular region for part 2 where we need to count vertices
            let mut this_region: Array2<bool> = Array2::from_elem((rows, cols), false);
            this_region[(rdx, cdx)] = true;
            let mut area = 1usize;
            let mut perimeter = 4usize;
            let mut corners = 0usize;
            let mut possible = matching_directions(&garden, &(rdx, cdx), &region_name, rows, cols);
            while let Some(position) = possible.pop() {
                if traversed[position] {
                    continue;
                }
                // grow garden to this position
                traversed[position] = true;
                this_region[position] = true;
                // how many other directions from here match current garden?
                let new_possibilities =
                    matching_directions(&garden, &position, &region_name, rows, cols);
                let mut num_traversed_adjacent = 0usize;
                for poss in &new_possibilities {
                    if traversed[*poss] {
                        num_traversed_adjacent += 1;
                    }
                }
                match num_traversed_adjacent {
                    0 => {} // no perimeter change
                    1 => perimeter += 2,
                    2 => {} // no perimeter change
                    3 => perimeter -= 2,
                    4 => perimeter -= 4,
                    _ => panic!("not possible"),
                }
                area += 1;
                // println!("new poss={:?}", new_possibilities);
                // println!("check {:?} {} {}", position, num_traversed_adjacent, perimeter);
                // for poss in new_possibilities {
                //     if !traversed[poss] {
                //         possible.push(poss);
                //     }
                // }
                possible.extend(new_possibilities);
            }
            if is_part1 {
                price += area * perimeter;
            } else {
                // now iterate over this region and find corners
                for subrdx in 0..rows {
                    for subcdx in 0..cols {
                        let contiguous = this_region[(subrdx, subcdx)];
                        let is_region = garden[(subrdx, subcdx)] == region_name;
                        // if garden[(subrdx, subcdx)] != region_name {
                        //     continue;
                        // }
                        let region_match = matching_directions(
                            &garden,
                            &(subrdx, subcdx),
                            &region_name,
                            rows,
                            cols,
                        );
                        match region_match.len() {
                            0 => {
                                if contiguous {
                                    corners += 4;
                                }
                            }
                            1 => {
                                if contiguous {
                                    corners += 2;
                                }
                            }
                            2 => {
                                let both_contiguous =
                                    this_region[region_match[0]] && this_region[region_match[1]];
                                if is_region
                                    && both_contiguous
                                    && !same_row_or_same_column(&region_match)
                                {
                                    // exterior corner
                                    println!("ext");
                                    corners += 1
                                } else if !is_region
                                    && both_contiguous
                                    && !same_row_or_same_column(&region_match)
                                {
                                    // println!("int");
                                    // avoid mobius corner
                                    let mut mobius = false;
                                    if region_match[0].0 == subrdx {
                                        // the first match is in the same row
                                        let opposite = (region_match[1].0, region_match[0].1);
                                        // println!("opposite {:?}", opposite);
                                        if !this_region[opposite] {
                                            mobius = true;
                                        }
                                    } else {
                                        // the first match is in the same column
                                        let opposite = (region_match[0].0, region_match[1].1);
                                        // println!("opposite {:?}", opposite);
                                        if !this_region[opposite] {
                                            mobius = true;
                                        }
                                    }

                                    if !mobius {
                                        println!("not mobius->+1");
                                        // interior corner
                                        corners += 1
                                    }
                                }
                            }
                            3 => {
                                if !contiguous
                                    && this_region[region_match[0]]
                                    && this_region[region_match[1]]
                                    && this_region[region_match[2]]
                                {
                                    corners += 2;
                                }
                            }
                            4 => {
                                if !contiguous
                                    && this_region[region_match[0]]
                                    && this_region[region_match[1]]
                                    && this_region[region_match[2]]
                                    && this_region[region_match[3]]
                                {
                                    // we are surrounding a blank spot
                                    corners += 4;
                                }
                            }
                            _ => panic!("not possible"),
                        }
                        // if region_match.len() > 0 {
                        //     println!("check {:?} -> {}", (subrdx, subcdx), corners);
                        // }
                    }
                }
                price += area * corners;
            }
            println!(
                "{} area={} perim={} corners={}",
                garden[(rdx, cdx)],
                area,
                perimeter,
                corners
            );

            // println!("@({},{}) possible={:?}", rdx, cdx, possible);
        }
    }
    return price;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    // assert_eq!(part(&format!("input/{:02}_train0", day), true), 140);
    // assert_eq!(part(&format!("input/{:02}_train1", day), true), 772);
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // // Test part-2 solver, then apply to real input.
    // assert_eq!(part(&format!("input/{:02}_train0", day), false), 80);
    // assert_eq!(part(&format!("input/{:02}_train1", day), false), 436);
    // assert_eq!(part(&format!("input/{:02}_train2", day), false), 236);
    // assert_eq!(part(&format!("input/{:02}_train3", day), false), 1206);
    // assert_eq!(part(&format!("input/{:02}_train4", day), false), 368);
    assert_eq!(part(&format!("input/{:02}_train5", day), false), 414);
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), false),
    //     aoc::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    // );
    // assert_eq!(
    //     part(&format!("input/{:02}_train_extra", day), false),
    //     aoc::read_lines_as::<usize>(&format!("input/{:02}_val_extra", day))[0]
    // );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: 120+120+60+ Minutes");
    // 7431260 is too high
    // 867192 is wrong
    // 855693 is wrong
    // 855497 is wrong
    // 849322 target
    // 842136 is too low
}
