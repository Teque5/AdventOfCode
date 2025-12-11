use aoc;
use std::collections::{HashMap, HashSet};

/// Reactor
fn part(filename: &str, is_part1: bool) -> usize {
    let lines = aoc::read_lines(filename);
    // key is Str, val is usize
    let mut names = HashMap::<String, usize>::new();
    // key is usize, val is vec of usize
    let mut servers = HashMap::<usize, Vec<usize>>::new();
    for line in lines {
        let src = line[0..3].to_string();
        if !names.contains_key(&src) {
            names.insert(src.clone(), names.len());
        }
        let dsts = aoc::parse_delimited::<String>(&line[5..], ' ');
        for dst in &dsts {
            if !names.contains_key(dst) {
                names.insert(dst.clone(), names.len());
            }
        }
        servers.insert(
            *names.get(&src).unwrap(),
            dsts.iter().map(|d| *names.get(d).unwrap()).collect(),
        );
    }
    // println!("{:?}", servers);

    let mut visited = HashSet::<usize>::new();
    let start = *names.get("you").unwrap();
    let mut acc = 0usize;
    if is_part1 {
        // part 1 -> count how many paths from you to out
        acc = follow(&names, &servers, start, &mut visited);
    } else {
        // // part 2 -> count how many p aths from svr to out, through dac and fft
        // acc = follow_dac_fft(&names, &servers, start, &mut visited);
    }

    return acc;
}

/// From start, recursively return += 1 for each path ending at "out"
fn follow(
    names: &HashMap<String, usize>,
    servers: &HashMap<usize, Vec<usize>>,
    current: usize,
    visited: &mut HashSet<usize>,
) -> usize {
    if current == *names.get("out").unwrap() {
        return 1;
    }
    if visited.contains(&current) {
        return 0;
    }
    visited.insert(current);
    let mut acc = 0;
    if let Some(neighbors) = servers.get(&current) {
        for &neighbor in neighbors {
            acc += follow(names, servers, neighbor, visited);
        }
    }
    visited.remove(&current);
    acc
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 5);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // assert_eq!(part(&format!("input/{:02}_train", day), false), 123);
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: XX minutes.");
}
