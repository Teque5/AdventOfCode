#[path = "common.rs"]
mod common;
use ndarray::{Array2, Axis, stack, s};

/// Cosmic Expansion
fn part(filename: &str, is_part1: bool) -> usize {
    let mut acc = 0usize;
    let mut newrdx: Vec<usize> = Vec::new();
    let mut newcdx: Vec<usize> = Vec::new();

    // parse info
    let (universe, rows, cols) = common::read_2d_chars(filename);

    let mut empty = true;
    // figure out where expansion occurs
    for rdx in 0..rows {
        empty = true;
        for cdx in 0..cols {
            if universe[(rdx, cdx)] == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            newrdx.push(rdx);
        }
    }
    println!("newrdx{:?}",newrdx);
    for cdx in 0..cols {
        empty = true;
        for rdx in 0..rows {
            if universe[(rdx, cdx)] == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            newcdx.push(cdx);
        }
    }
    // too hard to stack in rust
    println!("newcdx {:?}\n",newcdx);
    // let mut expanded = universe.clone();
    // for rdx in newrdx {
    //     let blank = Array2::from_elem((rows, 1), '.');
    //     expanded = stack![
    //         Axis(1),
    //         &expanded.slice(s![..,..rdx]),
    //         &blank,
    //         &expanded.slice(s![.., rdx..])
    //     ];
    //     // expanded.insert_axis(Axis(1), blank);
    //     println!("{}", rdx)
    // }

    // create universe expanded 2d vector
    // println!("new {:?} from {:?}", (rows + newrdx.len(), cols + newcdx.len()), (rows, cols));
    let mut expanded = Array2::from_elem((rows + newrdx.len(), cols + newcdx.len()), '.');
    let mut galaxy_positions: Vec<(usize, usize)> = Vec::new();
    let mut r_extra = 0;
    for rdx in 0..rows{
        if newrdx.contains(&rdx) {
            r_extra += 1;
        }
        let mut c_extra = 0;
        for cdx in 0..cols{
            if newcdx.contains(&cdx) {
                c_extra += 1;
            }

            if universe[(rdx, cdx)] == '#' {
                let pos = (rdx + r_extra, cdx + c_extra);
                // println!("{:?} {}", pos, universe[(rdx, cdx)]);
                expanded[pos] = universe[(rdx, cdx)];
                galaxy_positions.push(pos);
            }
        }
    }
    let nck_combos = combinations(galaxy_positions);
    let mut zzz = 0;
    for (pos1, pos2) in nck_combos.iter() {
        acc += ((pos1.0 as i32) - (pos2.0 as i32)).abs() as usize;
        acc += ((pos1.1 as i32) - (pos2.1 as i32)).abs() as usize;
        println!("{}", ((pos1.0 as i32) - (pos2.0 as i32)).abs())

    }
    println!("{}", zzz);
    // for rdx in 0..(rows+newrdx.len()) {

    //     let some_string: String = expanded.slice(s![rdx, ..]).into_iter().collect();
    //     println!("{:}", some_string);
    // }


    return acc;
}

fn combinations<T: Clone>(items: Vec<T>) -> Vec<(T, T)> {
    let mut combinations = Vec::new();

    for i in 0..items.len() {
        for j in i + 1..items.len() {
            combinations.push((items[i].clone(), items[j].clone()));
        }
    }

    combinations
}

pub fn solve() {
    let day: usize = 11;
    // Test part-1 solver, then apply to real input.
    assert_eq!(
        part(&format!("input/{:02}_train", day), true),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", day))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // // Test part-2 solver, then apply to real input.
    // assert_eq!(
    //     part(&format!("input/{:02}_train", day), false),
    //     common::read_lines_as::<isize>(&format!("input/{:02}_val2", day))[0]
    // );
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: xxx minutes");
}
