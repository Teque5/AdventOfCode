#[path = "common.rs"]
mod common;

/// are these char vectors equal?
fn are_equal(alpha: &Vec<char>, omega: &Vec<char>) -> bool {
    let mut is_equal = true;
    for idx in 0..alpha.len() {
        if (alpha[(idx)] != omega[(idx)]) && ('?' != omega[(idx)]) {
            // println!("{} {}", alpha[(idx)], omega[(idx)]);
            is_equal = false;
        }
    }
    return is_equal;
}

fn generate_combinations(
    n: usize,
    r: usize,
    current_combination: &mut Vec<usize>,
    all_combinations: &mut Vec<Vec<usize>>,
    current_spot: usize,
) {
    if current_spot == r {
        // If we have filled all spots and the sum is equal to n, add the current combination to the result
        if current_combination.iter().sum::<usize>() == n {
            all_combinations.push(current_combination.clone());
        }
    } else {
        // Place the ball in the current spot and proceed to the next spot
        for balls_in_current_spot in 0..=n {
            current_combination.push(balls_in_current_spot);
            generate_combinations(
                n,
                r,
                current_combination,
                all_combinations,
                current_spot + 1,
            );
            current_combination.pop();
        }
    }
}

// Broken Hot Springs
fn part(filename: &str, is_part1: bool) -> usize {
    let mut arrangements = 0usize;
    // let all_groups: Vec<Vec<usize>> = Vec::new();
    // let all_records: Vec<Vec<char>> = Vec::new();
    // parse info
    let lines = common::read_lines(filename);
    for line in lines {
        let mut record_and_groups = line.split_whitespace();
        let record = record_and_groups
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<_>>();
        let groups = common::split_str_as::<usize>(record_and_groups.next().unwrap(), ',');
        // determine possible arrangements
        let min_len = groups.iter().sum::<usize>() + groups.len() - 1;
        if min_len == record.len() {
            // only 1 possible arrangement; isn't likely to occur
            arrangements += 1;
            continue;
        }

        // construct a possibility and then fuck with it
        let mut possible: Vec<char> = Vec::new();
        // let mut stretch_spots: Vec<usize> = Vec::new();

        // possible.extend(vec!['.'; record.len() - min_len]);
        // let mut combos = Vec::new()
        // generate_combinations(record.len() - min_len, groups.len()+1, &mut combos)

        // figure out where blank space could be distributed
        let mut all_combinations = Vec::new();
        let mut current_combination = Vec::new();
        generate_combinations(
            record.len() - min_len,
            groups.len() + 1,
            &mut current_combination,
            &mut all_combinations,
            0,
        );
        for combo in all_combinations.iter() {
            possible.clear();

            if combo[0] != 0 {
                possible.extend(vec!['.'; combo[0]]);
            }

            for (cdx, count) in groups.iter().enumerate() {
                if cdx != 0 {
                    let mut dotcount = 1usize;
                    // // possible.push('.');
                    if combo[cdx] != 0 {
                        dotcount += combo[cdx];
                    }
                    possible.extend(vec!['.'; dotcount]);
                }
                possible.extend(vec!['#'; *count]);
            }
            if combo[groups.len()] != 0 {
                possible.extend(vec!['.'; combo[groups.len()]]);
            }
            let is_ok = are_equal(&possible, &record);
            println!(
                "{:?} {:?} {}",
                combo,
                possible.clone().into_iter().collect::<String>(),
                is_ok
            );
            if is_ok {
                arrangements += 1;
            }
        }

        // println!("{:?} combo{:?}", all_combinations, all_combinations.len());
        // println!("mmm {}", are_equal(&possible, &record));
        // println!(
        //     "{:?}\n{:?}\n{:?}\n\n",
        //     groups,
        //     record.into_iter().collect::<String>(),
        //     possible.into_iter().collect::<String>(),
        // );
        // if groups.iter.sum()
    }

    return arrangements;
}

pub fn solve() {
    let day: usize = 12;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), false),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    // );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: xxx minutes");
}
