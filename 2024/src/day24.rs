use aoc;
use std::collections::HashMap;

/// Crossed Wires
fn part(filename: &str, is_part1: bool) -> String {
    // store wire values here
    let mut lut: HashMap<String, u8> = HashMap::new();
    let mut zzz: HashMap<String, u8> = HashMap::new();
    let mut eqs: HashMap<String, (String, String, String)> = HashMap::new();
    // parse initial values
    let lines = aoc::read_lines(filename);
    for line in lines {
        if line.contains(":") {
            // initial wire value
            // why is string parsing in rust so heinous
            let mut parts = line.splitn(2, ": ");
            let key = parts.next().unwrap();
            let value: u8;
            match parts.next().unwrap() {
                "1" => value = 1,
                "0" => value = 0,
                _ => unreachable!("not possible"),
            }
            lut.insert(key.to_string(), value);
        } else if line.len() > 0 {
            // is wire connection
            let mut parts = line.splitn(5, " ");
            let key_left = parts.next().unwrap();
            let gate = parts.next().unwrap();
            let key_right = parts.next().unwrap();
            parts.next(); // skip "->"
            let result = parts.next().unwrap();
            eqs.insert(
                result.to_string(),
                (
                    key_left.to_string(),
                    gate.to_string(),
                    key_right.to_string(),
                ),
            );
            // println!("{:?} {:?} {:?} {:?}", key_left, gate, key_right, result);
        }
    }
    // println!("{:?}", lut);
    // println!("{:?}", eqs);

    // lut.insert("z00".to_string(), 0);
    // lut.insert("z01".to_string(), 0);
    // lut.insert("z02".to_string(), 1);
    // lut.insert("z03".to_string(), 0);

    let keys_for_processing: Vec<String> = eqs
        .keys()
        .filter(|&key| key.starts_with("z"))
        .cloned()
        .collect();
    if is_part1 {
        // iterate up looking for z outputs
        for key in keys_for_processing {
            // println!("{:?}", key);
            zzz.insert(key.clone(), process(&key, &mut lut, &mut eqs));
            // println!("{}", result);
        }
        return check_results(&zzz, "z".to_string()).to_string();
    } else {
        println!("{:?}", keys_for_processing);
        // pretend x and y are all 1s and find the bogus gates?

        // let x_val = check_results(&lut, "x".to_string());
        // let y_val = check_results(&lut, "y".to_string());
        let mut dummy_lut: HashMap<String, u8> = HashMap::new();
        for idx in 0..keys_for_processing.len() {
            dummy_lut.insert(format!("x{:02}", idx), 0);
            dummy_lut.insert(format!("y{:02}", idx), 0);
        }
        for idx in 0..keys_for_processing.len() {
            dummy_lut.insert(format!("x{:02}", idx), 1);
            dummy_lut.insert(format!("y{:02}", idx), 1);
            for key in &keys_for_processing {
                // println!("{:?}", key);
                zzz.insert(key.clone(), process(&key, &mut dummy_lut, &mut eqs));
                // println!("{}", result);
            }
            let z_current = check_results(&zzz, "z".to_string());
            dummy_lut.insert(format!("x{:02}", idx), 0);
            dummy_lut.insert(format!("y{:02}", idx), 0);
            // println!("digit={} res={}", idx, (z_current >> idx) & 1);
            if ((z_current >> idx) & 1) == 0 {
                println!("problem with x{:02} or y{:02} processing", idx, idx);
            } else {
                println!("wire {} ok", idx);
            }
        }
        // println!("{:?}", dummy_lut);
        // for key in keys_for_processing {
        //     // println!("{:?}", key);
        //     zzz.insert(key.clone(), process(&key, &mut dummy_lut, &mut eqs));
        //     // println!("{}", result);
        // }
        // let z_current = check_results(&zzz, "z".to_string());
        // let z_true = x_val & y_val; // objective

        // println!("{:?}", );
        // println!("{:?}", check_results(&lut, "y".to_string()));
        return "fuck".to_string();
    }
}

fn process(
    key: &String,
    lut: &mut HashMap<String, u8>,
    eqs: &mut HashMap<String, (String, String, String)>,
) -> u8 {
    let eqs_copy = eqs.clone();
    let (left_key, eq, right_key) = eqs_copy.get(key).unwrap();
    let left_value: u8;
    let right_value: u8;
    if left_key.starts_with("x") || left_key.starts_with("y") {
        left_value = *lut.get(left_key).unwrap();
    } else {
        left_value = process(left_key, lut, eqs);
    }
    if right_key.starts_with("x") || right_key.starts_with("y") {
        right_value = *lut.get(right_key).unwrap();
    } else {
        right_value = process(right_key, lut, eqs);
    }
    match eq.as_str() {
        // this boolean logic in rust for u8 in kinda janky
        "AND" => return left_value & right_value,
        "OR" => return left_value | right_value,
        "XOR" => return left_value ^ right_value,
        _ => unreachable!("eq invalid"),
    }
}

fn check_results(lut: &HashMap<String, u8>, delim: String) -> usize {
    let mut acc = 0usize;
    for key in lut.keys() {
        if key.starts_with(&delim) {
            let digit = key[1..3].parse::<usize>().unwrap();
            // println!("{} {}", digit, key);
            acc += (*lut.get(key).unwrap() as usize) << digit
        }
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    // assert_eq!(part(&format!("input/{:02}_train0", day), true), "2024");
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    // assert_eq!(
    //     part(&format!("input/{:02}_train1", day), false),
    //     "z00,z01,z02,z05"
    // );
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: xx Minutes");
}
