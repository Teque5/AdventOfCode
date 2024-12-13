use aoc;
use lru::LruCache;
use std::num::NonZeroUsize;

struct Cache {
    cache: LruCache<(usize, usize), usize>,
    max_depth: usize,
}

impl Cache {
    /// Create a new cache with a specified capacity
    fn new(capacity: usize, max_depth: usize) -> Self {
        Cache {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
            max_depth: max_depth,
        }
    }
    /// given a stone and a depth, return # of stones if processed from there
    fn process_stone(&mut self, stone: usize, depth: usize) -> usize {
        if let Some(&count) = self.cache.get(&(stone, depth)) {
            // cache hit
            return count;
        }
        if depth == self.max_depth {
            // when we are at the max depth, there is only 1 stone
            return 1usize;
        }
        let stone_str = stone.to_string();
        let num_digits = stone_str.len();
        if stone == 0 {
            // stone is zero -> make it 1
            let count = self.process_stone(1, depth + 1);
            self.cache.put((stone, depth), count);
            return count;
        } else if num_digits % 2 == 0 {
            // stone has even digits -> cut it into two stones
            let half = num_digits / 2;
            let divisor = 10_isize.pow(half as u32) as usize;
            let count = self.process_stone(stone / divisor, depth + 1)
                + self.process_stone(stone % divisor, depth + 1);
            self.cache.put((stone, depth), count);
            return count;
        } else {
            // multiply stone by 2024
            let count = self.process_stone(stone * 2024, depth + 1);
            self.cache.put((stone, depth), count);
            return count;
        }
    }
}

/// Plutonian Pebbles (caching version)
/// Un-cached version died after 48 iterations and >32 GB RAM use
fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let line = aoc::read_lines(filename);
    let stones = aoc::parse_numbers(&line[0]);
    let num_blinks = if is_part1 { 25 } else { 75 };
    // process_stones
    let mut acc = 0;
    // allow 1 million cache entries
    let mut cache = Cache::new(1 << 20, num_blinks);
    for stone in stones {
        acc += cache.process_stone(stone as usize, 0);
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 55312);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 101 Minutes");
}
