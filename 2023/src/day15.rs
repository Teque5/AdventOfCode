#[path = "common.rs"]
mod common;

fn hash_string(some_chars: &Vec<char>) -> usize {
    let mut hash = 0usize;
    for some_char in some_chars {
        hash += *some_char as usize;
        hash *= 17;
        hash %= 256;
    }
    return hash;
}

/// Lens Library
fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let line = common::read_lines(filename)[0].clone();
    let split_line = common::split_str_as::<String>(&line, ',');

    if is_part1 {
        let mut acc = 0usize;
        for step in split_line.iter() {
            acc += hash_string(&step.chars().collect());
        }
        return acc;
    }

    // part2 is here
    let mut boxes: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];

    for step in split_line.iter() {
        let mut label: Vec<char> = Vec::new();
        let mut focus = 999usize;
        let mut action = 'z';
        for c in step.chars() {
            if c.is_digit(10) {
                focus = c.to_digit(10).unwrap() as usize;
            } else if c != '=' && c != '-' {
                label.push(c);
            } else if c == '=' {
                action = c;
            } else if c == '-' {
                action = c;
            } else {
                panic!("no, bro");
            }
        }
        let box_num: usize = hash_string(&label);
        let box_label: String = label.clone().into_iter().collect();
        let box_inspect = boxes[box_num].clone();
        match action {
            '=' => {
                let mut replaced = false;
                for (idx, (old_label, _)) in box_inspect.iter().enumerate() {
                    if box_label == *old_label {
                        // replace contents
                        boxes[box_num][idx] = (box_label.clone(), focus);
                        replaced = true;
                    }
                }
                if !replaced {
                    // add to box
                    boxes[box_num].push((box_label, focus));
                }
            }
            '-' => {
                for (idx, (old_label, _)) in box_inspect.iter().enumerate() {
                    if box_label == *old_label {
                        // remove from box
                        boxes[box_num].remove(idx);
                    }
                }
            }
            _ => {
                panic!("no action? fuck");
            }
        }
        // this disabled code prints the box states after every step
        // println!("\nstep {} {:?}{}{}", step, label, action, focus);
        // for (bdx, bbox) in boxes.iter().enumerate() {
        //     if bbox.len() != 0 {
        //         println!("box {}: {:?}", bdx, bbox)
        //     }
        // }
    }

    // calculate focus power
    let mut power = 0usize;
    for (bdx, bbox) in boxes.iter().enumerate() {
        for (sdx, (_, focus)) in bbox.iter().enumerate() {
            power += (bdx + 1) * (sdx + 1) * focus;
        }
    }
    return power;
}

pub fn solve() {
    let day: usize = 15;
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
    println!("Coded: 116 minutes");
}
