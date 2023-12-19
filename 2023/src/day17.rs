#[path = "common.rs"]
mod common;

// use petgraph::graph::{DiGraph, NodeIndex};
// use petgraph::algo::dijkstra;
use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::Uniform;
use rand::Rng;
// use std::collections::HashSet;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Heat Loss Path Finding
#[allow(unused_assignments)]
#[allow(unused_variables)] // remove when part2 working
fn part(filename: &str, is_part1: bool) -> usize {
    // The problem with my naive implementation is that it works for the
    // training data after a while, but the test grid is so huge that I can't even
    // find the start->end in a path at all, likely due to the many checks that
    // cause a loop to break if the move is invalid. Nothing is pulling the path
    // down to that bottom right corner. perhaps a better approach would be to start
    // with a straight line, then perturb it to lower the loss.

    let mut rng = rand::thread_rng();
    // let die = Uniform::from(0..=3);
    let zoom = Uniform::new(0.0, 1.0);
    // parse info
    let (grid, rows, cols) = common::read_2d_chars(filename);
    // let mut cache: HashSet<usize> = HashSet::new();

    // navigate the path
    let mut best: (usize, Vec<u8>) = (usize::MAX, Vec::new());
    let max_steps: usize = 10_000_000;

    // progress bar
    let progress = ProgressBar::new(max_steps as u64);
    progress.set_style(
        ProgressStyle::with_template("{bar:40.cyan/blue} {pos:>10}/{len:10} [{eta} left] {msg}")
            .unwrap(),
    );

    for sss in 0..max_steps {
        progress.set_position(sss as u64);
        // init
        let mut loss = 0usize;
        // start location
        let mut prev_loc = (7usize, 7usize);
        let mut this_loc = (0usize, 0usize);
        let mut next_loc = (0usize, 0usize);
        // minimize heat loss, move max 3 blocks in a single direction
        let mut path: Vec<u8> = Vec::new();
        let mut path_locs: Vec<(usize, usize)> = Vec::new();

        // randomly fast forward on best path, do this much more during higher steps
        // the divisor controls distribution
        let probability = sigmoid((sss as f64) / ((max_steps as f64) / 2.));

        let rand_01: f64 = rng.sample(zoom);
        let do_fast_forward = rand_01 < probability;
        // do_fast_forward = rng.gen_range(0, 2);

        if do_fast_forward && best.0 < usize::MAX {
            let fast_forward_count: usize = rng.gen_range(0..best.1.len() - 1);
            // println!("ff {}", fast_forward_count);
            for fdx in 0..fast_forward_count {
                let direction: u8 = best.1[fdx];
                // when we are fast-forwarding we can skip all checks
                let possibles: Vec<(usize, usize)> = vec![
                    (this_loc.0 + 1, this_loc.1),
                    (this_loc.0.wrapping_sub(1), this_loc.1),
                    (this_loc.0, this_loc.1 + 1),
                    (this_loc.0, this_loc.1.wrapping_sub(1)),
                ];
                next_loc = possibles[direction as usize];

                // this is our next step
                prev_loc = this_loc;
                this_loc = next_loc;
                // add to path and loss
                path.push(direction);
                path_locs.push(this_loc);
                loss += grid[this_loc].to_digit(10).unwrap() as usize;
            }
        }
        loop {
            let possibles: Vec<(usize, usize)> = vec![
                (this_loc.0 + 1, this_loc.1),             // down
                (this_loc.0.wrapping_sub(1), this_loc.1), // up
                (this_loc.0, this_loc.1 + 1),
                (this_loc.0, this_loc.1.wrapping_sub(1)),
            ];
            let mut possible_pdxs: Vec<u8> = Vec::new();
            for (pdx, possible) in possibles.iter().enumerate() {
                let mut is_okay = true;
                if path_locs.contains(&possible) {
                    // prior position
                    is_okay = false;
                }
                if possible == &prev_loc || possible.0 == usize::MAX || possible.1 == usize::MAX {
                    // cannot move like this
                    is_okay = false;
                }
                if possible.0 == rows || possible.1 == cols {
                    // cannot move outside grid
                    is_okay = false;
                }
                if path.len() >= 3 {
                    if path[path.len() - 3..].iter().all(|&x| x == pdx as u8) {
                        // cannot move in a straight line more than three spaces
                        is_okay = false;
                    }
                }
                if is_okay {
                    possible_pdxs.push(pdx as u8);
                }
            }
            if possible_pdxs.len() == 0 {
                // no step from the current position is allowed
                break;
            }
            let ppdx: usize = rng.gen_range(0..possible_pdxs.len());

            let (direction, next_loc) =
                (possible_pdxs[ppdx], possibles[possible_pdxs[ppdx] as usize]);
            // this is our next step
            prev_loc = this_loc;
            this_loc = next_loc;
            // add to path and loss
            path.push(direction);
            path_locs.push(this_loc);

            // let path_hash = hash_vector(&path);
            // if cache.contains(&path_hash) {
            //     // we already went this way
            //     break;
            // } else {
            //     // new route
            //     cache.insert(path_hash);
            // }
            loss += grid[this_loc].to_digit(10).unwrap() as usize;
            if loss >= best.0 {
                // no point following a worse path
                break;
            }
            if this_loc == (rows - 1, cols - 1) {
                // in bottom right
                // println!("loss {}", loss);
                if loss < best.0 {
                    let mut fancy_path: Vec<char> = Vec::new();
                    for pos in path.iter() {
                        match pos {
                            0 => fancy_path.push('v'),
                            1 => fancy_path.push('^'),
                            2 => fancy_path.push('>'),
                            3 => fancy_path.push('<'),
                            // 0 => fancy_path.push('🡳'),
                            // 1 => fancy_path.push('🡱'),
                            // 2 => fancy_path.push('🡲'),
                            // 3 => fancy_path.push('🡰'),
                            _ => {}
                        }
                    }

                    progress.set_message(format!(
                        "best={} p={:.3} {:?}",
                        loss,
                        probability,
                        fancy_path.into_iter().collect::<String>()
                    ));

                    best.0 = loss;
                    best.1 = path;
                }
                break;
            }
        }
    }
    progress.finish_with_message(format!("done; best={} {:?}", best.0, best.1));
    return best.0;
}

pub fn solve() {
    let day: usize = 17;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), false),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    // );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: 120+ minutes");
}
