#[path = "common.rs"]
mod common;
use indicatif::{ProgressBar, ProgressStyle};
use ndarray::Array2;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

fn roll(ray: &mut Array2<char>, direction: usize, rows: usize, cols: usize) {
    // roll all rocks north

    match direction {
        0 => {
            // North
            for cdx in 0..cols {
                for rdx in 0..rows {
                    if ray[(rdx, cdx)] == '.' {
                        // position is empty, roll if possible
                        for subrdx in rdx + 1..rows {
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
        }
        1 => {
            // West
            for rdx in 0..rows {
                for cdx in 0..cols {
                    if ray[(rdx, cdx)] == '.' {
                        // position is empty, roll if possible
                        for subcdx in cdx + 1..rows {
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
        }
        2 => {
            // South
            for cdx in 0..cols {
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
        }
        3 => {
            // East
            for rdx in 0..rows {
                for cdx in (0..cols).rev() {
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
        }
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
    // parse info
    let (ray_original, rows, cols) = common::read_2d_chars(filename);
    let mut ray = ray_original.clone();

    if is_part1 {
        // roll everything north once
        roll(&mut ray, 0, rows, cols);
    } else {
        // 4 trillion rolls
        let max_rounds = 999_999_999usize;
        // fancy progress bar
        let progress = ProgressBar::new(max_rounds as u64);
        progress.set_style(
            ProgressStyle::with_template(
                "{bar:40.cyan/blue} {pos:>9}/?         [{eta} left] {msg}",
            )
            .unwrap(),
        );
        progress.set_message("finding repeating sequence...");
        // store round number and reflector state
        let mut hash_map: HashMap<u64, ()> = HashMap::new();
        let mut hash_vec: Vec<u64> = Vec::new();
        let mut rounds_of_valid = 0usize;
        let mut rounds_of_garbage = 0usize;
        for round_num in 0..max_rounds {
            roll(&mut ray, 0, rows, cols); // N
            roll(&mut ray, 1, rows, cols); // W
            roll(&mut ray, 2, rows, cols); // S
            roll(&mut ray, 3, rows, cols); // E

            let some_hash = hash_array(&ray);
            // println!("dbug {} {}", round_num, some_hash);

            hash_map.insert(some_hash, ());
            if hash_map.len() - 1 != round_num {
                // we got a duplicate hash
                for hdx in 0..hash_vec.len() {
                    if some_hash == hash_vec[hdx] {
                        rounds_of_garbage = hdx;
                        // println!("rounds_of_garbage = {}", rounds_of_garbage);
                        break;
                    }
                }
                rounds_of_valid = round_num - rounds_of_garbage;
                // println!("rounds_of_valid at {}", rounds_of_valid);
                break;
            } else {
                hash_vec.push(some_hash);
            }

            progress.set_position(round_num as u64);

            // if round_num < 3 {
            //     println!("after round {}", round_num);
            //     common::print_2d_chars(&ray);
            // }
        }
        progress.finish_with_message(format!(
            "repeats after {} rounds",
            rounds_of_garbage + rounds_of_valid
        ));

        // rewind and only do a few rounds
        ray = ray_original.clone();
        let rounds_until_answer =
            (max_rounds - rounds_of_garbage) % rounds_of_valid + rounds_of_garbage;

        // println!("rounds until answer {}", rounds_until_answer);

        // fancy progress bar
        let progress = ProgressBar::new(rounds_until_answer as u64);
        progress.set_style(
            ProgressStyle::with_template("{bar:40.cyan/blue} {pos:>9}/{len:9} [{eta} left] {msg}")
                .unwrap(),
        );
        // rounds_until_answer += 10;
        // println!("go {} rounds", rounds_until_answer);
        for round_num in 0..rounds_until_answer {
            roll(&mut ray, 0, rows, cols); // N
            roll(&mut ray, 1, rows, cols); // W
            roll(&mut ray, 2, rows, cols); // S
            roll(&mut ray, 3, rows, cols); // E
            progress.set_position(round_num as u64);
        }
        progress.finish_with_message("done!");
    }

    let mut pressure = 0usize;
    // count north pressure
    for cdx in 0..cols {
        for rdx in 0..rows {
            if ray[(rdx, cdx)] == 'O' {
                pressure += rows - rdx;
            }
        }
    }
    return pressure;
}

pub fn solve() {
    let day: usize = 14;
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
    println!("Coded: 170 minutes");
}
