#[path = "common.rs"]
mod common;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

const KEY_SEED_TO_SOIL: u8 = 0;
const KEY_SOIL_TO_FERTILIZER: u8 = 1;
const KEY_FERTILIZER_TO_WATER: u8 = 2;
const KEY_WATER_TO_LIGHT: u8 = 3;
const KEY_LIGHT_TO_TEMPERATURE: u8 = 4;
const KEY_TEMPERATURE_TO_HUMIDITY: u8 = 5;
const KEY_HUMIDITY_TO_LOCATION: u8 = 6;

fn try_lookup_cache(lut: &HashMap<u8, Vec<Vec<usize>>>, key: &u8, alpha: &usize) -> usize {
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

/// Food Production Problem
fn part(filename: &str, is_part1: bool) -> usize {
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
            if is_part1 {
                // part 1 this line is seed numbers
                seeds = common::parse_numbers(line);
            } else {
                let seed_ranges = common::parse_numbers(line);
                for chunk in seed_ranges.chunks(2) {
                    let (seed_start, seed_len) = (chunk[0], chunk[1]);
                    seeds.extend(seed_start..(seed_start + seed_len));
                }
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

    // okay now find the path from seed to location
    let count = seeds.len();
    println!("seeds to locate: {}", count);
    // fancy progress bar
    let progress = ProgressBar::new(count as u64);
    progress.set_style(
        ProgressStyle::with_template("{bar:40.cyan/blue} {pos:>10}/{len:10} [{eta} left] {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    // variables we will access inside the parallel loop
    let lowest = Arc::new(Mutex::new(usize::MAX));
    let sdx = Arc::new(AtomicU64::new(0));
    // parallelized iterator w/rayon
    seeds.par_iter().for_each(|seed| {
        let mut next = try_lookup_cache(&lut, &KEY_SEED_TO_SOIL, &seed);
        next = try_lookup_cache(&lut, &KEY_SOIL_TO_FERTILIZER, &next);
        next = try_lookup_cache(&lut, &KEY_FERTILIZER_TO_WATER, &next);
        next = try_lookup_cache(&lut, &KEY_WATER_TO_LIGHT, &next);
        next = try_lookup_cache(&lut, &KEY_LIGHT_TO_TEMPERATURE, &next);
        next = try_lookup_cache(&lut, &KEY_TEMPERATURE_TO_HUMIDITY, &next);
        next = try_lookup_cache(&lut, &KEY_HUMIDITY_TO_LOCATION, &next);
        // is it the lowest?
        if next < *lowest.lock().unwrap() {
            *lowest.lock().unwrap() = next;
            progress.set_message(format!("lowest={}", lowest.lock().unwrap()));
        }
        let current_sdx = sdx.fetch_add(1, Ordering::SeqCst);
        if current_sdx % 100 == 0 {
            progress.set_position(current_sdx);
        }
    });
    progress.finish();
    return lowest.lock().unwrap().clone();
}

pub fn solve() {
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part("input/05_train", true),
        common::read_lines_as::<usize>("input/05_val1")[0]
    );
    println!("Part1: {}", part("input/05_test", true));

    // Test part-2 solver, then apply to real input.
    assert_eq!(
        part("input/05_train", false),
        common::read_lines_as::<usize>("input/05_val2")[0]
    );
    println!("Part2: {}", part("input/05_test", false));
    println!("Coded: 140 minutes");
}
