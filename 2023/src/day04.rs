#[path = "common.rs"]
mod common;
use ndarray::Array2;

/// Scratchcards
fn part1(filename: &str) -> usize {
    let mut acc = 0usize;
    let (ray, rows, cols) = common::read_2d_chars(filename);
    let mut temp: String = String::new();
    // println!("dbug {} {}", rows, cols);
    for rdx in 0..rows {
        let mut trigger = false;
        let mut card_start = false;
        let mut acc_card = 0;
        let mut winning_numbers: Vec<usize> = Vec::new();
        let mut numbers_i_have: Vec<usize> = Vec::new();
        for cdx in 0..cols {
            let foo = ray[(rdx, cdx)];
            if foo == ':' {
                card_start = true;
            }
            if foo == '|' {
                // now we consider numbers i have instead of winning numbers
                trigger = true;
                continue;
            }
            if !card_start {
                continue;
            }
            if foo.is_digit(10) {
                temp.push(foo);
            } else if !foo.is_digit(10) {
                if let Ok(parsed) = temp.parse::<usize>() {
                    if !trigger {
                        winning_numbers.push(parsed);
                    } else {
                        numbers_i_have.push(parsed)
                    }
                    temp.clear();
                }
            }
            // special case: on last digit
            if cdx + 1 == cols {
                if let Ok(parsed) = temp.parse::<usize>() {
                    numbers_i_have.push(parsed);
                    temp.clear();
                }
            }
        }
        // check for winning numbers
        for winner in winning_numbers.iter() {
            if numbers_i_have.contains(winner) {
                if acc_card == 0 {
                    acc_card += 1;
                } else {
                    acc_card *= 2;
                }
            }
        }
        acc += acc_card;

        // println!("acc_card: {}", acc_card);
        // print!("\nhave ");
        // for num in numbers_i_have.iter() {
        //     print!("{} ", num);
        // }
        // print!("\nwin ");
        // for num in winning_numbers.iter() {
        //     print!("{} ", num);
        // }
        // println!();
    }
    return acc;
}

/// returns win reward for a particular row
fn check_card(ray: &Array2<char>, cols: usize, rdx: usize) -> usize {
    // println!("check {}", rdx);
    let mut temp: String = String::new();
    let mut trigger = false;
    let mut card_start = false;
    let mut acc_card = 0;
    let mut winning_numbers: Vec<usize> = Vec::new();
    let mut numbers_i_have: Vec<usize> = Vec::new();
    for cdx in 0..cols {
        let foo = ray[(rdx, cdx)];
        if foo == ':' {
            card_start = true;
        }
        if foo == '|' {
            // now we consider numbers i have instead of winning numbers
            trigger = true;
            continue;
        }
        if !card_start {
            continue;
        }
        if foo.is_digit(10) {
            temp.push(foo);
        } else if !foo.is_digit(10) {
            if let Ok(parsed) = temp.parse::<usize>() {
                if !trigger {
                    winning_numbers.push(parsed);
                } else {
                    numbers_i_have.push(parsed)
                }
                temp.clear();
            }
        }
        // special case: on last digit
        if cdx + 1 == cols {
            if let Ok(parsed) = temp.parse::<usize>() {
                numbers_i_have.push(parsed);
                temp.clear();
            }
        }
    }
    // check for winning numbers
    for winner in winning_numbers.iter() {
        if numbers_i_have.contains(winner) {
            acc_card += 1;
        }
    }
    // println!("accc{}", acc_card);
    return acc_card;
}

/// now count total winning cards
fn part2(filename: &str) -> usize {
    let (ray, rows, cols) = common::read_2d_chars(filename);

    let mut card_count: Vec<usize> = vec![1usize; rows];
    let mut card_winners: Vec<bool> = vec![false; rows];
    for card_index in 0..rows {
        let score = check_card(&ray, cols, card_index);
        // println!("{} {}", card_index, score);
        if score > 0 {
            card_winners[card_index] = true;
            for new_index in (card_index + 1)..(card_index + score + 1) {
                // println!("{} add {}x{}", card_index, new_index, card_count[card_index]);
                card_count[new_index] += card_count[card_index];
            }
        }
    }
    return card_count.iter().sum();
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part1("input/04_train"),
        common::read_lines_as::<usize>("input/04_val1")[0]
    );
    println!("Part1: {}", part1("input/04_test"));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part2("input/04_train"),
        common::read_lines_as::<usize>("input/04_val2")[0]
    );
    println!("Part2: {}", part2("input/04_test"));
    println!("Coded: 113 minutes");
}
