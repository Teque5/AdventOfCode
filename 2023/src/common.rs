/// Commonly-used library functions for Advent of Code solutions
/// Copyright 2021 Alex Utter, 2022-2023 Teque5
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::io::Read;
use std::str::FromStr;
use ndarray::Array2;

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

/// Read as a 2d vector (if square only for now)
#[allow(dead_code)]
pub fn read_lines_2d(filename: &str) -> (Array2<char>, usize) {
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    // read file to bytes
    println!("yo!");
    file.read_to_end(&mut buffer).unwrap();
    // printable characters are above 32
    buffer.retain(|&x| x > 32);
    let charbuffer = buffer.iter().map(|b| *b as char).collect::<Vec<_>>();
    let dim = (buffer.len() as f64).sqrt() as usize;
    let square = Array2::from_shape_vec((dim, dim), charbuffer).unwrap();
    return (square, dim);
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

/// Parse a string's numeric components.
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

/// Print a labelled list of items.
#[allow(dead_code)]
pub fn print_list<T: std::fmt::Display>(lbl: &str, iter: impl Iterator<Item = T>) {
    print!("{}: [", lbl);
    for (n, x) in iter.enumerate() {
        if n == 0 {
            print!("{}", x);
        } else {
            print!(", {}", x);
        }
    }
    println!("]");
}
