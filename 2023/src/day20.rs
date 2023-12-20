#[path = "common.rs"]
mod common;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone)]
struct Module {
    destinations: Vec<String>,
    fmemory: bool,
    cmemory: HashMap<String, bool>,
    // TODO: implement an enum for mode instead of these two bools storing 3 states
    is_broadcast: bool,
    // if false, then is conjunction module
    is_flip_flop: bool,
    hi_count: usize,
    lo_count: usize,
}

impl Module {
    /// Constructor
    fn new(destinations: Vec<String>, is_broadcast: bool, is_flip_flop: bool) -> Self {
        Self {
            destinations: destinations,
            fmemory: false,
            cmemory: HashMap::new(),
            is_broadcast: is_broadcast,
            is_flip_flop: is_flip_flop,
            hi_count: 0usize,
            lo_count: 0usize,
        }
    }

    /// return a vector of (src, dst, pulse) values for a given (src, dst, pulse)
    fn eat(&mut self, src: String, dst: String, pulse: bool) -> Vec<(String, String, bool)> {
        // println!("eat {} {} {} bb={}, ff={}, {:?}", src, dst, pulse, self.is_broadcast, self.is_flip_flop, self.destinations);
        let mut out: Vec<(String, String, bool)> = Vec::new();
        if self.is_broadcast {
            // println!("bb");
            for next in &self.destinations {
                // println!("bb to {}", next);
                out.push((dst.clone(), next.to_string(), false));
                self.lo_count += 1;
            }
        } else if self.is_flip_flop {
            // flip-flop module
            if !pulse {
                // rx low pulse, flip state
                self.fmemory = !self.fmemory;
                for next in &self.destinations {
                    out.push((dst.clone(), next.to_string(), self.fmemory));
                    if self.fmemory {
                        self.hi_count += 1;
                    } else {
                        self.lo_count += 1;
                    }
                }
            }
        } else {
            // conjunction module
            self.cmemory.insert(src, pulse);
            let mut all_hi = true;
            for (_, &cpulse) in self.cmemory.iter() {
                if !cpulse {
                    all_hi = false;
                    break;
                }
            }
            if all_hi {
                // send lo
                for next in &self.destinations {
                    out.push((dst.clone(), next.to_string(), false));
                    self.lo_count += 1;
                }
            } else {
                // send hi
                for next in &self.destinations {
                    out.push((dst.clone(), next.to_string(), true));
                    self.hi_count += 1;
                }
            }
        }
        return out;
    }
}

fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let mut lut: HashMap<String, Module> = HashMap::new();
    let lines = common::read_lines(filename);
    for line in lines {
        if line.contains("broadcaster") {
            // broadcaster module
            let mut left_right = line.split(" -> ");
            let src = left_right.next().unwrap().to_string();
            let tmp = left_right.next().unwrap().replace(",", "");
            let destinations: Vec<String> = tmp.split_whitespace().map(|s| s.to_string()).collect();
            lut.insert(src, Module::new(destinations, true, false));
        } else {
            // flip-flop or conjunction
            let mut left_right = line.split(" -> ");
            let src = left_right.next().unwrap().replace("%", "").replace("&", "");
            let tmp = left_right.next().unwrap().replace(",", "");
            let destinations: Vec<String> = tmp.split_whitespace().map(|s| s.to_string()).collect();
            lut.insert(src, Module::new(destinations, false, line.contains("%")));
        }
    }
    // iterate over the conjunction modules to add input vectors that make sense
    let keys: Vec<String> = lut.keys().cloned().collect();
    for srckey in keys.iter() {
        if !(lut[srckey].is_flip_flop) {
            for subkey in keys.iter() {
                if lut[subkey].destinations.contains(srckey) {
                    // lut[srckey].cmemory.insert(subkey.to_string(), false);
                    if let Some(src_value) = lut.get_mut(srckey) {
                        src_value.cmemory.insert(subkey.to_string(), false);
                    }
                }
            }
        }
    }

    // for (key, value) in &lut {
    //     println!("{} bb={} ff={} {:?} {:?}", key, value.is_broadcast, value.is_flip_flop, value.destinations, value.cmemory);
    // }

    // count pulses
    let mut fifo: VecDeque<(String, String, bool)> = VecDeque::new();

    if is_part1 {
        let mut hi_count = 0usize;
        let mut lo_count = 0usize;
        for _ in 0..1000 {
            lo_count += 1;
            let bla = lut.get_mut("broadcaster").unwrap().eat(
                "null".to_string(),
                "broadcaster".to_string(),
                false,
            );
            fifo.extend(bla);
            while let Some((src, dst, pulse)) = fifo.pop_front() {
                // for each module, eat pulse waiting in fifo if module exists
                // println!("pop {} {} {}", src, dst, pulse);
                if lut.contains_key(&dst) {
                    let bla = lut.get_mut(&dst).unwrap().eat(src, dst, pulse);
                    fifo.extend(bla);
                }
            }
        }
        for (key, value) in &lut {
            // println!("{:>30} {:>4} {:>4}", key, value.lo_count, value.hi_count);
            hi_count += value.hi_count;
            lo_count += value.lo_count;
        }
        return hi_count * lo_count;
    } else {
        // part 2: stop pressing the button when a single low pulse is sent to "rx"
        // gh leads to rx and is a conjunction node, so the several inputs to gh all need to be
        let mut rx_found = false;
        let mut presses = 0usize;
        while !rx_found {
            presses += 1;
            let bla = lut.get_mut("broadcaster").unwrap().eat(
                "null".to_string(),
                "broadcaster".to_string(),
                false,
            );
            fifo.extend(bla);
            while let Some((src, dst, pulse)) = fifo.pop_front() {
                // for each module, eat pulse waiting in fifo if module exists
                if dst == "rx" && pulse == false {
                    println!("found! {}", presses);
                    rx_found = true;
                }
                if lut.contains_key(&dst) {
                    let bla = lut.get_mut(&dst).unwrap().eat(src.clone(), dst.clone(), pulse);
                    fifo.extend(bla);
                }
            }
            if presses % 1_000_000 == 0 {
                println!("{}e6 Presses", (presses as f32) / 1e6);
            }
        //     if presses > 3730 {
        //         println!("finish");
        //     }
        }
        println!("found after {} presses", presses);
        // higher than 3734
        return presses
    }
}

pub fn solve() {
    let day: usize = 20;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train1", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    assert_eq!(
        part(&format!("input/{:02}_train2", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val2", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 165 minutes");
}
