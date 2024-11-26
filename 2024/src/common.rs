/// Commonly-used library functions for Advent of Code solutions
/// Copyright 2021 Alex Utter, 2022-2024 Teque5
use ndarray::{Array2, Axis};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::str::FromStr;

/// Read a file with one String per line.
#[allow(dead_code)]
pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.filter_map(Result::ok).collect()
}

/// Read a file with one number per line. Return -1 if not parsable.
#[allow(dead_code)]
pub fn read_ints(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    let mut bla = Vec::new();
    for line_result in lines {
        match str::parse::<i32>(&line_result.unwrap()) {
            Ok(okay) => bla.push(okay),
            Err(_) => bla.push(-1i32),
        }
    }
    return bla;
}

/// read a rectangular text block as a 2d array of chars
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

// given a 2d array of chars, print the whole thing as a block
#[allow(dead_code)]
pub fn print_2d_chars(ray: &Array2<char>) {
    for row in ray.axis_iter(Axis(0)) {
        let some_string: String = row.into_iter().collect();
        println!("{:}", some_string);
    }
}

/// Read a file with one number per line.
#[allow(dead_code)]
pub fn read_lines_as<T: FromStr>(filename: &str) -> Vec<T> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<T>().ok())
        .collect()
}

/// Parse character-delimited string as Vec<T>
#[allow(dead_code)]
pub fn split_str_as<T: FromStr>(line: &str, delim: char) -> Vec<T> {
    line.split(delim)
        .filter_map(|x| x.parse::<T>().ok())
        .collect()
}

/// ignore text and just return single or multi-digit numbers
#[allow(dead_code)]
pub fn parse_numbers(line: &str) -> Vec<usize> {
    line.chars()
        .filter(|c| c.is_digit(10) || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

/// same as parse_numbers, but allow negative numbers
#[allow(dead_code)]
pub fn parse_numbers_isize(line: &str) -> Vec<isize> {
    line.chars()
        .filter(|c| c.is_digit(10) || c.is_whitespace() || *c == '-')
        .collect::<String>()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

/// Parse a string's numeric components; ignore all spaces and other chars
#[allow(dead_code)]
pub fn split_numeric(line: &str) -> Vec<u64> {
    let mut temp: Option<u64> = None;
    let mut result: Vec<u64> = Vec::new();
    for c in line.chars() {
        if let Some(n) = c.to_digit(10) {
            temp = Some(n as u64);
        }
        if let Some(n) = temp {
            result.push(n);
        }
    }
    return result;
}
