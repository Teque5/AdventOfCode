use aoc;

/// Cafeteria
/// Ostensibly count fresh ingredients
/// Actually count overlaps in ranges
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;
    // parse info
    let lines = aoc::read_lines(filename);
    // store vector of usize tuples
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut ingredients: Vec<usize> = Vec::new();
    for line in lines {
        // if line contains "-", push to ranges, otherwise push to ingredients
        let edges: Vec<usize> = aoc::parse_delimited(&line, '-');
        if edges.len() == 2 {
            ranges.push((edges[0], edges[1]));
        } else if edges.len() == 1 {
            ingredients.push(edges[0]);
        }
    }
    if is_part1 {
        // check each ingredient against ranges
        for ingredient in ingredients {
            for range in &ranges {
                if ingredient >= range.0 && ingredient <= range.1 {
                    acc += 1;
                    break;
                }
            }
        }
        return acc;
    }
    // how many ingredients are in all ranges?
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged: Vec<(usize, usize)> = Vec::new();
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range.0 <= last.1 {
                // overlap -> merge
                last.1 = last.1.max(range.1);
            } else {
                // new range
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    // count size of all merged ranges
    for range in &merged {
        acc += range.1 - range.0 + 1;
    }

    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 3);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 14);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 35 minutes.");
}
