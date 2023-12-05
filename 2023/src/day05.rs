#[path = "common.rs"]
mod common;
use std::collections::HashMap;

const key_seed_to_soil: &str = "seed-to-soil";
const key_soil_to_fertilizer: &str = "soil-to-fertilizer";
const key_fertilizer_to_water: &str = "fertilizer-to-water";
const key_water_to_light: &str = "water-to-light";
const key_light_to_temperature: &str = "light-to-temperature";
const key_temperature_to_humidity: &str = "temperature-to-humidity";
const key_humidity_to_location: &str = "humidity-to-location";

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
        } else if line.contains(key_seed_to_soil) {
            key = key_seed_to_soil;
        } else if line.contains(key_soil_to_fertilizer) {
            key = key_soil_to_fertilizer;
        } else if line.contains(key_fertilizer_to_water) {
            key = key_fertilizer_to_water;
        } else if line.contains(key_water_to_light) {
            key = key_water_to_light;
        } else if line.contains(key_light_to_temperature) {
            key = key_light_to_temperature;
        } else if line.contains(key_temperature_to_humidity) {
            key = key_temperature_to_humidity;
        } else if line.contains(key_humidity_to_location) {
            key = key_humidity_to_location;
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
    let mut next = 0usize;
    for seed in seeds {
        // println!("seed {}", seed);
        next = try_lookup(&lut, key_seed_to_soil, &seed);
        next = try_lookup(&lut, key_soil_to_fertilizer, &next);
        next = try_lookup(&lut, key_fertilizer_to_water, &next);
        next = try_lookup(&lut, key_water_to_light, &next);
        next = try_lookup(&lut, key_light_to_temperature, &next);
        next = try_lookup(&lut, key_temperature_to_humidity, &next);
        next = try_lookup(&lut, key_humidity_to_location, &next);
        // is it the lowest?
        if next < lowest {
            lowest = next;
        }
    }

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
    // println!("Part2: {}", part2("input/05_test"));
    // println!("94 minutes");
}
