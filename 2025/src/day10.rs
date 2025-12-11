use aoc;
use itertools::Itertools;

/// Factory
/// Button combinatorics
fn part(filename: &str, _is_part1: bool) -> usize {
    let mut acc = 0usize;
    let lines = aoc::read_lines(&filename);
    let mut indicator = Vec::<Vec<bool>>::new();
    let mut wiring = Vec::<Vec<Vec<usize>>>::new();
    let mut joltage = Vec::<Vec<usize>>::new();
    for (ldx, line) in lines.iter().enumerate() {
        // pretty tricky parsing this time
        let mut mode = "indicator";
        let mut wdx = 0;
        // push empty vectors to indicator & wiring
        indicator.push(Vec::new());
        wiring.push(Vec::new());
        for (cdx, ch) in line.chars().enumerate() {
            match mode {
                "indicator" => match ch {
                    '.' => {
                        indicator[ldx].push(false);
                    }
                    '#' => {
                        indicator[ldx].push(true);
                    }
                    ']' => {
                        mode = "wiring";
                    }
                    _ => {}
                },
                "wiring" => match ch {
                    ')' => {
                        wdx += 1;
                    }
                    '0'..='9' => {
                        if wiring[ldx].len() <= wdx {
                            wiring[ldx].push(Vec::new());
                        }
                        let digit = ch.to_digit(10).unwrap() as usize;
                        wiring[ldx][wdx].push(digit);
                    }
                    '{' => {
                        mode = "joltage";
                    }
                    _ => {}
                },
                "joltage" => {
                    // read remaining as joltage requirement
                    joltage.push(aoc::parse_delimited::<usize>(
                        &line[cdx..line.len() - 1],
                        ',',
                    ));
                    break;
                }
                _ => {}
            }
        }
        // println!("buttons: {:?}", wiring[ldx]);
        // println!("joltage: {:?}", joltage[ldx]);
    }
    // pushing buttons twice is pointless -> consider all possible combinations of button pushes to get the correct indicator
    for ldx in 0..indicator.len() {
        // println!("dbug {:?}", indicator[ldx]);
        // for each indicator, check shortest to longest combinations of buttons
        'outer: for m in 1..=wiring[ldx].len() {
            for combo in wiring[ldx].iter().combinations(m) {
                let mut this_indicator = vec![false; indicator[ldx].len()];
                for button in &combo {
                    for &bdx in *button {
                        this_indicator[bdx] = !this_indicator[bdx];
                    }
                }
                if this_indicator == indicator[ldx] {
                    // println!("{} ok! +{} {:?}", ldx, m, combo);
                    acc += m;

                    break 'outer;
                }
            }
        }
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 7);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 33);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 60+? minutes.");
}
