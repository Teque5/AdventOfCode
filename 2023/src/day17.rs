#[path = "common.rs"]
mod common;

// use petgraph::graph::{DiGraph, NodeIndex};
// use petgraph::algo::dijkstra;
use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn step(direction: u8, some_loc: (usize, usize)) -> (usize, usize) {
    let mut next_loc: (usize, usize) = (0, 0);
    match direction {
        // down
        0 => next_loc = (some_loc.0 + 1, some_loc.1),
        // up
        1 => next_loc = (some_loc.0.wrapping_sub(1), some_loc.1),
        // right
        2 => next_loc = (some_loc.0, some_loc.1 + 1),
        // left
        3 => next_loc = (some_loc.0, some_loc.1.wrapping_sub(1)),
        _ => panic!("not possible"),
    }
    return next_loc;
}

// Function to hash a vector
fn hash_vector(vector: &[u8]) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    vector.hash(&mut hasher);
    hasher.finish() as usize
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Crucible
fn part(filename: &str, is_part1: bool) -> usize {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..=3);
    let zoom = Uniform::new(0.0, 1.0);
    // parse info
    let (grid, rows, cols) = common::read_2d_chars(filename);
    // let mut cache: HashSet<usize> = HashSet::new();

    // navigate the path
    let mut best: (usize, Vec<u8>) = (usize::MAX, Vec::new());
    let max_steps: usize = 100_000_000;

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
        // if sss % 1000 == 0 {
        //     println!("p {} {:.3}", sss, probability);
        // }
        let rand_01: f64 = rng.sample(zoom);
        let do_fast_forward = rand_01 < probability;
        // do_fast_forward = rng.gen_range(0, 2);

        if do_fast_forward && best.0 < usize::MAX {
            let fast_forward_count: usize = rng.gen_range(0..best.1.len() - 1);
            // println!("ff {}", fast_forward_count);
            for fdx in 0..fast_forward_count {
                let direction: u8 = best.1[fdx];
                next_loc = step(direction, this_loc);
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
            let direction: u8 = die.sample(&mut rng);
            let next_loc = step(direction, this_loc);
            if next_loc == prev_loc || next_loc.0 == usize::MAX || next_loc.1 == usize::MAX {
                // cannot move like this
                continue;
            }
            if next_loc.0 == rows || next_loc.1 == cols {
                // cannot move outside grid
                continue;
            }
            if path.len() >= 3 {
                if path[path.len() - 3..].iter().all(|&x| x == direction) {
                    // cannot move in a straight line more than three spaces
                    continue;
                }
            }
            if path_locs.contains(&next_loc) {
                // no crossing over self
                break;
            }
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
    // println!("Coded: xxx minutes");
}
