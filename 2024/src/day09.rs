use aoc;

const EMPTY: usize = usize::MAX;

/// Disk Defragmenter
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0;
    // parse info
    let mut disk: Vec<usize> = Vec::new();
    let line = &aoc::read_lines(filename)[0];
    for (cdx, ccc) in line.chars().enumerate() {
        let count = ccc.to_digit(10).unwrap() as usize;
        if cdx % 2 == 0 {
            // even -> is file
            disk.extend(vec![cdx / 2; count]);
        } else {
            // odd -> is space
            disk.extend(vec![EMPTY; count]);
        }
    }
    // the disk is now complete; so we have to start moving items around
    if is_part1 {
        // move parts of tiles
        let mut start = 0usize;
        // iterate backwards over files
        for ddx in (0..disk.len()).rev() {
            // iterate forwards over spaces
            for sdx in start..ddx {
                if disk[sdx] == EMPTY {
                    // this spot is empty -> swap
                    disk[sdx] = disk[ddx];
                    disk[ddx] = EMPTY;
                    break;
                } else {
                    // speed up next iteration
                    start = sdx;
                }
            }
        }
    } else {
        // part 2: move whole files (try only once!)
        let mut end = disk.len() - 1;
        let mut filesize = 0usize;
        let mut emptysize = 0usize;
        // iterate backwards over files
        for ddx in (0..disk.len()).rev() {
            let this_file = disk[ddx];
            if ddx > end {
                // this is part of a file we already found
                continue;
            } else if this_file == EMPTY {
                // this is not a file
                end -= 1;
                continue;
            }
            // println!("find={}, end={}", ddx, end);
            // determine filesize
            for edx in (0..ddx).rev() {
                if this_file != disk[edx] {
                    // changed value
                    filesize = ddx - edx;
                    end -= filesize;
                    break;
                }
            }
            // iterate forwards over spaces to look for empty space
            let mut start = 0usize;
            for sdx in 0..ddx {
                if sdx < start {
                    continue;
                } else if disk[sdx] != EMPTY {
                    start = sdx;
                    continue;
                }
                // find empty space size
                for idx in sdx..ddx {
                    if disk[idx] != EMPTY {
                        emptysize = idx - sdx;
                        start = sdx + emptysize;
                        // println!("start={}", start);
                        break;
                    }
                }
                // move files if possible
                if emptysize >= filesize {
                    // write
                    disk[sdx..(sdx + filesize)].fill(this_file);
                    // erase
                    disk[end + 1..end + 1 + filesize].fill(EMPTY);
                    // we are done with this defragment
                    break;
                }
            }
        }
    }
    // compute checksum
    for ddx in 0..disk.len() {
        if disk[ddx] != EMPTY {
            acc += ddx * disk[ddx]
        }
    }

    return acc;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 1928);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 2858);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: 87 Minutes");
}
