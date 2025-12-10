use aoc;
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

///
fn merge_circuits(circuits: &mut Vec<isize>, next_circuit: &mut isize, min_pair: (usize, usize)) {
    let left_circuit = circuits[min_pair.0];
    let right_circuit = circuits[min_pair.1];
    if left_circuit == -1 && right_circuit == -1 {
        // New circuit
        circuits[min_pair.0] = *next_circuit;
        circuits[min_pair.1] = *next_circuit;
        *next_circuit += 1;
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

/// Playground
/// connect light circuits by distance
fn part(filename: &str, is_part1: bool) -> usize {
    let mut positions = Vec::<Position>::new();
    let mut circuits = Vec::<isize>::new();
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
    }
    let mut distances = Vec::<((usize, usize), f64)>::new();
    // compute all-to-all distances
    for ldx in 0..positions.len() {
        for rdx in (ldx + 1)..positions.len() {
            let dist = positions[ldx].distance(&positions[rdx]);
            distances.push(((ldx, rdx), dist));
        }
    }
    // sort distance between nodes
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut next_circuit = 0isize;

    if is_part1 {
        let num_connections = if filename.contains("train") { 10 } else { 1000 };
        for ndx in 0..num_connections {
            let min_pair = distances[ndx].0;
            merge_circuits(&mut circuits, &mut next_circuit, min_pair);
        }
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
    } else {
        let mut num_circuits = usize::MAX;
        let mut ndx = 0usize;
        let mut min_pair = (0usize, 0usize);
        while num_circuits > 1 {
            min_pair = distances[ndx].0;
            ndx += 1;
            merge_circuits(&mut circuits, &mut next_circuit, min_pair);
            let unique_circuits: HashSet<isize> = circuits.iter().cloned().collect();
            num_circuits = unique_circuits.len();
        }
        // only one circuit remains, compute x distance
        return (positions[min_pair.0].x as usize) * (positions[min_pair.1].x as usize);
    }
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 40);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 25272);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 107 minutes.");
}
