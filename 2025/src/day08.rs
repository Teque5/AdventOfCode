use aoc;
// use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn distance(&self, other: &Position) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Playground
fn part1(filename: &str) -> usize {
    let mut positions = Vec::<Position>::new();
    let mut circuits = Vec::<isize>::new();
    // parse
    let lines = aoc::read_lines(filename);
    for line in lines {
        let xyz = aoc::parse_delimited::<i32>(&line, ',');
        let pos = Position {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        };
        positions.push(pos);
        circuits.push(-1);
        // println!("{} {:?}", positions.len() - 1, pos);
    }

    let mut next_circuit = 0isize;
    let mut pairs = HashSet::<(usize, usize)>::new();
    let num_connections = if filename.contains("train") { 10 } else { 1000 };
    let progress = ProgressBar::new(num_connections as u64);
    progress.set_style(ProgressStyle::with_template(aoc::STYLE).unwrap());
    for _ in 0..num_connections {
        let mut min_dist = std::f64::MAX;
        let mut min_pair = (0usize, 0usize);
        for ldx in 0..positions.len() {
            for rdx in 0..positions.len() {
                // println!("Considering pair: {} and {}", ldx, rdx);
                if ldx <= rdx {
                    continue;
                }
                if pairs.contains(&(ldx, rdx)) {
                    continue;
                }
                if pairs.contains(&(rdx, ldx)) {
                    continue;
                }
                let dist = positions[ldx].distance(&positions[rdx]);
                // println!("Distance between {} and {}: {}", ldx, rdx, dist);
                if dist < min_dist {
                    min_dist = dist;
                    min_pair = (ldx, rdx);
                }
            }
        }
        progress.inc(1);
        pairs.insert(min_pair);

        let left_circuit = circuits[min_pair.0];
        let right_circuit = circuits[min_pair.1];
        if left_circuit == -1 && right_circuit == -1 {
            // New circuit
            circuits[min_pair.0] = next_circuit;
            circuits[min_pair.1] = next_circuit;
            next_circuit += 1;
        } else if left_circuit != -1 && right_circuit == -1 {
            // Add to left circuit
            circuits[min_pair.1] = left_circuit;
        } else if left_circuit == -1 && right_circuit != -1 {
            // Add to right circuit
            circuits[min_pair.0] = right_circuit;
        } else {
            // Merge circuits to left_circuit;
            circuits.iter_mut().for_each(|circuit| {
                if *circuit == right_circuit {
                    *circuit = left_circuit;
                }
            });
        }
    }
    progress.finish_and_clear();
    // count size of each circuit
    let mut circuit_sizes = vec![0; (next_circuit) as usize];
    for circuit in circuits.iter() {
        if *circuit != -1 {
            circuit_sizes[*circuit as usize] += 1;
        }
    }

    // multiply sizes of three largest circuits
    circuit_sizes.sort();
    circuit_sizes.reverse();
    return circuit_sizes.iter().take(3).product();
}

fn part2(filename: &str) -> usize {
    // position, circuit index
    let mut positions = Vec::<Position>::new();
    let mut circuits = Vec::<isize>::new();
    // parse
    let lines = aoc::read_lines(filename);
    for line in lines {
        let xyz = aoc::parse_delimited::<i32>(&line, ',');
        let pos = Position {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        };
        positions.push(pos);
        circuits.push(-1);
        // println!("{} {:?}", positions.len() - 1, pos);
    }
    // create new progressbar with total number of positions
    let progress = ProgressBar::new(100u64);
    progress.set_style(ProgressStyle::with_template(aoc::STYLE).unwrap());

    let mut next_circuit = 0isize;
    let mut pairs = HashSet::<(usize, usize)>::new();
    loop {
        let mut min_dist = std::f64::MAX;
        let mut min_pair = (0usize, 0usize);
        for ldx in 0..positions.len() {
            for rdx in 0..positions.len() {
                // println!("Considering pair: {} and {}", ldx, rdx);
                if ldx <= rdx {
                    continue;
                }
                if pairs.contains(&(ldx, rdx)) {
                    continue;
                }
                if pairs.contains(&(rdx, ldx)) {
                    continue;
                }
                let dist = positions[ldx].distance(&positions[rdx]);
                // println!("Distance between {} and {}: {}", ldx, rdx, dist);
                if dist < min_dist {
                    min_dist = dist;
                    min_pair = (ldx, rdx);
                }
            }
        }
        pairs.insert(min_pair);
        let left_circuit = circuits[min_pair.0];
        let right_circuit = circuits[min_pair.1];
        if left_circuit == -1 && right_circuit == -1 {
            // New circuit
            circuits[min_pair.0] = next_circuit;
            circuits[min_pair.1] = next_circuit;
            next_circuit += 1;
        } else if left_circuit != -1 && right_circuit == -1 {
            // Add to left circuit
            circuits[min_pair.1] = left_circuit;
        } else if left_circuit == -1 && right_circuit != -1 {
            // Add to right circuit
            circuits[min_pair.0] = right_circuit;
        } else {
            // Merge circuits to left_circuit;
            circuits.iter_mut().for_each(|circuit| {
                if *circuit == right_circuit {
                    *circuit = left_circuit;
                }
            });
        }
        let unique_circuits: HashSet<isize> = circuits.iter().cloned().collect();
        let num_circuits = unique_circuits.len();
        if num_circuits < 100 {
            progress.set_position(101 - unique_circuits.len() as u64);
        }
        progress.set_message(format!("num_circuits = {}", num_circuits));
        if num_circuits == 1 {
            // exit if only one circuit remains
            progress.finish_and_clear();
            return (positions[min_pair.0].x as usize) * (positions[min_pair.1].x as usize);
        }
    }
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part1(&format!("input/{:02}_train", day)), 40);
    println!("Part1: {}", part1(&format!("input/{:02}_test", day)));

    assert_eq!(part2(&format!("input/{:02}_train", day)), 25272);
    println!("Part2: {}", part2(&format!("input/{:02}_test", day)));

    println!("Coded: 107 minutes.");
}
