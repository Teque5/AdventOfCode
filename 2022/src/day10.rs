#[path = "common.rs"] mod common;

use ndarray::array;
use ndarray::s;
use ndarray::Array2;
use ndarray::ArrayViewMut2;

/// follow path of rope with head and tail
fn part(filename: &str, part2: bool) -> isize {
    // read the nonsense
    let lines = common::read_lines(filename);
    let mut bleeps: Vec<String> = Vec::new();
    let mut values: Vec<isize> = Vec::new();
    let mut len: usize = 0;
    for line in lines.iter() {
        let mut parts = line.trim().split_whitespace();
        bleeps.push(parts.next().unwrap().to_string());
        match parts.next() {
            Some(value) => values.push(value.parse::<isize>().unwrap()),
            None => values.push(0),
        };
        len += 1;
    }
    let mut reg = 1isize; // register
    let mut acc = 0isize; // output accumulator

    let mut ldx = 0;
    let mut cycle = 1;
    let mut wait = 0;
    let mut next = 0isize;
    while ldx != len {
        if wait > 0 {
            // cooldown
            wait -= 1;
        } else {
            reg += next;
            // add to register and setup next
            if bleeps[ldx] == "addx" {
                // addx
                wait = 1;
                next = values[ldx];
            } else {
                // noop
                wait = 0;
                next = 0;
            }
            ldx += 1;
        }
        //println!("{:04} reg={:5} acc={} ({} {})", cycle, reg, acc, bleeps[ldx], values[ldx]);
        if (cycle == 20) || (cycle > 30 && (cycle+20) % 40 == 0) {
            acc += reg * cycle as isize;
            //println!("{} {}", cycle, reg * cycle)
        }
        cycle += 1;
    }
    //println!("{:04} reg={:5} acc={}", cycle, reg, acc);

    return acc
}


pub fn solve() {
    let pdx = 10;

    // validate each solver, then apply to test
    assert_eq!(
        part(&format!("input/{:02}_train", pdx), false),
        common::read_lines_as::<isize>(&format!("input/{:02}_val1", pdx))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", pdx), false));
    /*
    assert_eq!(
        part(&format!("input/{:02}_train2", pdx), true),
        common::read_lines_as::<i32>(&format!("input/{:02}_val2", pdx))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", pdx), true));
    */
}
