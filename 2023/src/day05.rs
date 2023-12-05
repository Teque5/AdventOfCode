#[path = "common.rs"]
mod common;
use lru_cache::LruCache;
use std::collections::HashMap;

const KEY_SEED_TO_SOIL: u8 = 0;
const KEY_SOIL_TO_FERTILIZER: u8 = 1;
const KEY_FERTILIZER_TO_WATER: u8 = 2;
const KEY_WATER_TO_LIGHT: u8 = 3;
const KEY_LIGHT_TO_TEMPERATURE: u8 = 4;
const KEY_TEMPERATURE_TO_HUMIDITY: u8 = 5;
const KEY_HUMIDITY_TO_LOCATION: u8 = 6;

fn try_lookup_cache(
    lut: &HashMap<u8, Vec<Vec<usize>>>,
    key: &u8,
    alpha: &usize,
    cache: &mut LruCache<(u8, usize), usize>,
) -> usize {
    if cache.contains_key(&(*key, *alpha)) {
        if let Some(result) = cache.get_mut(&(*key, *alpha)) {
            // if it's in the cache, we are already done.
            return *result;
        }
    }
    for map in lut[key].iter() {
        // destination range start, source range start, range length
        if (map[1]..(map[1] + map[2])).contains(alpha) {
            // println!("{} match {} ", alpha, alpha-map[1]+map[0]);
            return alpha - map[1] + map[0];
        }
    }
    // backup condition
    return *alpha;
}

fn part1(filename: &str) -> usize {
    let mut lowest = usize::MAX;
    let mut lut: HashMap<u8, Vec<Vec<usize>>> = HashMap::new();
    let mut seeds: Vec<usize> = Vec::new();
    let mut key = 7u8;

    // parse this nightmare and create a map
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        if line.len() == 0 {
            // blank line
            continue;
        } else if line.contains("seeds") {
            // only the first line is like this
            seeds = common::parse_numbers(line);
        } else if line.contains("seed-to-soil") {
            key = KEY_SEED_TO_SOIL;
        } else if line.contains("soil-to-fertilizer") {
            key = KEY_SOIL_TO_FERTILIZER;
        } else if line.contains("fertilizer-to-water") {
            key = KEY_FERTILIZER_TO_WATER;
        } else if line.contains("water-to-light") {
            key = KEY_WATER_TO_LIGHT;
        } else if line.contains("light-to-temperature") {
            key = KEY_LIGHT_TO_TEMPERATURE;
        } else if line.contains("temperature-to-humidity") {
            key = KEY_TEMPERATURE_TO_HUMIDITY;
        } else if line.contains("humidity-to-location") {
            key = KEY_HUMIDITY_TO_LOCATION;
        } else {
            // update hash map
            let triple = common::parse_numbers(line);
            lut.entry(key).or_insert_with(Vec::new).push(triple);
        }
        // println!("{:?}", lut);
    }
    let mut cache = LruCache::new(1 << 12);
    // okay now find the path from seed to location
    for seed in seeds {
        // println!("seed {}", seed);
        let mut next = try_lookup_cache(&lut, &KEY_SEED_TO_SOIL, &seed, &mut cache);
        next = try_lookup_cache(&lut, &KEY_SOIL_TO_FERTILIZER, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_FERTILIZER_TO_WATER, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_WATER_TO_LIGHT, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_LIGHT_TO_TEMPERATURE, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_TEMPERATURE_TO_HUMIDITY, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_HUMIDITY_TO_LOCATION, &next, &mut cache);
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
    let mut lut: HashMap<u8, Vec<Vec<usize>>> = HashMap::new();
    let mut seeds: Vec<usize> = Vec::new();
    let mut key = 7u8;

    // parse this nightmare and create a map
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        if line.len() == 0 {
            // blank line
            continue;
        } else if line.contains("seeds") {
            // only the first line is like this
            let seed_ranges = common::parse_numbers(line);
            for chunk in seed_ranges.chunks(2) {
                let (seed_start, seed_len) = (chunk[0], chunk[1]);
                seeds.extend(seed_start..(seed_start + seed_len));
            }
        } else if line.contains("seed-to-soil") {
            key = KEY_SEED_TO_SOIL;
        } else if line.contains("soil-to-fertilizer") {
            key = KEY_SOIL_TO_FERTILIZER;
        } else if line.contains("fertilizer-to-water") {
            key = KEY_FERTILIZER_TO_WATER;
        } else if line.contains("water-to-light") {
            key = KEY_WATER_TO_LIGHT;
        } else if line.contains("light-to-temperature") {
            key = KEY_LIGHT_TO_TEMPERATURE;
        } else if line.contains("temperature-to-humidity") {
            key = KEY_TEMPERATURE_TO_HUMIDITY;
        } else if line.contains("humidity-to-location") {
            key = KEY_HUMIDITY_TO_LOCATION;
        } else {
            // update hash map
            let triple = common::parse_numbers(line);
            lut.entry(key).or_insert_with(Vec::new).push(triple);
        }
        // println!("{:?}", lut);
    }
    println!("how many seeds? {}", seeds.len());

    // okay now find the path from seed to location
    let mut count = seeds.len();
    // 1M cache takes about 16G ram
    let mut cache = LruCache::new(1 << 20);
    // this loop took 3.059 seconds for 1M seeds
    for seed in seeds {
        // println!("seed {}", seed);
        let mut next = try_lookup_cache(&lut, &KEY_SEED_TO_SOIL, &seed, &mut cache);
        next = try_lookup_cache(&lut, &KEY_SOIL_TO_FERTILIZER, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_FERTILIZER_TO_WATER, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_WATER_TO_LIGHT, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_LIGHT_TO_TEMPERATURE, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_TEMPERATURE_TO_HUMIDITY, &next, &mut cache);
        next = try_lookup_cache(&lut, &KEY_HUMIDITY_TO_LOCATION, &next, &mut cache);
        // is it the lowest?
        if next < lowest {
            lowest = next;
        }
        count -= 1;
        if count % 20000000 == 0 {
            // prints at 1% progress, about every minute
            println!("{} remaining", count);
        }
    }
    // took about 90 minutes as implemented
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
    assert_eq!(
        part2("input/05_train"),
        common::read_lines_as::<usize>("input/05_val2")[0]
    );
    println!("Part2: {}", part2("input/05_test"));
    println!("140 minutes");
}