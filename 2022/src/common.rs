/// Commonly-used library functions for Advent of Code solutions
/// Copyright 2021 by Alex Utter, 2022 Teque5

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::str::FromStr;

/// Read a file with one String per line.
#[allow(dead_code)]
pub fn read_lines(filename: &str) -> Vec<String>
{
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.filter_map(Result::ok).collect()
}

/// Read a file with one number per line.
#[allow(dead_code)]
pub fn read_ints(filename: &str) -> Vec<i32>
{
// KYLES CUSTOM GARBAGE
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    let mut bla = Vec::new();// Vec<i32>::new();
    for line_result in lines {
//        bla.push(line_result.parse::i32.ok());
//        bla.push(str::parse::<i32>(&line_result.unwrap()).ok_or(-1i32));
        //let val: i32 = line_result.parse()
        //match line_result.parse::<i32>() {
        match str::parse::<i32>(&line_result.unwrap()) {
            Ok(okay) => { bla.push(okay)}
            Err(_) => { bla.push(-1i32) }
        }
    }
    return bla
//  lines.filter_map(Result::ok)
//         .filter_map(|line| line.trim().parse::<T>().ok())
//         .filter_map(|line| line.trim().parse::<i32>().ok())
//         .map(|line| line.trim().parse::<T>())
//       .collect()
}

/// Read a file with one number per line.
#[allow(dead_code)]
pub fn read_lines_as<T: FromStr>(filename: &str) -> Vec<T>
{
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.filter_map(Result::ok)
         .filter_map(|line| line.trim().parse::<T>().ok())
//         .map(|line| line.trim().parse::<T>())
         .collect()
}

/// Parse character-delimited string as Vec<T>
#[allow(dead_code)]
pub fn split_str_as<T: FromStr>(line: &str, delim:char) -> Vec<T>
{
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
            temp = Some(10 * temp.unwrap_or(0) + n as u64);
        } else {
            if let Some(n) = temp {result.push(n)};
            temp = None;
        }
    }
    if let Some(n) = temp {result.push(n);}
    return result
}

/// Print a labelled list of items.
#[allow(dead_code)]
pub fn print_list<T: std::fmt::Display>(lbl: &str, iter: impl Iterator<Item=T>) {
    print!("{}: [", lbl);
    for (n,x) in iter.enumerate() {
        if n == 0   {print!("{}", x);}
        else        {print!(", {}", x);}
    }
    println!("]");
}
