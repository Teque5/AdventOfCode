use aoc;
use lru::LruCache;
use std::collections::HashMap;
use std::num::NonZeroUsize;

/// go away Strings nobody likes you
fn convert_towel_str_to_vec(towel_lut: &HashMap<char, u8>, some_str: &str) -> Vec<u8> {
    some_str
        .chars()
        .filter_map(|c| towel_lut.get(&c).copied())
        .collect()
}

struct Checker {
    cache: LruCache<(Vec<u8>, usize), usize>,
    patterns: Vec<Vec<u8>>,
}

impl Checker {
    /// Create a new cache with a specified capacity
    fn new(capacity: usize, patterns: Vec<Vec<u8>>) -> Self {
        Checker {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
            patterns: patterns,
        }
    }

    /// return number of designs possible
    fn check(&mut self, design: Vec<u8>, position: usize) -> usize {
        if let Some(&count) = self.cache.get(&(design.clone(), position)) {
            // cache hit
            return count;
        }
        let mut count = 0usize;
        for pattern in self.patterns.clone() {
            if position + pattern.len() > design.len() {
                // past end
                continue;
            }
            if design[position..position + pattern.len()] == *pattern {
                if position + pattern.len() == design.len() {
                    // this is the end of the design
                    count += 1;
                } else {
                    // continue with rest of design
                    count += self.check(design.clone(), position + pattern.len());
                }
            }
        }
        // save in cache
        self.cache.put((design.clone(), position), count);
        return count;
    }
}

/// Linen Layout
fn part(filename: &str, is_part1: bool) -> usize {
    let mut towel_lut = HashMap::new();
    towel_lut.insert('w', 0u8);
    towel_lut.insert('u', 1u8);
    towel_lut.insert('b', 2u8);
    towel_lut.insert('r', 3u8);
    towel_lut.insert('g', 4u8);

    let mut patterns: Vec<Vec<u8>> = Vec::new();
    let mut designs: Vec<Vec<u8>> = Vec::new();

    // parse
    let lines = aoc::read_lines(filename);
    let raw_patterns = aoc::parse_delimited::<String>(&lines[0], ',');
    for raw_pattern in raw_patterns {
        patterns.push(convert_towel_str_to_vec(&towel_lut, &raw_pattern));
    }
    for ldx in 2..lines.len() {
        designs.push(convert_towel_str_to_vec(&towel_lut, &lines[ldx]))
    }

    // println!("{:?}", patterns);
    // println!("{:?}", designs);

    // Ended up having 13k entries in the cache
    let mut checker = Checker::new(1 << 16, patterns);

    // determine if each design is possible given the patterns
    let mut acc = 0usize;
    for design in designs {
        let num_patterns = checker.check(design, 0usize);
        if is_part1 && num_patterns > 0 {
            acc += 1;
        } else if !is_part1 {
            acc += num_patterns;
        }
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 6);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    assert_eq!(part(&format!("input/{:02}_train", day), false), 16);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 82 Minutes");
}
