#[path = "common.rs"]
mod common;

/// check that update complies with the rules
fn check_correct(update: &Vec<usize>, rules: &Vec<Vec<usize>>, ok: &mut bool) -> usize {
    let mut bad_index = 9999usize;
    for (vdx, value) in update.iter().enumerate() {
        for later_value in rules[*value].iter() {
            if update[0..vdx].contains(&later_value) {
                // page that should be later, is earlier; fail
                *ok = false;
                // save bad position for part 2
                bad_index = update[0..vdx]
                    .iter()
                    .position(|&x| x == *later_value)
                    .unwrap();
                // we have failed and are done
                return bad_index;
            }
        }
    }
    *ok = true;
    return bad_index;
}

/// remove the bad index and place it at the end
fn reorder(update: &Vec<usize>, bad_index: &usize) -> Vec<usize> {
    let mut new_update = update.clone();
    let early = new_update.remove(*bad_index);
    new_update.push(early);
    return new_update;
}

/// Page Ordering
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;

    // parse info
    let lines = common::read_lines(filename);
    let mut is_rule = true;
    // there seem to be < 100 pages
    let mut rules: Vec<Vec<usize>> = vec![Vec::new(); 100];
    for line in lines {
        if line.len() == 0 {
            // switch to update reader
            is_rule = false;
            continue;
        }
        if is_rule {
            // insert rules into LUT
            let raw_rule = common::parse_delimited::<usize>(&line, '|');
            rules[raw_rule[0]].push(raw_rule[1]);
        } else {
            // check updates
            let mut ok = true;
            let mut bad_index: usize;
            let update = common::parse_delimited::<usize>(&line, ',');
            bad_index = check_correct(&update, &rules, &mut ok);
            if ok && is_part1 {
                // add middle value
                acc += update[update.len() / 2]
            }
            if !ok && !is_part1 {
                // part 2 -> put the pages in the correct order
                let mut new_update = update.clone();
                while !ok {
                    new_update = reorder(&new_update, &bad_index);
                    bad_index = check_correct(&new_update, &rules, &mut ok);
                }
                // add middle value
                acc += new_update[update.len() / 2];
            }
        }
    }
    return acc;
}

pub fn solve() {
    let day: usize = 5;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), false),
        common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 78 Minutes");
}
