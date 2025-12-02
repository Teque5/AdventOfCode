use aoc;

/// Gift Shop
fn part(filename: &str, is_part1: bool, _render: bool) -> usize {
    let mut acc = 0usize; // invalid ID sum
                          // parse info
    let lines = aoc::read_lines(filename);
    // line is like XXXX-YYYY,XXXX-YYYY,XXXX-YYYY... read line as series of tuples
    for line in lines {
        let tuples: Vec<&str> = line.split(',').collect();
        for tuple in tuples {
            let edges: Vec<usize> = aoc::parse_delimited(&tuple, '-');
            // println!("Edges: {:?}.", edges);
            for id in edges[0]..=edges[1] {
                // count the number of digits occupied by this id
                let id_str = id.to_string();
                if is_part1 {
                    // invalid IDs have repeated digits
                    if id_str.len() % 2 == 0 {
                        // even length, check for repeated digits
                        let left = &id_str[0..(id_str.len() / 2)];
                        let right = &id_str[(id_str.len() / 2)..];
                        if left == right {
                            // invalid id
                            acc += id;
                        }
                    }
                } else {
                    // invalid IDs have perhaps many repeated digits
                    let max_items = id_str.len() / 2;
                    for chunk_size in 1..=max_items {
                        let mut all_equal = true;
                        let divisors = id_str.len() / chunk_size;
                        if chunk_size * divisors != id_str.len() {
                            continue;
                        }
                        let first_chunk = &id_str[0..chunk_size];
                        for divisor in 0..divisors {
                            let start = divisor * chunk_size;
                            let end = start + chunk_size;
                            let this_chunk = &id_str[start..end];
                            if this_chunk != first_chunk {
                                all_equal = false;
                                break;
                            }
                        }
                        if all_equal {
                            // invalid id
                            acc += id;
                            break;
                        }
                    }
                }
            }
        }
    }

    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(
        part(&format!("input/{:02}_train", day), true, false),
        1227775554
    );
    println!(
        "Part1: {}",
        part(&format!("input/{:02}_test", day), true, false)
    );

    assert_eq!(
        part(&format!("input/{:02}_train", day), false, true),
        4174379265
    );
    println!(
        "Part2: {}",
        part(&format!("input/{:02}_test", day), false, false)
    );

    println!("Coded: 40 minutes.");
}
