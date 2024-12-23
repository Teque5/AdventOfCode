use aoc;
use std::collections::{HashMap, HashSet};

/// LAN Party
fn part(filename: &str, is_part1: bool) -> String {
    // parse connections
    let lines = aoc::read_lines(filename);
    let mut pairs: Vec<Vec<String>> = Vec::new();
    for line in lines {
        let pair = aoc::parse_delimited::<String>(&line, '-');
        pairs.push(pair);
    }

    // return sets of fully connected computers
    let mut set_triple: HashSet<String> = HashSet::new();

    for pair in &pairs {
        // println!("pair={:?}", &pair);
        let mut set_alpha: HashSet<&str> = HashSet::new();
        let mut set_omega: HashSet<&str> = HashSet::new();
        // create set for one side
        for subpair in &pairs {
            if pair[0] == subpair[0] {
                set_alpha.insert(&subpair[1]);
            } else if pair[0] == subpair[1] {
                set_alpha.insert(&subpair[0]);
            }
        }

        // create set for other side
        for subpair in &pairs {
            if pair[1] == subpair[0] {
                set_omega.insert(&subpair[1]);
            } else if pair[1] == subpair[1] {
                set_omega.insert(&subpair[0]);
            }
        }
        // find overlap between sets
        let intersection = set_alpha.intersection(&set_omega);
        if intersection.clone().count() > 0 {
            // println!("pair={:?}", &pair);
        }
        // join triple and put in hashset w/just triples
        for possible in intersection {
            // println!("     {:?}", possible);
            if pair[0].starts_with('t') || pair[1].starts_with('t') || possible.starts_with('t') {
                let mut entry = vec![pair[0].clone(), pair[1].clone(), possible.to_string()];
                entry.sort();
                set_triple.insert(entry.join(","));
            }
        }
    }
    let mut sorted_triples: Vec<String> = set_triple.iter().cloned().collect();
    sorted_triples.sort();
    // for triple in sorted_triples {
    //     println!("{:?}", triple);
    // }
    if is_part1 {
        return set_triple.len().to_string();
    } else {
        // part 2 uses a much more elegant implementation
        // lookup table contains bidirectional connections for each node
        let mut lut: HashMap<String, HashSet<String>> = HashMap::new();
        for pair in &pairs {
            // insert into lookup table
            lut.entry(pair[0].clone())
                .or_insert_with(HashSet::new)
                .insert(pair[1].clone());
            lut.entry(pair[1].clone())
                .or_insert_with(HashSet::new)
                .insert(pair[0].clone());
        }
        // println!("{} keys in LUT", lut.len());

        let mut survivors: Vec<Vec<String>> = lut.keys().map(|key| vec![key.clone()]).collect();
        while survivors.len() > 1 {
            // println!("{:?} survivors", &survivors);
            // println!("{} survivors", survivors.len());
            let mut connections: HashSet<Vec<String>> = HashSet::new();
            for subset_keys in survivors {
                let mut subsets: Vec<HashSet<String>> = Vec::new();
                // create temporary vector with all connections to this key
                for key in &subset_keys {
                    subsets.push(lut.get(key.as_str()).unwrap().clone());
                }
                // find overlapping connections
                let mut intersection = subsets[0].clone();
                for set in subsets.iter().skip(1) {
                    intersection = intersection.intersection(set).cloned().collect();
                }
                // sort them out into new vector that we will push into the good HashSet
                for key in intersection {
                    let mut connection = subset_keys.clone();
                    connection.push(key);
                    connection.sort();
                    connections.insert(connection);
                }
            }
            survivors = connections.into_iter().collect();
        }
        return survivors[0].join(",");
    }
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), "7");
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        "co,de,ka,ta"
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 193 Minutes");
}
