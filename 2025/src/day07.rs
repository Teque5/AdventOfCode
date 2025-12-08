use aoc;
use std::collections::HashSet;

/// Laboratories
/// Quantum Manifold
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;
    let mut possible = 1usize;
    // parse info
    let (mut manifold, rows, cols) = aoc::read_2d_as::<char>(filename);
    // copy manifold for part 2
    let mut manifold_backup = manifold.clone();


    // Quantum Manifold
    for rdx in 0..rows {
        for cdx in 0..cols {
            match manifold[[rdx, cdx]] {
                'S' | '|' => {
                    if rdx + 1 >= rows {
                        break;
                    }
                    // waterfall down
                    match manifold[[rdx + 1, cdx]] {
                        '.' => {
                            // empty space, continue down
                            manifold[[rdx + 1, cdx]] = '|';
                        }
                        '^' => {
                            let mut is_split = false;
                            // split beam
                            // check left
                            if manifold[[rdx + 1, cdx - 1]] == '.' {
                                manifold[[rdx + 1, cdx - 1]] = '|';
                                is_split = true;
                                // println!("Split at {},{}", rdx + 1, cdx - 1);
                                // possible += 1;
                            }
                            // check right
                            if manifold[[rdx + 1, cdx + 1]] == '.' {
                                manifold[[rdx + 1, cdx + 1]] = '|';
                                is_split = true;
                                // println!("Split at {},{}", rdx + 1, cdx + 1);
                                // possible += 1;
                            }
                            if is_split {
                                acc += 1;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // println!("row {} possible {}", rdx, possible);
    }
    // println!("Possible paths: {}", possible);
    aoc::print_2d(&manifold);
    if !is_part1  {
        // Quantum Tachyon Manifold
        // create hashset for paths
        let mut paths: HashSet<Vec<(usize, usize)>> = HashSet::new();
        let mut path = Vec::<(usize, usize)>::new();
        let mut growing = true;
        let mut set_size = 0usize;
        while growing {
            // println!("go");
            // run every possible configuration
            let mut rewind = manifold_backup.clone();
            for rdx in 0..rows {
                for cdx in 0..cols {
                    match rewind[[rdx, cdx]] {
                        'S' | '|' => {
                            if rdx + 1 >= rows {
                                break;
                            }
                            // waterfall down
                            match rewind[[rdx + 1, cdx]] {
                                '.' => {
                                    // empty space, continue down
                                    rewind[[rdx + 1, cdx]] = '|';
                                    path.push((rdx +1, cdx));
                                }
                                '^' | '>' => {
                                    // split beam left
                                    if rewind[[rdx + 1, cdx - 1]] == '.' {
                                        rewind[[rdx + 1, cdx - 1]] = '|';
                                        manifold_backup[[rdx +1, cdx]] = '<';
                                        // acc += 1;
                                        path.push((rdx +1, cdx -1));
                                    }
                                }
                                '<' => {
                                    // split beam right
                                    if rewind[[rdx + 1, cdx + 1]] == '.' {
                                        rewind[[rdx + 1, cdx + 1]] = '|';
                                        path.push((rdx +1, cdx +1));
                                        manifold_backup[[rdx +1, cdx]] = '>';
                                        // acc += 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            aoc::print_2d(&rewind);

            //insert path into hashset
            paths.insert(path.clone());
            println!("Paths size: {}", paths.len());
            // break;
            if paths.len() == set_size {
                growing = false;
            } else {
                set_size = paths.len();
            }
            acc = paths.len();
        }

        // for cdx in 0..cols {
        //     if manifold[[rows - 1, cdx]] == '|' {
        //         println!("Exit at column {}", cdx);
        //         let mut done = false;
        //         // this is a particle exit point
        //         // run waterfall backward from this position to simulate single particles
        //         let mut rewind = manifold_backup.clone();
        //         rewind[[rows - 1, cdx]] = '|';
        //         for rdx2 in (0..rows).rev() {
        //             for cdx2 in (0..cols) {
        //                 match rewind[[rdx2, cdx2]] {
        //                     '|' => {
        //                         // println!("Rewind at {},{}", rdx2, cdx2);
        //                         // check above (in manifold)
        //                         if manifold[[rdx2 -1, cdx2]] == '|' {
        //                             rewind[[rdx2 -1, cdx2]] = '|';
        //                         }
        //                         // check left
        //                         if cdx2 != 0 {
        //                             let left = rewind[[rdx2, cdx2 - 1]];
        //                             // println!("left{}", left);
        //                             if left == '^' {
        //                                 rewind[[rdx2-1, cdx2 -1]] = '|';
        //                                 manifold_backup[[rdx2, cdx2 - 1]] = '<';
        //                                 // acc += 1;
        //                                 continue;
        //                             }
        //                             if left == '>' {
        //                                 rewind[[rdx2-1, cdx2 -1]] = '|';
        //                                 manifold_backup[[rdx2, cdx2 - 1]] = 'x';
        //                                 // acc += 1;
        //                                 continue;

        //                             }
        //                         }
        //                         // check right
        //                         if cdx2 != cols -1 {
        //                             let right = rewind[[rdx2, cdx2 + 1]];
        //                             // println!("right{}", right);
        //                             if right == '^' {
        //                                 rewind[[rdx2-1, cdx2 +1]] = '|';
        //                                 manifold_backup[[rdx2, cdx2 + 1]] = '>';
        //                                 // acc += 1;
        //                                 continue;
        //                             }
        //                             if right == '<' {
        //                                 rewind[[rdx2-1, cdx2 +1]] = '|';
        //                                 manifold_backup[[rdx2, cdx2 + 1]] = 'x';
        //                                 // acc += 1;
        //                                 continue;
        //                             }
        //                         }

        //                     }
        //                     'S' => {
        //                         acc += 1;
        //                     }
        //                     _ => {}
        //                 }
        //             }
        //         }
        //         aoc::print_2d(&rewind);


        //     }
        // }
    }

    // .......S.......
    // .......|.......
    // ......|^.......
    // ......|........
    // .....|^.^......
    // .....|.........
    // ....|^.^.^.....
    // ....|..........
    // ...|^.^...^....
    // ...|...........
    // ..|^.^...^.^...
    // ..|............
    // .|^...^.....^..
    // .|.............
    // |^.^.^.^.^...^.
    // |..............

    aoc::print_2d(&manifold_backup);

    return acc;
}


/// Check training data, then apply to test data
pub fn solve(day: usize) {
    // assert_eq!(part(&format!("input/{:02}_train", day), true), 21);
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 40);
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: XX minutes.");
}
