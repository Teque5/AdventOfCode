use aoc;

use indicatif::{ProgressBar, ProgressStyle};
use lru::LruCache;
use ndarray::Array2;
use pathfinding::directed::astar::astar;
use std::num::NonZeroUsize;

static TRACK_PARTS: &[char] = &['.', 'S', 'E'];

/// Race Condition
fn part(filename: &str, is_part1: bool, is_train: bool) -> usize {
    // parse
    let (track, rows, cols) = aoc::read_2d_chars(filename);

    // find start & end
    let mut start = (0usize, 0usize);
    let mut goal = (0usize, 0usize);
    for ((rdx, cdx), &value) in track.indexed_iter() {
        if value == 'E' {
            goal = (rdx, cdx);
        } else if value == 'S' {
            start = (rdx, cdx);
        }
    }

    // compute a best path if possible
    let mut _path: Vec<Position> = Vec::new();
    let mut cost = 0usize;
    let mut cache: LruCache<((usize, usize), (usize, usize)), Option<(Vec<Position>, usize)>> =
        LruCache::new(NonZeroUsize::new(1 << 16).unwrap());

    // solve racetrack normally
    if let Some((this_path, this_cost)) = astar_cached(&track, &mut cache, start, goal) {
        // path found
        _path = this_path;
        cost = this_cost;
    }
    let base_cost = cost as isize;
    let _base_path = _path;

    let threshold = if is_train { 50 } else { 100 };
    let cheat_dist = if is_part1 { 2 } else { 20 };
    let mut cheats = 0usize; // number of working cheats

    // for every two positions in the track
    // is there a way to connect them through the wall that costs less than some threshold
    let progress = ProgressBar::new((rows * cols * rows * cols) as u64);
    progress.set_style(ProgressStyle::with_template(aoc::STYLE).unwrap());
    progress.set_message("Simulating Cheats");
    for rdx_a in 0..rows {
        for cdx_a in 0..cols {
            for rdx_b in 0..rows {
                for cdx_b in 0..cols {
                    let manhattan_dist = rdx_a.abs_diff(rdx_b) + cdx_a.abs_diff(cdx_b);
                    if manhattan_dist > cheat_dist {
                        // distance between points too far to even be possible
                        continue;
                    }
                    if rdx_a == rdx_b && cdx_a == cdx_b {
                        // cannot use same position
                        continue;
                    }
                    let char_a = track[(rdx_a, cdx_a)];
                    let char_b = track[(rdx_b, cdx_b)];
                    if !TRACK_PARTS.contains(&char_a) || !TRACK_PARTS.contains(&char_b) {
                        // both start and end must be valid track
                        continue;
                    }
                    // both positions are good and logically within cheat dist
                    cost = manhattan_dist;
                    let cheat_alpha = (rdx_a, cdx_a);
                    let cheat_omega = (rdx_b, cdx_b);

                    // compute cost to go from start to start of cheat
                    if let Some((this_path, this_cost)) =
                        astar_cached(&track, &mut cache, start, cheat_alpha)
                    {
                        // path found
                        _path = this_path;
                        cost += this_cost;
                    } else {
                        // no path
                        continue;
                    }

                    // compute cost to go from end of cheat to goal
                    if let Some((this_path, this_cost)) =
                        astar_cached(&track, &mut cache, cheat_omega, goal)
                    {
                        // path found
                        _path = this_path;
                        cost += this_cost;
                    } else {
                        // no path
                        continue;
                    }

                    // did this cheat actually save us any cost?
                    let improvement = base_cost - (cost as isize);
                    if improvement >= threshold {
                        // this was a good cheat!
                        cheats += 1;
                    }
                    progress.set_position(
                        (rdx_a * cols * rows * cols + cdx_a * rows * cols + rdx_b * cols + cdx_b)
                            as u64,
                    );
                }
            }
        }
    }
    return cheats;
}

fn astar_cached(
    track: &Array2<char>,
    cache: &mut LruCache<((usize, usize), (usize, usize)), Option<(Vec<Position>, usize)>>,
    start: (usize, usize),
    stop: (usize, usize),
) -> Option<(Vec<Position>, usize)> {
    // cache hit?
    if let Some(Some((path, cost))) = cache.get(&(start, stop)) {
        return Some((path.clone(), *cost));
    }
    // cache miss
    if let Some((path, cost)) = astar(
        &Position { current: start },
        |p| p.successors(&track),
        |p| p.heuristic(stop),
        |p| p.current == stop,
    ) {
        cache.put((start, stop), Some((path.clone(), cost)));
        return Some((path.clone(), cost));
    }
    // no path found
    cache.put((start, stop), None);
    return None;
}

#[rustfmt::skip]
fn is_in_track(rows: usize, cols: usize, position: (isize, isize)) -> bool {
    position.0 >= 0
    && position.1 >= 0
    && position.0 < rows as isize
    && position.1 < cols as isize
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    current: (usize, usize),
}

/// This Position object will be used to
/// 1. keep current position
/// 2. compute possible successors (neighbors) & cost
/// 3. compute hueristic (distance to goal)
impl Position {
    fn heuristic(&self, goal: (usize, usize)) -> usize {
        (self.current.0 as isize - goal.0 as isize).abs() as usize
            + (self.current.1 as isize - goal.1 as isize).abs() as usize
    }

    fn successors(&self, maze: &Array2<char>) -> Vec<(Position, usize)> {
        let mut neighbors = Vec::new();

        for (drow, dcol) in aoc::DIRECTIONS {
            // calculate possible position and the cost to move there
            let new_pos = (
                self.current.0 as isize + drow,
                self.current.1 as isize + dcol,
            );
            if !is_in_track(maze.nrows(), maze.ncols(), new_pos) {
                // skip if outside maze
                continue;
            }
            let new_pos_usize = (new_pos.0 as usize, new_pos.1 as usize);
            if TRACK_PARTS.contains(&maze[new_pos_usize]) {
                // this is a valid position
                let cost = 1usize;
                neighbors.push((
                    Position {
                        current: (new_pos.0 as usize, new_pos.1 as usize),
                    },
                    cost,
                ));
            }
        }
        return neighbors;
    }
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true, true), 1);
    println!(
        "Part1: {}",
        part(&format!("input/{:02}_test", day), true, false)
    );
    assert_eq!(part(&format!("input/{:02}_train", day), false, true), 285);
    println!(
        "Part2: {}",
        part(&format!("input/{:02}_test", day), false, false)
    );
    println!("Coded: 150 Minutes");
}
