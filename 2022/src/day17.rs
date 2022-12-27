#[path = "common.rs"] mod common;

use ndarray::Array2;
use ndarray::array;
use ndarray::s;
use ndarray::stack;
use ndarray::ArrayViewMut2;
use std::fs::File;
use std::io::prelude::*;



/// tetris time
fn part(filename: &str, part2: bool) -> usize {
    //    let raw = common::read_lines(filename);
    let mut file = File::open(filename).unwrap();
    let mut winds = Vec::new();
    // read file to bytes
    file.read_to_end(&mut winds).unwrap();
    // < is 60; > is 62
    // get rid of control chars
    winds.retain(|&x| x >= 32);
    // on the board we'll make 0 be empty, solid be > 0
    let mut board: Array2<isize> = Array2::ones((1, 7));
    
    drop(&mut board, 0);
    return board.dim().0;
}

// drop a new tetrominio and return new board
fn drop(board: &mut Array2<isize>, tetromino_idx: isize) {
    let mut tetromino: Array2<isize>;
    match tetromino_idx {
        0 => tetromino = array![[0,0,1,1,1,1,0]],
        1 => {
        },
        2 => {},
        3 => {},
        4 => {},
        _ => panic!("invalid"),
    }
}

pub fn solve() {
    let pdx = 17;

    // validate each solver, then apply to test
    assert_eq!(
        part(&format!("input/{:02}_train", pdx), false),
        common::read_lines_as::<usize>(&format!("input/{:02}_val1", pdx))[0]
    );
    //println!("Part1: {}", part(&format!("input/{:02}_test", pdx), false));

    // part 2
    /*
    assert_eq!(
        part(&format!("input/{:02}_train", pdx), true),
        common::read_lines_as::<i32>(&format!("input/{:02}_val2", pdx))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", pdx), true));
    */
}
