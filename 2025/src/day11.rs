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
        // println!("src: {}", src);
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

    let mut visited = HashSet::<usize>::new();
    let out_idx = *names.get("out").unwrap();
    if is_part1 {
        let you_idx = *names.get("you").unwrap();
        // part 1 -> count how many paths from you to out
        return follow(&names, &servers, you_idx, out_idx, &mut visited);
    } else {
        // // part 2 -> count how many paths from svr to out, through dac and fft
        // println!("names: {:?}", names);
        let svr_idx = *names.get("svr").unwrap();
        let fft_idx = *names.get("fft").unwrap();
        let dac_idx = *names.get("dac").unwrap();
        let svr_to_fft = follow(&names, &servers, svr_idx, fft_idx, &mut visited);
        visited.clear();
        println!("svr_to_fft: {}", svr_to_fft);
        // clear visited
        let svr_to_dac = follow(&names, &servers, svr_idx, dac_idx, &mut visited);
        visited.clear();
        println!("svr_to_dac: {}", svr_to_dac);
        let fft_to_dac = follow(&names, &servers, fft_idx, dac_idx, &mut visited);
        visited.clear();
        println!("fft_to_dac: {}", fft_to_dac);
        let dac_to_fft = follow(&names, &servers, dac_idx, fft_idx, &mut visited);
        visited.clear();
        println!("dac_to_fft: {}", dac_to_fft);
        let fft_to_out = follow(&names, &servers, fft_idx, out_idx, &mut visited);
        visited.clear();
        println!("fft_to_out: {}", fft_to_out);
        let dac_to_out = follow(&names, &servers, dac_idx, out_idx, &mut visited);
        visited.clear();
        println!("dac_to_out: {}", dac_to_out);
        // these values are number of routes between these locations.
        // return number of paths between svr and out that also go through fft and dac

        // return 0
        // let bla =  follow_dac_fft(&names, &servers, svr_idx, &mut visited, false, false);
        // println!("bla: {}", bla);
        return svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out;
    }
}

/// From start, recursively return += 1 for each path ending at "out"
fn follow(
    names: &HashMap<String, usize>,
    servers: &HashMap<usize, Vec<usize>>,
    current: usize,
    end: usize,
    visited: &mut HashSet<usize>,
) -> usize {
    if current == end {
        return 1;
    }
    if visited.contains(&current) {
        return 0;
    }
    visited.insert(current);
    let mut acc = 0;
    if let Some(neighbors) = servers.get(&current) {
        for &neighbor in neighbors {
            acc += follow(names, servers, neighbor, end, visited);
        }
    }
    // allow other paths to backtrack to here
    visited.remove(&current);
    acc
}

fn follow_dac_fft(
    names: &HashMap<String, usize>,
    servers: &HashMap<usize, Vec<usize>>,
    current: usize,
    visited: &mut HashSet<usize>,
    dac_ok: bool,
    fft_ok: bool,
) -> usize {
    if current == *names.get("out").unwrap() && dac_ok && fft_ok {
        return 1;
    }
    if visited.contains(&current) {
        return 0;
    }
    let dac_ok = dac_ok || current == *names.get("dac").unwrap();
    let fft_ok = fft_ok || current == *names.get("fft").unwrap();
    visited.insert(current);
    let mut acc = 0;
    if let Some(neighbors) = servers.get(&current) {
        for &neighbor in neighbors {
            acc += follow_dac_fft(names, servers, neighbor, visited, dac_ok, fft_ok);
        }
    }
    // allow other paths to backtrack to here
    visited.remove(&current);
    acc
}



/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train1", day), true), 5);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // TODO: day11p2 not working on test data
    assert_eq!(part(&format!("input/{:02}_train2", day), false), 2);
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 45 minutes.");
}
