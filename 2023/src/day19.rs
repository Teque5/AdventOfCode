#[path = "common.rs"]
mod common;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

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
    // for (key, value) in &rules {
    //     println!("{:?} {:?}", key, value);
    // }
    // actually parse all the parts with the rules
    if is_part1 {
        // part 1
        let mut acc = 0usize;
        for some_part in parts {
            // println!("{:?}", some_part);
            let is_in_a = workflow(&rules, &some_part, &"in".to_string());
            if is_in_a {
                acc += some_part.x;
                acc += some_part.m;
                acc += some_part.a;
                acc += some_part.s;
            }
        }
        return acc;
    } else {
        // part 2, check combinations backwards kinda
        // FIXME: I believe the issue is that I was assuming there is a good set of values for all rules, but actually for each A destination there is a separate set of good values
        let mut good = GoodValues {
            x: HashSet::new(),
            m: HashSet::new(),
            a: HashSet::new(),
            s: HashSet::new(),
        };
        work_backward(&mut good, &rules, &"A".to_string());
        println!(
            "{} {} {} {}",
            good.x.len(),
            good.m.len(),
            good.a.len(),
            good.s.len()
        );
        return good.x.len() * good.m.len() * good.a.len() * good.s.len();
    }
    // else {
        // .par_iter().for_each(|line| {
        // let mut acc = 0usize;
        // for px in 1..=4000 {
        //     for pm in 1..=4000 {
        //         for pa in 1..=4000 {
        //             for ps in 1..=4000 {

        //             }
        //         }
        //     }
        // }
    // }
}

struct GoodValues {
    x: HashSet<usize>,
    m: HashSet<usize>,
    a: HashSet<usize>,
    s: HashSet<usize>,
}

fn work_backward(good: &mut GoodValues, rules: &HashMap<String, Vec<MiniRule>>, some_loc: &String) {
    let mut dummy_set = HashSet::new();
    for (_, some_rules) in rules {
        for minirule in some_rules {
            if &minirule.dest == some_loc {
                println!("{:?}", minirule);
                let some_set: &mut HashSet<usize> = match minirule.a {
                    'x' => &mut good.x,
                    'm' => &mut good.m,
                    'a' => &mut good.a,
                    's' => &mut good.s,
                    ' ' => &mut dummy_set,
                    _ => panic!("not possible"),
                };
                match minirule.cmp {
                    '>' => {
                        for quant in 1..=4000 {
                            if quant > minirule.val {
                                some_set.insert(quant);
                            }
                        }
                    }
                    '<' => {
                        for quant in 1..=4000 {
                            if quant < minirule.val {
                                some_set.insert(quant);
                            }
                        }
                    }
                    '.' => {
                        // this is the else condition
                        for quant in 1..=4000 {
                            some_set.insert(quant);
                        }
                    }
                    _ => panic!("not possible"),
                }
            }
        }
    }
}

/// process workflow at some destination
fn workflow(rules: &HashMap<String, Vec<MiniRule>>, some_part: &Part, loc: &String) -> bool {
    for minirule in &rules[loc] {
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
                        return workflow(rules, some_part, &minirule.dest);
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
                        return workflow(rules, some_part, &minirule.dest);
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
                    return workflow(rules, some_part, &minirule.dest);
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
