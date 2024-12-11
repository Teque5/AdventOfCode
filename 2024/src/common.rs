/// Library functions for Advent of Code solutions
/// Copyright 2021 Alex Utter, 2022-2024 Teque5
use ndarray::{Array2, Axis};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::str::FromStr;

/// style string for indicatif::ProgressBar
#[allow(dead_code)]
pub const STYLE: &str = "{bar:40.cyan/blue} {pos:>9}/{len:9} [{eta} left] {msg}";

/// read a file into a vector of strings
#[allow(dead_code)]
pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.filter_map(Result::ok).collect()
}

/// read a file with one number per line
#[allow(dead_code)]
pub fn read_lines_as<T: FromStr>(filename: &str) -> Vec<T> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<T>().ok())
        .collect()
}

/// read a file with rectangular text block as a 2d array of chars
/// 123
/// 456
/// 789
#[allow(dead_code)]
pub fn read_2d_chars(filename: &str) -> (Array2<char>, usize, usize) {
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    // read file to bytes
    file.read_to_end(&mut buffer).unwrap();
    // 10 is the line feed ascii decimal
    let cols = match buffer.iter().position(|&c| c == 10) {
        Some(pos) => pos,
        None => panic!("no line feed found"),
    };
    // printable characters are above 32
    buffer.retain(|&x| x >= 32);
    let charbuffer = buffer.iter().map(|b| *b as char).collect::<Vec<_>>();
    let rows = charbuffer.len() / cols;
    // println!("dbug {} {} {}", rows, cols, charbuffer.len());
    let ray = Array2::from_shape_vec((rows, cols), charbuffer).unwrap();
    return (ray, rows, cols);
}

/// print 2d array of chars
#[allow(dead_code)]
pub fn print_2d_chars(ray: &Array2<char>) {
    for row in ray.axis_iter(Axis(0)) {
        let some_string: String = row.into_iter().collect();
        println!("{:}", some_string);
    }
}

/// parse character-delimited string as vector
/// 1,-3,4,5
#[allow(dead_code)]
pub fn parse_delimited<T: FromStr>(line: &str, delim: char) -> Vec<T> {
    line.split(delim)
        .filter_map(|x| x.parse::<T>().ok())
        .collect()
}

/// parse string, ignore text, and return +/- single or multi-digit numbers
/// 1 -3 4 5
#[allow(dead_code)]
pub fn parse_numbers(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .map(|s| s.trim())
        .filter_map(|s| s.parse::<isize>().ok())
        .collect()
}
