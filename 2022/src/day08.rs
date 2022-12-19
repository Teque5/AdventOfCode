#[path = "common.rs"] mod common;

use ndarray::Array2;
use std::fs::File;
use std::io::prelude::*;

/// tree heightmap stuff
fn part(filename: &str, part2: bool) -> i32 {
    //    let raw = common::read_lines(filename);
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    // read file to bytes
    file.read_to_end(&mut buffer).unwrap();
    // numbers are only between ascii 48 and 57
    buffer.retain(|&x| x > 45);
    let dim = (buffer.len() as f64).sqrt() as usize;
    //println!("dbug {}", dim);
    let mut forest = Array2::from_shape_vec((dim, dim), buffer).unwrap();
    // convert chars to numbers
    forest -= 48;
    let mut fval: u8;
    if !part2 {
        let mut visible: Array2<i32> = Array2::zeros((dim, dim));
        let mut height: i32;
        // top
        for cdx in 0..dim {
            height = -1;
            for rdx in 0..dim {
                fval = forest[(rdx, cdx)];
                if fval as i32 > height {
                    visible[(rdx, cdx)] = 1;
                    height = forest[(rdx, cdx)] as i32;
                }
            }
        }
        // right
        for rdx in 0..dim {
            height = -1;
            for cdx in 0..dim {
                fval = forest[(rdx, dim - cdx - 1)];
                if fval as i32 > height {
                    visible[(rdx, dim - cdx - 1)] = 1;
                    height = forest[(rdx, dim - cdx - 1)] as i32;
                }
            }
        }
        // bottom
        for cdx in 0..dim {
            height = -1;
            for rdx in 0..dim {
                fval = forest[(dim - rdx - 1, cdx)];
                if fval as i32 > height {
                    visible[(dim - rdx - 1, cdx)] = 1;
                    height = forest[(dim - rdx - 1, cdx)] as i32;
                }
            }
        }
        // left
        for rdx in 0..dim {
            height = -1;
            for cdx in 0..dim {
                fval = forest[(rdx, cdx)];
                if fval as i32 > height {
                    visible[(rdx, cdx)] = 1;
                    height = forest[(rdx, cdx)] as i32;
                    //println!("{} {}, {}", rdx, cdx, fval);
                }
            }
        }
        return visible.sum();
    } else {
        // part 2 measures viewing distances
        let mut height: u8;
        let mut acc: i32;
        let mut score: i32;
        let mut best_score: i32 = -1;
        for rdx in 0..dim {
            for cdx in 0..dim {
                score = 1;
                //println!("\t({} {})", rdx, cdx);
                height = forest[(rdx, cdx)];
                // look down
                if rdx != dim-1 {
                    acc = 0;
                    for subr in rdx+1..dim {
                        fval = forest[(subr, cdx)];
                        acc += 1;
                        if fval >= height {
                            break;
                        }
                    }
                    score *= acc;
                }
                // look up
                if rdx != 0 {
                    acc = 0;
                    for subr in (0..rdx).rev() {
                        fval = forest[(subr, cdx)];
                        acc += 1;
                        if fval >= height {
                            break;
                        }
                    }
                    score *= acc;
                }
                // look left
                if cdx != 0 {
                    acc = 0;
                    for subc in (0..cdx).rev() {
                        fval = forest[(rdx, subc)];
                        acc += 1;
                        if fval >= height {
                            break;
                        }
                    }
                    score *= acc;
                }
                // look right
                if cdx != dim-1 {
                    acc = 0;
                    for subc in cdx+1..dim {
                        fval = forest[(rdx, subc)];
                        acc += 1;
                        if fval >= height {
                            break;
                        }
                    }
                    score *= acc;
                }
                if score > best_score {
                    best_score = score;
                }
            }
        }
        return best_score;
    }
}

pub fn solve() {
    let pdx = 8;

    // validate each solver, then apply to test
    assert_eq!(
        part(&format!("input/{:02}_train", pdx), false),
        common::read_lines_as::<i32>(&format!("input/{:02}_val1", pdx))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", pdx), false));

    // part 2
    assert_eq!(
        part(&format!("input/{:02}_train", pdx), true),
        common::read_lines_as::<i32>(&format!("input/{:02}_val2", pdx))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", pdx), true));
}
