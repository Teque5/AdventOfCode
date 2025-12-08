use aoc;
use ndarray::Array2;
use std::collections::HashMap;

/// Laboratories, Quantum Manifold
fn part1(filename: &str) -> usize {
    let mut acc = 0usize;
    let (mut manifold, rows, cols) = aoc::read_2d_as::<char>(filename);
    for rdx in 0..rows {
        for cdx in 0..cols {
            match manifold[[rdx, cdx]] {
                'S' | '|' => {
                    if rdx + 1 >= rows {
                        break;
                    }
                    // waterfall down
                    match manifold[[rdx + 1, cdx]] {
                        '.' => {
                            // empty space, continue down
                            manifold[[rdx + 1, cdx]] = '|';
                        }
                        '^' => {
                            let mut is_split = false;
                            // split beam left
                            if manifold[[rdx + 1, cdx - 1]] == '.' {
                                manifold[[rdx + 1, cdx - 1]] = '|';
                                is_split = true;
                            }
                            // split beam right
                            if manifold[[rdx + 1, cdx + 1]] == '.' {
                                manifold[[rdx + 1, cdx + 1]] = '|';
                                is_split = true;
                            }
                            if is_split {
                                acc += 1;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    return acc;
}

/// Laboratories, Quantum Tachyon Manifold
/// very different from part1 - count number of possible paths
fn part2(filename: &str) -> usize {
    let mut paths = 1usize;
    let (manifold, rows, cols) = aoc::read_2d_as::<char>(filename);
    let mut memo = HashMap::<(usize, usize), usize>::new();

    for cdx in 0..cols {
        if manifold[[0, cdx]] == 'S' {
            // begin beam splitting - start with 1 path from the source
            paths += navigate(&manifold, rows, cols, 1, cdx, &mut memo);
        }
    }
    println!("Cache: {} Positions", memo.len());
    return paths;
}

/// Recursive beam function with memoization
fn navigate(
    manifold: &Array2<char>,
    rows: usize,
    cols: usize,
    rdx: usize,
    cdx: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // boundary check
    if rdx + 1 >= rows {
        return 0;
    }
    let current_pos = (rdx, cdx);
    // check cache
    if let Some(&cached_paths) = memo.get(&current_pos) {
        return cached_paths;
    }

    let mut paths_from_here = 0usize;
    match manifold[[rdx + 1, cdx]] {
        '.' => {
            // waterfall down
            paths_from_here = navigate(manifold, rows, cols, rdx + 1, cdx, memo)
        }
        '^' => {
            // split beam left
            if manifold[[rdx + 1, cdx - 1]] == '.' {
                paths_from_here += navigate(manifold, rows, cols, rdx + 1, cdx - 1, memo);
            }
            // split beam right
            if manifold[[rdx + 1, cdx + 1]] == '.' {
                paths_from_here += navigate(manifold, rows, cols, rdx + 1, cdx + 1, memo);
            }
            paths_from_here += 1;
        }
        _ => {}
    };
    // cache result
    memo.insert(current_pos, paths_from_here);
    return paths_from_here;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part1(&format!("input/{:02}_train", day)), 21);
    println!("Part1: {}", part1(&format!("input/{:02}_test", day)));

    assert_eq!(part2(&format!("input/{:02}_train", day)), 40);
    println!("Part2: {}", part2(&format!("input/{:02}_test", day)));

    println!("Coded: 240 minutes.");
}
