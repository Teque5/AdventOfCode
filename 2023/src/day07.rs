#[path = "common.rs"]
mod common;
use std::collections::HashMap;
use std::collections::HashSet;

/// return the ranks of a vector as a new vector
fn rank(vector: &Vec<usize>) -> Vec<usize> {
    let mut ranked_indices: Vec<usize> = (0..vector.len()).collect();
    ranked_indices.sort_by_key(|&i| vector[i]);
    let mut ranks = vec![0; vector.len()];

    for (rank, &index) in ranked_indices.iter().enumerate() {
        ranks[index] = rank;
    }

    ranks
}

/// Camel Card Scoring
fn part(filename: &str, is_part1: bool) -> usize {
    // Basic approach is to set a score for each hand on a base15 number system
    // Hand type * 1e6 + score per card * position per card

    // HashMap allow us to check quickly for # of unique cards per hand
    let mut lut: HashMap<char, usize> = vec![
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]
    .into_iter()
    .collect();
    if !is_part1 {
        // J is now joker and lowest value
        lut.insert('J', 1);
    }

    // parse instructions
    let lines = common::read_lines(filename);
    let mut bids: Vec<usize> = Vec::new();
    let mut powers: Vec<usize> = Vec::new();
    let mut hands: Vec<Vec<usize>> = Vec::new();
    // any base power over 15**5 works, but make it million so we can see hand types
    let base_power = 1_000_000usize;
    for line in lines {
        let mut hand_and_bid = line.split_whitespace();
        let hand = hand_and_bid.next().unwrap().chars().collect::<Vec<_>>();
        let bid = hand_and_bid.next().unwrap().parse::<usize>().unwrap();
        let numeric_hand: Vec<usize> = hand.iter().map(|&c| *lut.get(&c).unwrap_or(&0)).collect();
        let have_jokers = !is_part1 && numeric_hand.contains(&1);
        // do a quick power check based on hand type
        let mut power = 0usize;
        // check for all different values
        let hash_hand: HashSet<_> = numeric_hand.iter().cloned().collect();
        let unique_count = hash_hand.len();
        match unique_count {
            1 => {
                // Five of a Kind: all values are the same
                power = 6 * base_power;
            }
            2 => {
                // Full house || 4 of a kind
                let mut count = 0usize;
                let some_val = numeric_hand[0];
                for idx in 1..5 {
                    if some_val == numeric_hand[idx] {
                        count += 1;
                    }
                }
                if count == 3 || count == 0 {
                    // 4 of a kind
                    power = 5 * base_power;
                } else {
                    // Full House
                    power = 4 * base_power;
                }
                if have_jokers {
                    // hand is now five-of-a-kind
                    power = 6 * base_power;
                }
            }
            3 => {
                // Three of a kind || Two Pair
                let mut max_similar = 0usize;
                for idx in 0..5 {
                    let mut similar = 0usize;
                    for jdx in 0..5 {
                        if numeric_hand[idx] == numeric_hand[jdx] {
                            similar += 1;
                        }
                    }
                    if similar > max_similar {
                        max_similar = similar;
                    }
                }
                if max_similar == 3 {
                    // Three of a Kind
                    power = 3 * base_power;
                    if have_jokers {
                        // hand is now four-of-a-kind
                        power = 5 * base_power;
                    }
                } else {
                    // Two Pair
                    power = 2 * base_power;
                    if have_jokers {
                        let mut joker_count = 0usize;
                        for val in numeric_hand.iter() {
                            if val == &1 {
                                joker_count += 1
                            }
                        }
                        match joker_count {
                            // hand is now full house
                            1 => power = 4 * base_power,
                            // hand is now four-of-a-kind
                            2 => power = 5 * base_power,
                            _ => panic!("not possible"),
                        }
                    }
                }
            }
            4 => {
                // One Pair only
                power = 1 * base_power;
                if have_jokers {
                    // hand is now three-of-a-kind
                    power = 3 * base_power;
                }
            }
            5 => {
                // High Card
                if have_jokers {
                    // hand is now one pair
                    power = 1 * base_power;
                }
            }
            _ => panic!("fuck"),
        }
        // now we add the powers of the cards too
        for idx in 0..5 {
            power += 15_usize.pow(4 - idx) * numeric_hand[idx as usize]
        }
        hands.push(numeric_hand);
        bids.push(bid);
        powers.push(power);
    }
    // now we sort all the hands and assign ranks
    let ranks: Vec<usize> = rank(&powers);

    let mut winnings = 0usize;
    for idx in 0..hands.len() {
        winnings += bids[idx] * (ranks[idx] + 1);
    }

    // for idx in 0..hands.len() {
    //     println!(
    //         "{:3} {:4} {:12} {:?} {} -> {}",
    //         idx,
    //         ranks[idx],
    //         powers[idx],
    //         hands[idx],
    //         bids[idx],
    //         bids[idx] * (ranks[idx] + 1)
    //     )
    // }

    return winnings;
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part("input/07_train", true),
        common::read_lines_as::<usize>("input/07_val1")[0]
    );
    println!("Part1: {}", part("input/07_test", true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part("input/07_train", false),
        common::read_lines_as::<usize>("input/07_val2")[0]
    );
    println!("Part2: {}", part("input/07_test", false));
    println!("Coded: 164 minutes");
}
