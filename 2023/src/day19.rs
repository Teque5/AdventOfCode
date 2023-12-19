#[path = "common.rs"]
mod common;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct MiniRule {
    a: char,
    cmp: char,
    val: usize,
    dest: String,
}

fn part(filename: &str, is_part1: bool) -> usize {
    let mut rules: HashMap<String, Vec<MiniRule>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    // parse info
    let lines = common::read_lines(filename);
    for line in lines {
        match line.chars().next() {
            Some('{') => {
                // parse machine part
                // preprocess line into normal json
                let json_line = line
                    .replace(",", ",\"")
                    .replace("{", "{\"")
                    .replace("=", "\"=")
                    .replace("=", ":");
                // deserialize json into struct
                let some_part: Part = serde_json::from_str(&json_line).unwrap();
                parts.push(some_part);
            }
            Some(_) => {
                // parse rule
                // preprocess line
                let mut minirules: Vec<MiniRule> = Vec::new();
                let clean_line = line.replace("}", "");
                let mut clean_cut = clean_line.split("{");
                let loc: String = clean_cut.next().unwrap().to_string();
                for mini in clean_cut.next().unwrap().split(",") {
                    let mut some_minirule: MiniRule = MiniRule {
                        a: ' ',
                        cmp: '.',
                        val: 0,
                        dest: "xenu".to_string(),
                    };
                    let mut some_val: Vec<char> = Vec::new();
                    let mut some_dest: Vec<char> = Vec::new();
                    let mut is_dest = false;
                    if mini.contains(":") {
                        for (mdx, mchar) in mini.char_indices() {
                            if mdx == 0 {
                                some_minirule.a = mchar;
                            } else if mdx == 1 {
                                some_minirule.cmp = mchar;
                            } else if mchar == ':' {
                                is_dest = true;
                            } else {
                                if is_dest {
                                    some_dest.push(mchar);
                                } else {
                                    some_val.push(mchar);
                                }
                            }
                        }
                        some_minirule.val = some_val
                            .iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                        some_minirule.dest = some_dest.into_iter().collect();
                    } else {
                        // we are at the final else statement
                        some_minirule.dest = mini.to_string();
                    }
                    minirules.push(some_minirule);
                }
                rules.insert(loc, minirules);
            }
            None => {
                // skip blank line
            }
        }
    }
    for (key, value) in &rules {
        // println!("{:?} {:?}", key, value);
    }
    // actually parse all the parts with the rules
    let mut acc = 0usize;
    for some_part in parts {
        // println!("{:?}", some_part);
        let is_in_a = workflow(&rules, &some_part, "in".to_string());
        if is_in_a {
            acc += some_part.x;
            acc += some_part.m;
            acc += some_part.a;
            acc += some_part.s;
        }
    }
    return acc;
}

/// process workflow at some destination
fn workflow(rules: &HashMap<String, Vec<MiniRule>>, some_part: &Part, loc: String) -> bool {
    let some_rules = &rules[&loc];
    for minirule in some_rules {
        let quantity: usize = match minirule.a {
            'x' => some_part.x,
            'm' => some_part.m,
            'a' => some_part.a,
            's' => some_part.s,
            ' ' => 0,
            _ => panic!("not possible"),
        };
        match minirule.cmp {
            '>' => {
                if quantity > minirule.val {
                    if minirule.dest == "A" {
                        return true;
                    } else if minirule.dest == "R" {
                        return false;
                    } else {
                        return workflow(rules, some_part, minirule.dest.clone());
                    }
                }
            }
            '<' => {
                if quantity < minirule.val {
                    if minirule.dest == "A" {
                        return true;
                    } else if minirule.dest == "R" {
                        return false;
                    } else {
                        return workflow(rules, some_part, minirule.dest.clone());
                    }
                }
            }
            '.' => {
                // this is the else condition
                if minirule.dest == "A" {
                    return true;
                } else if minirule.dest == "R" {
                    return false;
                } else {
                    return workflow(rules, some_part, minirule.dest.clone());
                }
            }
            _ => panic!("not possible"),
        }
    }
    return true;
}

pub fn solve() {
    let day: usize = 19;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), false),
    //     common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    // );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: xxx minutes");
}
