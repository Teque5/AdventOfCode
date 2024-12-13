use aoc;
use ndarray::s;
use ndarray::ArrayView2;

/// Check if vector matches XMAS or SAMX
fn is_xmas(some_vec: &Vec<char>) -> usize {
    let mut acc = 0usize;
    let xmas = vec!['X', 'M', 'A', 'S'];
    let smax: Vec<char> = xmas.clone().into_iter().rev().collect();
    if *some_vec == xmas {
        acc += 1;
    }
    if *some_vec == smax {
        acc += 1;
    }
    return acc;
}

/// Check if 2D array matches 4 possible rotations
fn is_mas(something: &ArrayView2<char>) -> usize {
    let mut acc = 0usize;
    let locs: Vec<(usize, usize)> = vec![(0, 0), (0, 2), (2, 0), (2, 2)];
    let cross: Vec<Vec<char>> = vec![
        vec!['M', 'M', 'S', 'S'],
        vec!['S', 'S', 'M', 'M'],
        vec!['M', 'S', 'M', 'S'],
        vec!['S', 'M', 'S', 'M'],
    ];
    // middle must be A
    if something[(1, 1)] == 'A' {
        for pdx in 0..4 {
            // all corners must match one of four permutations
            let mut ok = true;
            for ldx in 0..4 {
                if something[locs[ldx]] != cross[pdx][ldx] {
                    ok = false;
                }
            }
            if ok {
                acc += 1
            }
        }
    }
    return acc;
}

/// Word Search
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;

    let (grid, rows, cols) = aoc::read_2d_chars(filename);
    // aoc::print_2d_chars(&grid);

    if is_part1 {
        // word search
        // scan horizontal
        for rdx in 0..rows {
            for cdx in 0..(cols - 3) {
                let slice: Vec<char> = grid
                    .slice(s![rdx, cdx..cdx + 4])
                    .to_owned()
                    .into_iter()
                    .collect();
                acc += is_xmas(&slice);
            }
        }
        // println!("{}", acc);
        // scan vertical
        for cdx in 0..cols {
            for rdx in 0..(rows - 3) {
                let slice: Vec<char> = grid
                    .slice(s![rdx..rdx + 4, cdx])
                    .to_owned()
                    .into_iter()
                    .collect();
                acc += is_xmas(&slice);
            }
        }
        // println!("{}", acc);
        for rdx in 0..rows - 3 {
            // scan diagonal
            for cdx in 0..cols - 3 {
                let mut slice: Vec<char> = Vec::new();
                for sdx in 0..4 {
                    slice.push(grid[(rdx + sdx, cdx + sdx)]);
                }
                acc += is_xmas(&slice);
                // println!("{} {} {:?}", rdx, cdx, slice);
            }
            // scan opposite diagonal
            for cdx in 3..cols {
                let mut slice: Vec<char> = Vec::new();
                for sdx in 0..4 {
                    slice.push(grid[(rdx + sdx, cdx - sdx)]);
                }
                acc += is_xmas(&slice);
                // println!("{} {} {:?}", rdx, cdx, slice);
            }
        }
    } else {
        // MAS in the shape of an X w/rotations
        for rdx in 0..rows - 2 {
            for cdx in 0..cols - 2 {
                let slice = grid.slice(s![rdx..rdx + 3, cdx..cdx + 3]);
                acc += is_mas(&slice);
            }
        }
    }
    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 18);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 9);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 112 Minutes");
}
