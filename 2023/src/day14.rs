#[path = "common.rs"]
mod common;
use ndarray::Array2;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{HashMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

fn roll(ray: &mut Array2<char>, direction:usize, rows:usize, cols:usize) {
    // roll all rocks north

    match direction {
        0 => {
            // North
            for cdx in 0..cols {
                for rdx in 0..rows {
                    if ray[(rdx,cdx)] == '.' {
                        // position is empty, roll if possible
                        for subrdx in rdx+1..rows {
                            if ray[(subrdx, cdx)] == 'O' {
                                // roll up
                                ray[(rdx, cdx)] = 'O';
                                ray[(subrdx, cdx)] = '.';
                                break;
                            }
                            if ray[(subrdx, cdx)] == '#' {
                                // cannot roll past this
                                break;
                            }
                        }
                    }
                }
            }
        },
        3 => {
            // East
            for rdx in 0..rows {
                for cdx in (0..cols).rev(){
                    // from bot to top
                    if ray[(rdx, cdx)] == '.' {
                        // position is empty, roll if possible
                        for subcdx in (0..cdx).rev() {
                            if ray[(rdx, subcdx)] == 'O' {
                                // roll up
                                ray[(rdx, cdx)] = 'O';
                                ray[(rdx, subcdx)] = '.';
                                break;
                            }
                            if ray[(rdx, subcdx)] == '#' {
                                // cannot roll past this
                                break;
                            }
                        }
                    }
                }
            }

        },
        2 => {
            // South
            for cdx in 0..cols{
                for rdx in (0..rows).rev() {
                    // from bot to top
                    if ray[(rdx, cdx)] == '.' {
                        // position is empty, roll if possible
                        for subrdx in (0..rdx).rev() {
                            if ray[(subrdx, cdx)] == 'O' {
                                // roll up
                                ray[(rdx, cdx)] = 'O';
                                ray[(subrdx, cdx)] = '.';
                                break;
                            }
                            if ray[(subrdx, cdx)] == '#' {
                                // cannot roll past this
                                break;
                            }
                        }
                    }
                }
            }
        },
        1 => {
            // West
            for rdx in 0..rows {
                for cdx in 0..cols{
                    if ray[(rdx,cdx)] == '.' {
                        // position is empty, roll if possible
                        for subcdx in cdx+1..rows {
                            if ray[(rdx, subcdx)] == 'O' {
                                // roll up
                                ray[(rdx, cdx)] = 'O';
                                ray[(rdx, subcdx)] = '.';
                                break;
                            }
                            if ray[(rdx, subcdx)] == '#' {
                                // cannot roll past this
                                break;
                            }
                        }
                    }
                }
            }
        },
        _ => panic!("can only roll directions (0, 1, 2, 3)"),
    }

}

fn hash_array<T: Hash>(array: &Array2<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    array.hash(&mut hasher);
    hasher.finish()
}

/// Focus the Reflector
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;

    // parse info
    let (mut ray, rows, cols) = common::read_2d_chars(filename);

    if is_part1 {
        // roll everything north once
        roll(&mut ray, 0, rows, cols);
    } else {
        // 4 trillion rolls
        let rounds = 1_000_000_000usize;
        // fancy progress bar
        let progress = ProgressBar::new(rounds as u64);
        progress.set_style(
            ProgressStyle::with_template("{bar:40.cyan/blue} {pos:>10}/{len:10} [{eta} left] {msg}")
                .unwrap()
                // .progress_chars("#>-"),
        );

        // store round number and reflector state
        let mut hash_map: HashMap<usize, u64> = HashMap::new();
        //sanity_check
        roll(&mut ray, 0, rows, cols);
        let bla1 = hash_array(&ray);
        roll(&mut ray, 2, rows, cols);
        roll(&mut ray, 2, rows, cols);
        roll(&mut ray, 0, rows, cols);
        let bla2 = hash_array(&ray);
        println!("should be same {} {} {}", bla1, bla2, bla1==bla2);

        for round_num in 0..rounds {
            hash_map.insert(round_num, hash_array(&ray));
            if hash_map.len() != round_num + 1 {
                // we started over
                println!("repeat found {}", round_num);
                break;
            }
            roll(&mut ray, 0, rows, cols);
            roll(&mut ray, 1, rows, cols);
            roll(&mut ray, 2, rows, cols);
            roll(&mut ray, 3, rows, cols);
            if round_num % 10_000 == 0 {
                progress.set_position(round_num as u64);
            }
            if round_num < 3 {
                println!("after round {}", round_num);
                common::print_2d_chars(&ray);
            }
        }
        progress.finish();
    }

    // count north pressure
    for cdx in 0..cols {
        for rdx in 0..rows {
            if ray[(rdx, cdx)] == 'O' {
                acc += rows-rdx;
            }
        }
    }
    return acc;
}

pub fn solve() {
    let day: usize = 14;
    // Test part-1 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), true),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    // );
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: 118 minutes");
}
