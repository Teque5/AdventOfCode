#[path = "common.rs"]
mod common;
use std::collections::HashMap;
use chrono::{Local, Duration};

const KEY_SEED_TO_SOIL: &str = "seed-to-soil";
const KEY_SOIL_TO_FERTILIZER: &str = "soil-to-fertilizer";
const KEY_FERTILIZER_TO_WATER: &str = "fertilizer-to-water";
const KEY_WATER_TO_LIGHT: &str = "water-to-light";
const KEY_LIGHT_TO_TEMPERATURE: &str = "light-to-temperature";
const KEY_TEMPERATURE_TO_HUMIDITY: &str = "temperature-to-humidity";
const KEY_HUMIDITY_TO_LOCATION: &str = "humidity-to-location";

fn try_lookup(lut: &HashMap<String, Vec<Vec<usize>>>, key: &str, alpha: &usize) -> usize{
    for map in lut[key].iter() {
        // destination range start, source range start, range length
        if (map[1]..(map[1]+map[2])).contains(alpha) {
            // println!("{:?} {} ", map, alpha);
            // println!("{} match {} ", alpha, alpha-map[1]+map[0]);
            // println!("ddd {} {} {} {}", alpha, map[0], map[1], map[2]);
            return alpha-map[1]+map[0];
        }
    }
    // backup condition
    return *alpha;
}

fn part1(filename: &str) -> usize {
    let mut lowest = usize::MAX;
    let mut lut: HashMap<String, Vec<Vec<usize>>> = HashMap::new();
    let mut seeds: Vec<usize> = Vec::new();
    let mut key = "";


    // parse this nightmare and create a map
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        if line.len() == 0 {
            // blank line
            // println!("blank");
            continue
        } else if line.contains("seeds") {
            // only the first line is like this
            seeds = common::parse_numbers(line);
            // println!("ONCE");
            // for item in &seeds {
            //     println!("{} a", item);
            // }
        } else if line.contains(KEY_SEED_TO_SOIL) {
            key = KEY_SEED_TO_SOIL;
        } else if line.contains(KEY_SOIL_TO_FERTILIZER) {
            key = KEY_SOIL_TO_FERTILIZER;
        } else if line.contains(KEY_FERTILIZER_TO_WATER) {
            key = KEY_FERTILIZER_TO_WATER;
        } else if line.contains(KEY_WATER_TO_LIGHT) {
            key = KEY_WATER_TO_LIGHT;
        } else if line.contains(KEY_LIGHT_TO_TEMPERATURE) {
            key = KEY_LIGHT_TO_TEMPERATURE;
        } else if line.contains(KEY_TEMPERATURE_TO_HUMIDITY) {
            key = KEY_TEMPERATURE_TO_HUMIDITY;
        } else if line.contains(KEY_HUMIDITY_TO_LOCATION) {
            key = KEY_HUMIDITY_TO_LOCATION;
        } else {
            // update hash map
            let triple = common::parse_numbers(line);
            // println!("update >> {}", key);
            // for tri in &triple {
            //     print!("{} ", tri);
            // }
            // println!("");
            lut.entry(key.to_string()).or_insert_with(Vec::new).push(triple);
            // println!("huh {}", key.to_string());
        }
        // println!("{:?}", lut);
    }

    // okay now find the path from seed to location
    for seed in seeds {
        // println!("seed {}", seed);
        let mut next = try_lookup(&lut, KEY_SEED_TO_SOIL, &seed);
        next = try_lookup(&lut, KEY_SOIL_TO_FERTILIZER, &next);
        next = try_lookup(&lut, KEY_FERTILIZER_TO_WATER, &next);
        next = try_lookup(&lut, KEY_WATER_TO_LIGHT, &next);
        next = try_lookup(&lut, KEY_LIGHT_TO_TEMPERATURE, &next);
        next = try_lookup(&lut, KEY_TEMPERATURE_TO_HUMIDITY, &next);
        next = try_lookup(&lut, KEY_HUMIDITY_TO_LOCATION, &next);
        // is it the lowest?
        if next < lowest {
            lowest = next;
        }
    }

    return lowest;
}

fn part2(filename: &str) -> usize {
    // much the same but I have WAY more seeds
    let mut lowest = usize::MAX;
    let mut lut: HashMap<String, Vec<Vec<usize>>> = HashMap::new();
    let mut seeds: Vec<usize> = Vec::new();
    let mut key = "";


    // parse this nightmare and create a map
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        if line.len() == 0 {
            // blank line
            // println!("blank");
            continue
        } else if line.contains("seeds") {
            // only the first line is like this
            let seed_ranges = common::parse_numbers(line);
            for chunk in seed_ranges.chunks(2) {
                let (seed_start, seed_len) = (chunk[0], chunk[1]);
                seeds.extend(seed_start..(seed_start+seed_len));
            }
            // println!("ONCE");
            // for item in &seeds {
            //     println!("{} a", item);
            // }
        } else if line.contains(KEY_SEED_TO_SOIL) {
            key = KEY_SEED_TO_SOIL;
        } else if line.contains(KEY_SOIL_TO_FERTILIZER) {
            key = KEY_SOIL_TO_FERTILIZER;
        } else if line.contains(KEY_FERTILIZER_TO_WATER) {
            key = KEY_FERTILIZER_TO_WATER;
        } else if line.contains(KEY_WATER_TO_LIGHT) {
            key = KEY_WATER_TO_LIGHT;
        } else if line.contains(KEY_LIGHT_TO_TEMPERATURE) {
            key = KEY_LIGHT_TO_TEMPERATURE;
        } else if line.contains(KEY_TEMPERATURE_TO_HUMIDITY) {
            key = KEY_TEMPERATURE_TO_HUMIDITY;
        } else if line.contains(KEY_HUMIDITY_TO_LOCATION) {
            key = KEY_HUMIDITY_TO_LOCATION;
        } else {
            // update hash map
            let triple = common::parse_numbers(line);
            // println!("update >> {}", key);
            // for tri in &triple {
            //     print!("{} ", tri);
            // }
            // println!("");
            lut.entry(key.to_string()).or_insert_with(Vec::new).push(triple);
            // println!("huh {}", key.to_string());
        }
        // println!("{:?}", lut);
    }
    println!("how many seeds? {}", seeds.len());

    // okay now find the path from seed to location
    let mut count = 10000isize;
    let now = Local::now();
    for seed in seeds {
        // println!("seed {}", seed);
        let mut next = try_lookup(&lut, KEY_SEED_TO_SOIL, &seed);
        next = try_lookup(&lut, KEY_SOIL_TO_FERTILIZER, &next);
        next = try_lookup(&lut, KEY_FERTILIZER_TO_WATER, &next);
        next = try_lookup(&lut, KEY_WATER_TO_LIGHT, &next);
        next = try_lookup(&lut, KEY_LIGHT_TO_TEMPERATURE, &next);
        next = try_lookup(&lut, KEY_TEMPERATURE_TO_HUMIDITY, &next);
        next = try_lookup(&lut, KEY_HUMIDITY_TO_LOCATION, &next);
        // is it the lowest?
        if next < lowest {
            lowest = next;
        }
        count -=1;
        if count == 0 {
            break;
        }
    }
    let end_time = Local::now();
    let time_delta = end_time.signed_duration_since(now);

    // Print the time delta in seconds
    println!("Time delta: {} seconds", time_delta.num_milliseconds());

    return lowest;
}


pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part1("input/05_train"),
        common::read_lines_as::<usize>("input/05_val1")[0]
    );
    println!("Part1: {}", part1("input/05_test"));

    // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part2("input/05_train"),
    //     common::read_lines_as::<usize>("input/05_val2")[0]
    // );
    println!("Part2: {}", part2("input/05_test"));
    // println!("94 minutes");
}
