use aoc;
use std::collections::HashMap;
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
fn part(filename: &str, _is_part1: bool) -> usize {
    let mut acc = 0usize;
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

    let mut next_circuit = 0isize;
    let mut pairs = HashSet::<(usize, usize)>::new();
    let num_connections = if filename.contains("train") { 10 } else { 1000 };
    for ndx in 0..num_connections {
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
        println!("{} Min pair: {:?} at distance {}", ndx, min_pair, min_dist);
        let left_pos = positions[min_pair.0];
        let right_pos = positions[min_pair.1];
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
    println!("Final circuits: {:?}", circuits);
    // count size of each circuit
    let mut circuit_sizes = vec![0; (next_circuit) as usize];
    for circuit in circuits.iter() {
        if *circuit != -1 {
            circuit_sizes[*circuit as usize] += 1;
        }
    }
    println!("Circuit sizes: {:?}", circuit_sizes);

    // multiply sizes of three largest circuits
    circuit_sizes.sort();
    circuit_sizes.reverse();
    acc = circuit_sizes.iter().take(3).product();
    return acc;
}

fn part2(filename: &str, _is_part1: bool) -> usize {
    let mut acc = 0usize;
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

    let mut next_circuit = 0isize;
    let mut pairs = HashSet::<(usize, usize)>::new();
    let num_connections = if filename.contains("train") { 10 } else { 1000 };
    let mut ndx = 0usize;
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
        println!("{} Min pair: {:?} at distance {}", ndx, min_pair, min_dist);
        let left_pos = positions[min_pair.0];
        let right_pos = positions[min_pair.1];
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
        ndx += 1;
        // exit if all circuits are equal

        // bla
        let unique_circuits: HashSet<isize> = circuits.iter().cloned().collect();
        if unique_circuits.len() == 1 {
            // print last connected circuits
            println!("break {:?} {:?}", left_circuit, right_circuit);
            println!(
                "break {:?} {:?}",
                positions[min_pair.0], positions[min_pair.1]
            );
            acc = (positions[min_pair.0].x as usize) * (positions[min_pair.1].x as usize);
            break;
        }
    }
    // println!("Final circuits: {:?}", circuits);
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    // assert_eq!(part(&format!("input/{:02}_train", day), true), 40);
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part2(&format!("input/{:02}_train", day), false), 25272);
    println!("Part2: {}", part2(&format!("input/{:02}_test", day), false));

    println!("Coded: 107 minutes.");
}
